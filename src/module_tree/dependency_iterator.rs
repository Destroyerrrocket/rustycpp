use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::{Condvar, Mutex};

use priority_queue::PriorityQueue;

use crate::compiler::TranslationUnit;

use super::structs::{ModuleDeclaration, ModuleTree, Node};

struct Data {
    rootsNotReady: Vec<Node>,
    rootsReady: PriorityQueue<Node, usize>,
    rootsSentButNotDone: HashMap<TranslationUnit, Node>,
    childModules: HashMap<ModuleDeclaration, Node>,
    minStageCompleted: usize,
    totalNumModules: usize,
}

/// Iterator over the dependency tree, with priorization based on depth and stage completed.
pub struct DependencyIterator {
    waitForNewRoots: Condvar,
    d: Mutex<Data>,
}

lazy_static! {
    static ref ALL_DEPENDENCY_ITERATORS_WAIT: Condvar = Condvar::new();
    static ref ALL_DEPENDENCY_ITERATORS_MUTEX: Mutex<()> = Mutex::new(());
}

impl DependencyIterator {
    pub fn new(dependencyTree: &ModuleTree, minStageCompleted: usize) -> Self {
        Self {
            waitForNewRoots: Condvar::new(),
            d: Mutex::new(Data {
                rootsNotReady: dependencyTree.roots.values().cloned().collect(),
                rootsReady: PriorityQueue::new(),
                rootsSentButNotDone: HashMap::new(),
                childModules: dependencyTree.childModules.clone(),
                minStageCompleted,
                totalNumModules: dependencyTree.roots.len() + dependencyTree.childModules.len(),
            }),
        }
    }

    fn updateReadies(d: &mut Data) {
        let mut stillNotReady = Vec::new();
        for root in d.rootsNotReady.drain(..) {
            let stepsCompleted = root.stepsCompleted.load(Ordering::Relaxed);
            if stepsCompleted >= d.minStageCompleted {
                let idx = root.dependedBy.len() * d.totalNumModules + root.depth;
                d.rootsReady.push(root, idx);
            } else {
                stillNotReady.push(root);
            }
        }
        d.rootsNotReady = stillNotReady;
    }

    pub fn markDone(&self, translationUnit: &TranslationUnit, newCompletionState: usize) {
        {
            let mut d = self.d.lock().unwrap();
            if let Some(root) = d.rootsSentButNotDone.remove(translationUnit) {
                root.stepsCompleted
                    .fetch_max(newCompletionState, Ordering::Relaxed);
                for child in root.dependedBy {
                    if let Some(childNode) = d.childModules.get_mut(&child.0) {
                        childNode.dependsOn.remove(&root.module);
                        if childNode.dependsOn.is_empty() {
                            let childNode = childNode.clone();
                            d.childModules.remove(&child.0);
                            d.rootsNotReady.push(childNode);
                        }
                    } else {
                        unreachable!("A child module was not found?");
                    }
                }
            } else {
                unreachable!("You marked as done a TU that was not sent!");
            }
        }
        Self::updateReadies(&mut self.d.lock().unwrap());
        self.waitForNewRoots.notify_one();
        ALL_DEPENDENCY_ITERATORS_WAIT.notify_all();
    }

    pub fn wouldLockIfNext(&self) -> bool {
        let d = self.d.lock().unwrap();
        d.rootsNotReady.is_empty() && d.rootsReady.is_empty() && !d.childModules.is_empty()
    }

    pub fn next(&self) -> Option<TranslationUnit> {
        let d = self
            .waitForNewRoots
            .wait_while(self.d.lock().unwrap(), |d| {
                d.rootsNotReady.is_empty()
                    && d.rootsReady.is_empty()
                    && !d.rootsSentButNotDone.is_empty()
                    && !d.childModules.is_empty()
            })
            .unwrap();

        if d.childModules.is_empty() && d.rootsNotReady.is_empty() && d.rootsReady.is_empty() {
            return None;
        }

        if d.rootsNotReady.is_empty()
            && d.rootsReady.is_empty()
            && d.rootsSentButNotDone.is_empty()
            && !d.childModules.is_empty()
        {
            panic!(
                "Internal error: Invalid state: no more roots available and no way to get more!"
            );
        }

        let mut d = ALL_DEPENDENCY_ITERATORS_WAIT
            .wait_while(d, |d| {
                Self::updateReadies(d);
                d.rootsReady.is_empty()
            })
            .unwrap();

        let toSend = d.rootsReady.pop().unwrap().0;
        d.rootsSentButNotDone
            .insert(toSend.module.1.clone(), toSend.clone());
        Some(toSend.module.1.clone())
    }
}
