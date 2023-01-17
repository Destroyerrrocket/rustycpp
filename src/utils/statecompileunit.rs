//! `StateCompileUnit` used across the compilation in order to allow interaction
//! between the different stages and compilation units

use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Mutex},
};

use atomic_enum::atomic_enum;

use crate::{
    grammars::defineast::DefineAst,
    lex::token::Token,
    module_tree::{self, structs::ModuleOperator},
    utils::structs::{CompileMsg, FileTokPos},
};

#[atomic_enum]
#[derive(PartialEq, Eq)]
pub enum StageCompileUnit {
    Start,
    Lexer,

    Parser,
    // Mostly a placeholder, we'll probably need to do more...
    Compiler,
}

/// Used across the compilation in order to allow interaction between the
/// different stages and compilation units
#[derive(Debug)]
pub struct StateCompileUnit {
    /// Macro definitions that are enabled at the end of the file
    pub macroDefintionsAtTheEndOfTheFile: Mutex<HashMap<String, DefineAst>>,
    /// The module kind of the compilation unit
    pub moduleKind: Mutex<module_tree::structs::Node>,
    /// Stage done of the translation unit
    pub finishedStage: AtomicStageCompileUnit,
    /// Stage being done to the translation unit
    pub processingStage: AtomicStageCompileUnit,
    /// Errors that happened during the compilation
    pub errors: Mutex<Vec<CompileMsg>>,
    /// tokens (available only after lexing. Parsing will consume t)
    pub tokens: Mutex<Option<Vec<FileTokPos<Token>>>>,

    /// The token indexes of the module directives (import/module)
    pub moduleOperationPositions: Mutex<Vec<usize>>,
    /// Module operations
    pub moduleOperations: Mutex<Option<Vec<ModuleOperator>>>,
    /// Blocked by an import header. This can happen when we're lexing a module header, and we are unable to continue due to another import.
    pub blockedByImportHeader: AtomicU64,
}

impl StateCompileUnit {
    /// Creates a new `StateCompileUnit`
    pub fn new() -> Self {
        Self {
            macroDefintionsAtTheEndOfTheFile: Mutex::new(HashMap::new()),
            moduleKind: Mutex::new(module_tree::structs::Node::new_fake()),
            finishedStage: AtomicStageCompileUnit::new(StageCompileUnit::Start),
            processingStage: AtomicStageCompileUnit::new(StageCompileUnit::Start),
            errors: Mutex::new(Vec::new()),
            tokens: Mutex::new(None),
            moduleOperationPositions: Mutex::new(Vec::new()),
            moduleOperations: Mutex::new(None),
            blockedByImportHeader: AtomicU64::new(0),
        }
    }
}
