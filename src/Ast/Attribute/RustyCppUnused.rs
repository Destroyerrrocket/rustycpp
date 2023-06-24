use crate::Ast::Common::AstAttributeCXXRustyCppUnused;
use crate::{
    Ast::{
        Attribute::{AtrributeKindInfo, CXXAttribute, CXXAttributeKindInfo},
        Common::AstAttributeCXXRustyCppUnusedStructNode,
    },
    Base, Parent,
    Utils::StringRef::ToStringRef,
};
use deriveMacros::CommonAst;
use deriveMacros::RustycppInheritanceConstructors;

#[derive(Clone, Copy, CommonAst)]
pub struct AstAttributeCXXRustyCppUnusedStruct;

impl AstAttributeCXXRustyCppUnusedStruct {
    pub const fn new() -> Self {
        Self {}
    }
}

impl CXXAttributeKindInfo for AstAttributeCXXRustyCppUnusedStruct {
    fn getAtrributeKindInfo() -> AtrributeKindInfo {
        AtrributeKindInfo {
            name: "unused".to_StringRef(),
            namespace: Some("rustycpp".to_StringRef()),
            requiresParameters: false,
            parser: crate::Parse::Parser::Parser::parseRustyCppUnused,
        }
    }
}

impl CXXAttribute for &AstAttributeCXXRustyCppUnusedStructNode {}

#[RustycppInheritanceConstructors]
impl AstAttributeCXXRustyCppUnusedStructNode {
    pub fn new() -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::new(),
        }
    }
}
