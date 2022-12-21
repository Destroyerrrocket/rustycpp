//! Create the dependency tree from the translation units

use std::collections::{HashMap, HashSet};
use std::vec;

use crate::utils::structs::{CompileError, CompileMsg};

use super::structs::{ModuleDeclaration, ModuleTree, Node};

/// Check for loops in the dependency graph.
fn dfsLoops(tree: &mut ModuleTree) -> Result<(), Vec<CompileMsg>> {
    if tree.roots.is_empty() && !tree.childModules.is_empty() {
        let mut showALoopTree = tree.clone();
        let candidate = showALoopTree.childModules.keys().next().unwrap().clone();
        let candidate = showALoopTree
            .childModules
            .get_key_value(&candidate)
            .unwrap();
        showALoopTree
            .roots
            .insert(candidate.0.clone(), candidate.1.clone());
        dfsLoops(&mut showALoopTree)?;
        unreachable!();
    }

    let mut err = vec![];
    let mut visited = HashSet::new();
    let mut stack = vec![];
    let mut leftToVisit = vec![];

    for root in tree
        .roots
        .iter()
        .map(|x| (x.1.module.clone(), x.1.dependedBy.clone()))
        .collect::<Vec<_>>()
    {
        stack.push(root.0 .0.clone());
        leftToVisit.push(root.1.clone());
        visited.insert(root.0 .0.clone());

        loop {
            if leftToVisit.is_empty() {
                break;
            }

            if leftToVisit.last().unwrap().is_empty() {
                visited.remove(stack.last().unwrap());
                leftToVisit.pop();
                let currStackDecl = stack.last().unwrap();
                let maxChild = tree
                    .childModules
                    .get(currStackDecl)
                    .unwrap_or_else(|| tree.roots.get(currStackDecl).unwrap())
                    .dependedBy
                    .iter()
                    .map(|x| tree.childModules.get(&x.0).unwrap().depth)
                    .max();
                let currStack = tree
                    .childModules
                    .get_mut(currStackDecl)
                    .unwrap_or_else(|| tree.roots.get_mut(currStackDecl).unwrap());
                currStack.depth = maxChild.unwrap_or(0) + 1;
                stack.pop();
                continue;
            }

            let next = tree
                .childModules
                .get_mut(&leftToVisit.last_mut().unwrap().pop().unwrap().0)
                .unwrap();

            if visited.contains(&next.module.0) {
                err.push(CompileError::on_file(
                    format!(
                        "Loop detected: stack reached: {:?}, which also depends on {}",
                        &stack.iter(),
                        &next.module.0
                    ),
                    next.module.1,
                ));
                continue;
            }

            stack.push(next.module.0.clone());
            leftToVisit.push(next.dependedBy.clone());
            visited.insert(next.module.0.clone());
        }
    }

    Ok(())
}

/// Generates the module tree of the modules, and checks for loops.
pub fn generateModuleTree(
    nodes: HashMap<ModuleDeclaration, Node>,
) -> Result<ModuleTree, Vec<CompileMsg>> {
    let mut tree = ModuleTree {
        roots: HashMap::new(),
        childModules: HashMap::new(),
    };

    for node in nodes {
        if node.1.dependsOn.is_empty() {
            tree.roots.insert(node.0, node.1);
        } else {
            tree.childModules.insert(node.0.clone(), node.1);
        }
    }

    dfsLoops(&mut tree)?;
    Ok(tree)
}
