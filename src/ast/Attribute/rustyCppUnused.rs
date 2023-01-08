use deriveMacros::CommonAst;

use crate::{
    ast::Attribute::{AtrributeKindInfo, CXXAttribute, CXXAttributeKindInfo},
    utils::stringref::ToStringRef,
};

#[derive(Clone, Copy, CommonAst)]
pub struct AstRustyCppUnused;

impl AstRustyCppUnused {
    pub fn new() -> Self {
        Self {}
    }
}

impl CXXAttributeKindInfo for AstRustyCppUnused {
    fn getAtrributeKindInfo() -> AtrributeKindInfo {
        AtrributeKindInfo {
            name: "unused".to_StringRef(),
            namespace: Some("rustycpp".to_StringRef()),
            requiresParameters: false,
            parser: crate::parse::parser::Parser::parseRustyCppUnused,
        }
    }
}

impl CXXAttribute for AstRustyCppUnused {}
