use deriveMacros::CommonAst;

use crate::{
    ast::Attribute::{AtrributeKindInfo, CXXAttribute, CXXAttributeKindInfo},
    lex::token::Token,
    utils::{stringref::ToStringRef, structs::FileTokPos},
};

#[derive(Clone, Copy, CommonAst)]
pub struct AstRustyCppTagDecl {
    pub number: FileTokPos<Token>,
}

impl AstRustyCppTagDecl {
    pub const fn new(number: FileTokPos<Token>) -> Self {
        Self { number }
    }
}

impl CXXAttributeKindInfo for AstRustyCppTagDecl {
    fn getAtrributeKindInfo() -> AtrributeKindInfo {
        AtrributeKindInfo {
            name: "tagDecl".to_StringRef(),
            namespace: Some("rustycpp".to_StringRef()),
            requiresParameters: true,
            parser: crate::parse::parser::Parser::parseRustyCppTagDecl,
        }
    }
}

impl CXXAttribute for AstRustyCppTagDecl {}
