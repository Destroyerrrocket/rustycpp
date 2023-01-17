use std::collections::HashMap;

use crate::{compiler::TranslationUnit, utils::statecompileunit::StateCompileUnit};

use super::structs::{self, Node};

pub fn annotateTuWithKind(
    res: HashMap<structs::ModuleDeclaration, Node>,
    compileUnits: &HashMap<TranslationUnit, StateCompileUnit>,
) -> HashMap<structs::ModuleDeclaration, Node> {
    for node in res.values() {
        *compileUnits
            .get(&node.module.1)
            .unwrap()
            .moduleKind
            .lock()
            .unwrap() = node.clone();
    }
    res
}
