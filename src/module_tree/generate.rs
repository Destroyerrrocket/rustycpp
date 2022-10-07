use std::sync::{Arc, Mutex};

use crate::compiler::TranslationUnit;
use crate::utils::filemap::FileMap;
use crate::utils::structs::CompileMsg;

use super::dependency_dfs::generateModuleTree;
use super::dependency_interpreter::generateNodes;
use super::dependency_parser::parseModuleMacroOps;
use super::structs::ModuleTree;

pub fn generateDependencyTree(
    mainTranslationUnits: &Vec<TranslationUnit>,
    compileFiles: &mut Arc<Mutex<FileMap>>,
) -> Result<ModuleTree, Vec<CompileMsg>> {
    parseModuleMacroOps(mainTranslationUnits, compileFiles)
        .and_then(|x| generateNodes(x, compileFiles))
        .and_then(|x| generateModuleTree(x, compileFiles))
}
