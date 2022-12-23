use crate::utils::debugnode::DebugNode;

pub trait CommonAst {
    fn getDebugNode(&self) -> DebugNode;
}
