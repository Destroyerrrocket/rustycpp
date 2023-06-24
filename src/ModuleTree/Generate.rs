//! Wrapper
use crate::Utils::CompilerState::CompilerState;
use crate::Utils::Structs::CompileMsg;

use super::DependencyAnnotate::annotateTuWithKind;
use super::DependencyDfs::generateModuleTree;
use super::DependencyInterpreter::generateNodes;
use super::Structs::ModuleTree;

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
