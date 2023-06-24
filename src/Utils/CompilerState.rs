//! State of the compiler
use std::{collections::HashMap, sync::atomic::AtomicBool};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use crate::Compiler::TranslationUnit;

use super::FileMap::FileMap;
use super::Parameters::Parameters;
use super::StateCompileUnit::StateCompileUnit;

/// State of the compiler
#[derive(Debug, Clone)]
pub struct CompilerState {
    /// The parameters of the compilation
    pub parameters: Arc<Parameters>,
    /// The files opened by the compiler
    pub compileFiles: Arc<Mutex<FileMap>>,
    /// The translation units that are being compiled
    pub translationUnitsFiles: Arc<HashSet<u64>>,
    /// The translation units that are being compiled (These are module headers)
    pub moduleHeaderUnitsFiles: Arc<HashSet<u64>>,
    /// State of the compilation units
    pub compileUnits: Arc<HashMap<TranslationUnit, StateCompileUnit>>,
    pub foundErrors: Arc<AtomicBool>,
}
