use crate::ast::common::AstAttributeCXXRustyCppTagDecl;
use deriveMacros::{CommonAst, RustycppInheritanceConstructors};

use crate::{
    ast::{
        common::AstAttributeCXXRustyCppTagDeclStructNode,
        Attribute::{AtrributeKindInfo, CXXAttribute, CXXAttributeKindInfo},
    },
    lex::token::Token,
    utils::{stringref::ToStringRef, structs::FileTokPos},
    Base, Parent,
};

#[derive(Clone, Copy, CommonAst)]
pub struct AstAttributeCXXRustyCppTagDeclStruct {
    pub number: FileTokPos<Token>,
}

impl AstAttributeCXXRustyCppTagDeclStruct {
    pub const fn new(number: FileTokPos<Token>) -> Self {
        Self { number }
    }
}

impl CXXAttributeKindInfo for AstAttributeCXXRustyCppTagDeclStruct {
    fn getAtrributeKindInfo() -> AtrributeKindInfo {
        AtrributeKindInfo {
            name: "tagDecl".to_StringRef(),
            namespace: Some("rustycpp".to_StringRef()),
            requiresParameters: true,
            parser: crate::parse::parser::Parser::parseRustyCppTagDecl,
        }
    }
}

impl CXXAttribute for &AstAttributeCXXRustyCppTagDeclStructNode {}

#[RustycppInheritanceConstructors]
impl AstAttributeCXXRustyCppTagDeclStructNode {
    pub const fn getNumber(&self) -> FileTokPos<Token> {
        self.base.number
    }

    pub fn new(number: FileTokPos<Token>) -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::new(number),
        }
    }
}
