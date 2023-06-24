use crate::Ast::Common::{
    AstAttribute, AstAttributeCXX, AstAttributeCXXRustyCppCheckSymbolMatchTag,
    AstAttributeCXXRustyCppCheckSymbolMatchTagStructNode, AstAttributeCXXRustyCppTagDecl,
};
use crate::Base;
use crate::Parent;
use crate::{
    Ast::NestedNameSpecifier::AstNestedNameSpecifier, Parse::Parser::Parser,
    Utils::Structs::CompileNote,
};
use deriveMacros::{CommonAst, RustycppInheritanceConstructors};

use crate::{
    Ast::Attribute::{AtrributeKindInfo, CXXAttribute, CXXAttributeKindInfo},
    Lex::Token::Token,
    Utils::{
        StringRef::ToStringRef,
        Structs::{CompileError, CompileMsgImpl, FileTokPos},
    },
};

#[derive(Clone, Copy, CommonAst)]
pub struct AstAttributeCXXRustyCppCheckSymbolMatchTagStruct {
    pub numberOrFound: FileTokPos<Token>,
    pub qualifiedNameSpecifier: Option<&'static [AstNestedNameSpecifier]>,
    pub name: FileTokPos<Token>,
}

impl AstAttributeCXXRustyCppCheckSymbolMatchTagStruct {
    pub const fn new_unqualified(
        numberOrFound: FileTokPos<Token>,
        name: FileTokPos<Token>,
    ) -> Self {
        Self {
            numberOrFound,
            qualifiedNameSpecifier: None,
            name,
        }
    }

    pub const fn new_qualified(
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

#[RustycppInheritanceConstructors]
impl AstAttributeCXXRustyCppCheckSymbolMatchTagStructNode {
    pub fn new_unqualified(numberOrFound: FileTokPos<Token>, name: FileTokPos<Token>) -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::new_unqualified(numberOrFound, name),
        }
    }

    pub fn new_qualified(
        numberOrFound: FileTokPos<Token>,
        name: FileTokPos<Token>,
        qualified: &'static [AstNestedNameSpecifier],
    ) -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::new_qualified(numberOrFound, name, qualified),
        }
    }
}

impl CXXAttributeKindInfo for AstAttributeCXXRustyCppCheckSymbolMatchTagStruct {
    fn getAtrributeKindInfo() -> AtrributeKindInfo {
        AtrributeKindInfo {
            name: "checkSymbolMatchTag".to_StringRef(),
            namespace: Some("rustycpp".to_StringRef()),
            requiresParameters: true,
            parser: crate::Parse::Parser::Parser::parseRustyCppCheckSymbolMatchTag,
        }
    }
}

impl CXXAttribute for AstAttributeCXXRustyCppCheckSymbolMatchTagStruct {
    fn actOnAttributeDecl(&self, parser: &mut crate::Parse::Parser::Parser) {
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

        let found = decls[0]
            .getDecl()
            .getAttributes()
            .iter()
            .find_map(|attr: &AstAttribute| {
                let crate::Ast::Attribute::Kind::Cxx(attrmembers) = attr.getKind() else {
                    return None;
                };
                attrmembers.iter().find_map(
                    |attrmember: &AstAttributeCXX| -> Option<AstAttributeCXXRustyCppTagDecl> {
                        AstAttributeCXXRustyCppTagDecl::try_from(attrmember).ok()
                    },
                )
            });
        if let Some(tag) = found {
            let Token::IntegerLiteral(othernumber, _) = tag.getNumber().tokPos.tok else {
                unreachable!();
            };
            if number != othernumber {
                parser.addError(CompileError::fromPreTo(
                    format!("While trying to resolve name {name} we found a unique declaration, but it doesn't have a tag with the number {number}, it has tag number {othernumber}"),
                    &self.numberOrFound,
                ));
                parser.addError(CompileNote::fromSourceRange(
                    "Found decl is this one",
                    &decls[0].getDecl().getSourceRange(),
                ));
            }
        } else {
            parser.addError(CompileError::fromPreTo(
                format!("While trying to resolve name {name} we found a unique declaration, but it doesn't have a tag number."),
                &self.numberOrFound,
            ));
            parser.addError(CompileNote::fromSourceRange(
                "Found decl is this one",
                &decls[0].getDecl().getSourceRange(),
            ));
        }
    }
}

impl CXXAttribute for &AstAttributeCXXRustyCppCheckSymbolMatchTagStructNode {
    fn actOnAttributeDecl(&self, parser: &mut crate::Parse::Parser::Parser) {
        self.base.actOnAttributeDecl(parser);
    }
}
