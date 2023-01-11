use crate::{
    ast::{Attribute::AstAttribute, Decl::DeclAst, NestedNameSpecifier::AstNestedNameSpecifier},
    parse::parser::Parser,
    utils::structs::CompileNote,
};
use deriveMacros::CommonAst;

use crate::{
    ast::Attribute::{AstCXXAttribute, AtrributeKindInfo, CXXAttribute, CXXAttributeKindInfo},
    lex::token::Token,
    utils::{
        stringref::ToStringRef,
        structs::{CompileError, CompileMsgImpl, FileTokPos},
    },
};

#[derive(Clone, Copy, CommonAst)]
pub struct AstRustyCppCheckSymbolMatchTag {
    pub numberOrFound: FileTokPos<Token>,
    pub qualifiedNameSpecifier: Option<&'static [AstNestedNameSpecifier]>,
    pub name: FileTokPos<Token>,
}

impl AstRustyCppCheckSymbolMatchTag {
    pub fn new_unqualified(numberOrFound: FileTokPos<Token>, name: FileTokPos<Token>) -> Self {
        Self {
            numberOrFound,
            qualifiedNameSpecifier: None,
            name,
        }
    }

    pub fn new_qualified(
        numberOrFound: FileTokPos<Token>,
        name: FileTokPos<Token>,
        qualified: &'static [AstNestedNameSpecifier],
    ) -> Self {
        Self {
            numberOrFound,
            qualifiedNameSpecifier: Some(qualified),
            name,
        }
    }
}

impl CXXAttributeKindInfo for AstRustyCppCheckSymbolMatchTag {
    fn getAtrributeKindInfo() -> AtrributeKindInfo {
        AtrributeKindInfo {
            name: "checkSymbolMatchTag".to_StringRef(),
            namespace: Some("rustycpp".to_StringRef()),
            requiresParameters: true,
            parser: crate::parse::parser::Parser::parseRustyCppCheckSymbolMatchTag,
        }
    }
}

impl CXXAttribute for AstRustyCppCheckSymbolMatchTag {
    fn actOnAttributeDecl(&self, parser: &mut crate::parse::parser::Parser) {
        let Token::Identifier(name) = self.name.tokPos.tok else {
            unimplemented!()
        };

        let decls = self.qualifiedNameSpecifier.map_or_else(
            || parser.unqualifiedNameLookup(name),
            |qualified| {
                let qualifScope = qualified.last().unwrap().scope.borrow();
                if qualifScope.is_none() {
                    vec![]
                } else {
                    Parser::qualifiedNameLookup(name, qualifScope.as_ref().unwrap())
                }
            },
        );
        if decls.is_empty() {
            if self.numberOrFound.tokPos.tok != Token::BoolLiteral(false) {
                parser.addError(CompileError::fromPreTo(format!("While trying to resolve name {name} we found nothing, but we were expecting something"), &self.numberOrFound));
            }
            return;
        }
        if let Token::BoolLiteral(abool) = self.numberOrFound.tokPos.tok {
            if !abool {
                parser.addError(CompileError::fromPreTo(format!("While trying to resolve name {name} we found something, but we were expecting nothing"), &self.numberOrFound));
            }
            return;
        }

        if decls.len() > 1 {
            parser.addError(CompileError::fromPreTo(
                format!("While trying to resolve name {name} we found more than one declaration, so we can't match it to a tag"),
                &self.numberOrFound,
            ));
            return;
        }

        let Token::IntegerLiteral(number, _) = self.numberOrFound.tokPos.tok else {
            unreachable!();
        };

        let found = decls[0].getDecl().getAttributes().and_then(|attrs| {
            attrs.iter().find_map(|attr: &&AstAttribute| {
                let crate::ast::Attribute::Kind::Cxx(attrmembers) = attr.kind else {
                    return None;
                };
                attrmembers.iter().find_map(|attrmember: &AstCXXAttribute| {
                    let AstCXXAttribute::AstRustyCppTagDecl(tag) = attrmember else {
                            return None;
                        };
                    Some(tag)
                })
            })
        });
        if let Some(tag) = found {
            let Token::IntegerLiteral(othernumber, _) = tag.number.tokPos.tok else {
                unreachable!();
            };
            if number != othernumber {
                parser.addError(CompileError::fromPreTo(
                    format!("While trying to resolve name {name} we found a unique declaration, but it doesn't have a tag with the number {number}, it has tag number {othernumber}"),
                    &self.numberOrFound,
                ));
                parser.addError(CompileNote::fromSourceRange(
                    "Found decl is this one",
                    &decls[0].getDecl().getBaseDecl().sourceRange,
                ));
            }
        } else {
            parser.addError(CompileError::fromPreTo(
                format!("While trying to resolve name {name} we found a unique declaration, but it doesn't have a tag number."),
                &self.numberOrFound,
            ));
            parser.addError(CompileNote::fromSourceRange(
                "Found decl is this one",
                &decls[0].getDecl().getBaseDecl().sourceRange,
            ));
        }
    }
}
