use std::collections::HashMap;

use crate::{compiler::TranslationUnit, utils::statecompileunit::StateCompileUnit};

use super::structs::{self, Node};

pub fn annotateTuWithKind(
    res: HashMap<structs::ModuleDeclaration, Node>,
    compileUnits: &mut HashMap<TranslationUnit, StateCompileUnit>,
) -> HashMap<structs::ModuleDeclaration, Node> {
    for node in res.values() {
        compileUnits.get_mut(&node.module.1).unwrap().moduleKind = node.clone();
    }
    res
}
