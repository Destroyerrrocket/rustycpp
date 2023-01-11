use crate::utils::structs::TokPos;
use crate::utils::structs::{CompileNote, FileTokPos};
use crate::{ast, utils::stringref::StringRef};
use crate::{
    ast::Attribute::{AstAttribute, AstCXXAttribute},
    parse::parser::parserparse::ParseMatched,
};
use crate::{
    fileTokPosMatchArm,
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, SourceRange},
};

use super::super::Parser;

mod rustycppchecksymbolmatchtag;
mod rustycpptagdecl;
mod rustycppunused;

impl Parser {
    /**
     * Ignore unused attributes.
     * attribute-specifier-seq:
     *  attribute-specifier-seq [opt] attribute-specifier
     * attribute-specifier:
     *  [ [ attribute-using-prefix [opt] attribute-list ] ]
     *  alignment-specifier
     * alignment-specifier:
     *  alignas ( ignore-balanced )
     */
    pub fn ignoreAttributes(&mut self, lexpos: &mut StateBufferedLexer) {
        while let (_, ParseMatched::Matched) = self.optParseAttributeSpecifier(lexpos, true) {}
    }

    /**
     * Error on wrong attribute location.
     * attribute-specifier-seq:
     *  attribute-specifier-seq [opt] attribute-specifier
     * attribute-specifier:
     *  [ [ attribute-using-prefix [opt] attribute-list ] ]
     *  alignment-specifier
     * alignment-specifier:
     *  alignas ( ignore-balanced )
     */
    pub fn errorAttributes(&mut self, lexpos: &mut StateBufferedLexer) {
        while let (attr, ParseMatched::Matched) = self.optParseAttributeSpecifier(lexpos, false) {
            if let Some(attr) = attr {
                self.actWrongAttributeLocation(&[&attr]);
            }
        }
    }

    /**
     * Return attributes.
     * attribute-specifier-seq:
     *  attribute-specifier-seq [opt] attribute-specifier
     * attribute-specifier:
     *  [ [ attribute-using-prefix [opt] attribute-list ] ]
     *  alignment-specifier
     * alignment-specifier:
     *  alignas ( ignore-balanced )
     */
    pub fn parseAttributes(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> Vec<&'static AstAttribute> {
        let mut attributes = vec![];
        while let (attr, ParseMatched::Matched) = self.optParseAttributeSpecifier(lexpos, false) {
            if let Some(attr) = attr {
                attributes.push(&*self.alloc().alloc(attr));
            }
        }
        attributes
    }

    /**
     * Parses an optional attribute-specifier
     * attribute-specifier:
     *  [ [ attribute-using-prefix [opt] attribute-list ] ]
     *  alignment-specifier
     * alignment-specifier:
     *  alignas ( ignore-balanced )
     */
    fn optParseAttributeSpecifier(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        ignore: bool,
    ) -> (Option<AstAttribute>, ParseMatched) {
        let start = self.lexer().get(lexpos);
        match start {
            Some(fileTokPosMatchArm!(Token::LBracket)) => {
                let startSecondBracket = self.lexer().getWithOffset(lexpos, 1);
                if let Some(fileTokPosMatchArm!(Token::LBracket)) = startSecondBracket {
                    // We know that we have a CXX11 attribute
                    return (
                        self.parseCXX11Attribute(lexpos, ignore),
                        ParseMatched::Matched,
                    );
                } // Ignore otherwise
            }
            Some(fileTokPosMatchArm!(Token::Alignas)) => {
                self.lexer().next(lexpos);
                let Some(startSecondParen) = self.lexer().getIfEq(lexpos, Token::LParen) else {
                    self.errors.push(CompileError::fromPreTo(
                        "This alignas attribute is missing the '( type or constant expression )'.",
                        start.unwrap(),
                    ));
                    return (None, ParseMatched::Matched);
                };
                let Some(_) = self.parseAlmostBalancedPattern(lexpos) else {
                    self.errors.push(CompileError::fromPreTo(
                        "Couldn't find matching ')' for the start of this alignas attribute.",
                        startSecondParen,
                    ));
                    return (None, ParseMatched::Matched);
                };
                let endParen = self.lexer().getWithOffsetSaturating(lexpos, -1); // Saturating not needed in theory, but just in case
                return (
                    Some(AstAttribute::new(
                        ast::Attribute::Kind::AlignAs,
                        SourceRange::newDoubleTok(start.unwrap(), endParen),
                    )),
                    ParseMatched::Matched,
                );
            }
            _ => (),
        }
        (None, ParseMatched::NotMatched)
    }

    /**
     * ASSUMED THAT THE FIRST TWO BRACKETS ARE ALREADY MATCHED, BUT NOT CONSUMED
     * Parses a CXX11 attribute
     * attribute-specifier:
     *  [ [ attribute-using-prefix [opt] attribute-list ] ]
     * attribute-using-prefix:
     *  using attribute-namespace :
     * attribute-list:
     *  attribute [opt]
     *  attribute-list , attribute [opt]
     *  attribute ... [TODO: Not yet suported]
     *  attribute-list , attribute ... [TODO: Not yet suported]
     * attribute:
     *  attribute-token attribute-argument-clause [opt]
     * attribute-token:
     *  identifier
     *  attribute-scoped-token
     * attribute-scoped-token:
     *  attribute-namespace :: identifier
     * attribute-namespace:
     *  identifier
     * attribute-argument-clause:
     *  ( balanced-token-seq [opt] )
     */
    fn parseCXX11Attribute(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        ignore: bool,
    ) -> Option<AstAttribute> {
        #[allow(clippy::debug_assert_with_mut_call)]
        {
            debug_assert!(self.lexer().ifEqOffset(lexpos, Token::LBracket, 0));
            debug_assert!(self.lexer().ifEqOffset(lexpos, Token::LBracket, 1));
        }

        self.lexer().moveForward(lexpos, 2);
        let usingNamespace = {
            if let Some(usingNamespaceRangeStart) =
                self.lexer().getConsumeTokenIfEq(lexpos, Token::Using)
            {
                let Some(namespace) = self.lexer().getConsumeTokenIfIdentifier(lexpos) else {
                    self.errors.push(CompileError::fromPreTo(
                        "This attribute using prefix is missing a namespace.",
                        usingNamespaceRangeStart,
                    ));

                    return None;
                };
                let range = SourceRange::newDoubleTok(usingNamespaceRangeStart, namespace);
                let Token::Identifier(namespaceStr) = namespace.tokPos.tok else {
                    unreachable!();
                };
                if !self.lexer().consumeTokenIfEq(lexpos, Token::Colon) {
                    self.errors.push(CompileError::fromPreTo(
                        "This attribute using prefix is missing a ':'.",
                        namespace,
                    ));
                }
                Some((range, namespaceStr))
            } else {
                None
            }
        };

        let mut attributes = vec![];
        loop {
            if self.lexer().consumeTokenIfEq(lexpos, Token::RBracket) {
                if !self.lexer().consumeTokenIfEq(lexpos, Token::RBracket) {
                    let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
                    self.errors.push(CompileError::fromPreTo(
                        "This attribute is missing a final ']'.",
                        posErr,
                    ));
                }
                break;
            } else if self
                .lexer()
                .getIf(lexpos, |t| matches!(t, Token::Identifier(_)))
                .is_some()
            {
                if let Some(attr) = self.optParseAttributeElement(lexpos, &usingNamespace) {
                    if !ignore {
                        attributes.push(attr);
                    }
                }
            } else if self.lexer().consumeTokenIfEq(lexpos, Token::Comma) {
            } else {
                let posErr = self.lexer().getWithOffsetSaturating(lexpos, 0);
                self.errors.push(CompileError::fromPreTo(
                    "This attribute is missing an identifier.",
                    posErr,
                ));
                break;
            }
        }
        if ignore {
            return None;
        }
        let attributes = self.alloc().alloc_slice_copy(attributes.as_slice());
        Some(AstAttribute::new(
            ast::Attribute::Kind::Cxx(attributes),
            SourceRange::newDoubleTok(
                self.lexer().getWithOffsetSaturating(lexpos, -2),
                self.lexer().getWithOffsetSaturating(lexpos, -1),
            ),
        ))
    }

    /**
     * Parses a CXX11 attribute element, if present.
     *  attribute [opt]
     * attribute:
     *  attribute-token attribute-argument-clause [opt]
     * attribute-token:
     *  identifier
     *  attribute-scoped-token
     * attribute-scoped-token:
     *  attribute-namespace :: identifier
     * attribute-namespace:
     *  identifier
     * attribute-argument-clause:
     *  ( balanced-token-seq [opt] )
     */
    fn optParseAttributeElement(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        usingNamespace: &Option<(SourceRange, StringRef)>,
    ) -> Option<AstCXXAttribute> {
        let Some(nameAttr) = self.lexer().getConsumeTokenIfIdentifier(lexpos) else {
            return None;
        };

        let (namespace, nameAttr) = {
            if let Some(doubleColon) = self.lexer().getConsumeTokenIfEq(lexpos, Token::DoubleColon)
            {
                let namespaceAttr = nameAttr;
                let Some(realName) = self.lexer().getConsumeTokenIfIdentifier(lexpos) else {
                    self.errors.push(CompileError::fromSourceRange(
                        "This attribute prefix is missing an attribute.",
                        &SourceRange::newDoubleTok(nameAttr, doubleColon),
                    ));
                    return None;
                };

                if usingNamespace.is_some() {
                    self.errors.push(CompileError::fromSourceRange(
                    "This attribute is using a namespace, but the attribute is prefixed with a namespace as well. You can only use one or the other.",
                    &SourceRange::newDoubleTok(nameAttr, realName),
                ));
                }
                let Token::Identifier(namespaceAttr) = namespaceAttr.tokPos.tok else {
                    unreachable!();
                };

                (Some(namespaceAttr), realName)
            } else if let Some((_, namespaceName)) = usingNamespace {
                (Some(*namespaceName), nameAttr)
            } else {
                (None, nameAttr)
            }
        };

        let parens = {
            if let Some(lParenStart) = self.lexer().getIfEq(lexpos, Token::LParen) {
                let Some(contents) = self.parseAlmostBalancedPattern(lexpos) else {
                self.errors.push(CompileError::fromPreTo(
                    "Couldn't find matching ')' for the start of this attribute argument clause.",
                    lParenStart,
                ));
                return None;
            };
                Some(contents)
            } else {
                None
            }
        };

        let Token::Identifier(name) = nameAttr.tokPos.tok else {
            unreachable!("{:?}", nameAttr);
        };

        let Some(dispatcher) = ast::Attribute::ATTRIBUTE_DISPATCHER.getAtrributeKindInfo(namespace, name) else {
            self.errors.push(CompileNote::fromPreTo(
                format!("The attribute \"{}\" is not supported.",
                    namespace.map_or_else(|| name.to_string(), |namespace| format!("{namespace}::{name}"))
                ),
                nameAttr,
            ));
            return None;
        };
        if dispatcher.requiresParameters && parens.is_none() {
            self.errors.push(CompileError::fromPreTo(
                "This attribute requires parameters",
                nameAttr,
            ));
            return None;
        }
        (dispatcher.parser)(self, nameAttr, parens)
    }
}
