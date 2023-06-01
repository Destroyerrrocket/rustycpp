use crate::ast::Decl::DeclStruct;
use crate::utils::debugnode::DebugNode;

pub struct ASTNodeStruct;
pub struct EmptyStruct;
pub struct AsmStruct;
pub struct NamespaceStruct;
pub struct CustomRustyCppEnumStruct;
pub struct UsingNamespaceStruct;

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

pub trait CommonAst {
    fn getDebugNode(&self) -> DebugNode;
}
