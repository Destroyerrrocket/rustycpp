//! `StateCompileUnit` used across the compilation in order to allow interaction
//! between the different stages and compilation units

use std::collections::HashMap;

use crate::{grammars::defineast::DefineAst, module_tree};

/// Used across the compilation in order to allow interaction between the
/// different stages and compilation units
#[derive(Debug)]
pub struct StateCompileUnit {
    /// Macro definitions that are enabled at the end of the file
    pub macroDefintionsAtTheEndOfTheFile: HashMap<String, DefineAst>,

    pub moduleKind: module_tree::structs::Node,
}

impl StateCompileUnit {
    /// Creates a new `StateCompileUnit`
    pub fn new() -> Self {
        Self {
            macroDefintionsAtTheEndOfTheFile: HashMap::new(),
            moduleKind: module_tree::structs::Node::new_fake(),
        }
    }
}
