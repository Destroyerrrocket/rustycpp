use std::collections::HashMap;

use crate::{Compiler::TranslationUnit, Utils::StateCompileUnit::StateCompileUnit};

use super::Structs::{self, Node};

pub fn annotateTuWithKind(
    res: HashMap<Structs::ModuleDeclaration, Node>,
    compileUnits: &HashMap<TranslationUnit, StateCompileUnit>,
) -> HashMap<Structs::ModuleDeclaration, Node> {
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
