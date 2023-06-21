use crate::ast::common::AstAttributeCXXRustyCppUnused;
use crate::{
    ast::{
        common::AstAttributeCXXRustyCppUnusedStructNode,
        Attribute::{AtrributeKindInfo, CXXAttribute, CXXAttributeKindInfo},
    },
    utils::stringref::ToStringRef,
    Base, Parent,
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
            parser: crate::parse::parser::Parser::parseRustyCppUnused,
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
