//! State of the compiler
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::compiler::TranslationUnit;

use super::filemap::FileMap;
use super::parameters::Parameters;
use super::statecompileunit::StateCompileUnit;

/// State of the compiler
#[derive(Debug, Clone)]
pub struct CompilerState {
    /// The parameters of the compilation
    pub parameters: Arc<Parameters>,
    /// The files opened by the compiler
    pub compileFiles: Arc<Mutex<FileMap>>,
    /// State of the compilation units
    pub compileUnits: Arc<Mutex<HashMap<TranslationUnit, StateCompileUnit>>>,
}
