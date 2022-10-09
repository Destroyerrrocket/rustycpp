//! General structs used by this mod group.
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use crate::compiler::TranslationUnit;

/// Kind of module the TU is of. This also includes ones where the TU does not
/// use modules, like a generated one (import <header>) or a classical .cpp file
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

/// Rellevant module operators. These ony include the rellevant ones for dependency scanning!
#[derive(Debug, Clone)]
pub enum ModuleOperator {
    /// an import <module> directive.
    Import(String),
    /// an import <header> directive.
    ImportHeader(String),
    /// an export module <module> directive.
    ExportModule(String),
    /// a module <module> directive.
    Module(String),
}

/// A node holds all the relevant dependency information of a TU.
#[derive(Debug, Clone, Eq)]
pub struct Node {
    /// The module of the TU, if any
    pub module: Arc<(ModuleDeclaration, TranslationUnit)>,
    /// The TU that depend on this node
    pub dependedBy: Vec<Arc<(ModuleDeclaration, TranslationUnit)>>,
    /// The TU that this node depends on
    pub dependsOn: HashSet<Arc<(ModuleDeclaration, TranslationUnit)>>,
    /// How deep is the node in the tree. The way this is calculated is the
    /// inverse from the roots:
    ///
    /// In this diagram, the letters represent modules, and the arrows go
    /// downwards, indicating that the upper module is depended by the lower one
    ///
    /// a    b    c
    /// \   /     |
    ///   d      /
    ///   ---\  /
    ///       e
    /// In this diagram, a and b have a depth of 2, while c has a depth of 1.
    pub depth: usize,
    /// How many steps of the compilation have been completed? Can be used to
    /// start multiple stages of the compilation at the same time
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

/// Holds the resulting module tree. The `DependencyIterator` uses it to output a TU at a time.
#[derive(Debug, Clone)]
pub struct ModuleTree {
    /// The root of the ready to compile modules
    pub roots: HashMap<ModuleDeclaration, Node>,
    /// The list of all modules
    pub childModules: HashMap<ModuleDeclaration, Node>,
}
