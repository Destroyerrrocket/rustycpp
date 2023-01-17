//! Wrapper
use crate::utils::compilerstate::CompilerState;
use crate::utils::structs::CompileMsg;

use super::dependency_annotate::annotateTuWithKind;
use super::dependency_dfs::generateModuleTree;
use super::dependency_interpreter::generateNodes;
use super::structs::ModuleTree;

/// Wrapper over all the functionality of the module tree generation.
pub fn generateDependencyTree(
    compilerState: &CompilerState,
) -> Result<ModuleTree, Vec<CompileMsg>> {
    let mut it = compilerState.compileUnits.iter().map(|(tu, state)| {
        let operations = state.moduleOperations.lock().unwrap().take().unwrap();
        let isModuleHeaderFile = compilerState.moduleHeaderUnitsFiles.contains(tu);
        (*tu, operations, isModuleHeaderFile)
    });

    generateNodes(&mut it)
        .map(|x| annotateTuWithKind(x, &compilerState.compileUnits))
        .and_then(generateModuleTree)
}
