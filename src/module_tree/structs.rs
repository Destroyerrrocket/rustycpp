use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use crate::compiler::TranslationUnit;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ModuleDeclaration {
    /// Holds module name
    ExportPrimary(String),
    /// Holds module name
    Primary(String),
    /// Holds module name + partition
    ExportPartition(String, String),
    /// Holds module name + partition
    Partition(String, String),
    /// Holds resolved path
    ModuleHeaderUnit(String),
    /// Holds resolved path
    Global(String),
}

impl Display for ModuleDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExportPrimary(module) => format!("export module {}", module).fmt(f),
            Self::Primary(module) => format!("module {}", module).fmt(f),
            Self::ExportPartition(module, part) => {
                format!("export module {}:{}", module, part).fmt(f)
            }
            Self::Partition(module, part) => format!("export module {}:{}", module, part).fmt(f),
            Self::ModuleHeaderUnit(path) => format!("<{}>", path).fmt(f),
            Self::Global(path) => format!("Global module file {}", path).fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModuleOperator {
    Import(String),
    ImportHeader(String),
    ExportModule(String),
    Module(String),
}

#[derive(Debug, Clone, Eq)]
pub struct Node {
    pub module: Arc<(ModuleDeclaration, TranslationUnit)>,
    pub dependedBy: Vec<Arc<(ModuleDeclaration, TranslationUnit)>>,
    pub dependsOn: HashSet<Arc<(ModuleDeclaration, TranslationUnit)>>,
    pub depth: usize,
    pub stepsCompleted: Arc<AtomicUsize>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
    }
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.module.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct ModuleTree {
    /// The root of the ready to compile modules
    pub roots: HashMap<ModuleDeclaration, Node>,
    /// The list of all modules
    pub childModules: HashMap<ModuleDeclaration, Node>,
}
