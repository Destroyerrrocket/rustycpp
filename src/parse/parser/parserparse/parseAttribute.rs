use crate::ast;
use crate::ast::Attribute::AstAttribute;
use crate::utils::structs::FileTokPos;
use crate::utils::structs::TokPos;
use crate::{
    fileTokPosMatchArm,
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, SourceRange},
};

use super::super::Parser;
use super::parseMiscUtils::ParseMacroMatched;

// TODO: Right now, no attributes are supported. The gramar parsed just uses an ignore-balanced pattern.
impl Parser {
    /**
     * Ignore unused attributes.
     * attribute-specifier-seq:
     *  attribute-specifier-seq [opt] attribute-specifier
     * attribute-specifier:
     *  [ [ ignore-balanced ] ]
     *  alignment-specifier
     * alignment-specifier:
     *  alignas ( ignore-balanced )
     */
    pub fn ignoreAttributes(&mut self, lexpos: &mut StateBufferedLexer) {
        while let (_, ParseMacroMatched::Matched) = self.optParseAttributeSpecifier(lexpos) {}
    }

    /**
     * Return attributes.
     * attribute-specifier-seq:
     *  attribute-specifier-seq [opt] attribute-specifier
     * attribute-specifier:
     *  [ [ ignore-balanced ] ]
     *  alignment-specifier
     * alignment-specifier:
     *  alignas ( ignore-balanced )
     */
    pub fn parseAttributes(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> Vec<&'static AstAttribute> {
        let mut attributes = vec![];
        while let (attr, ParseMacroMatched::Matched) = self.optParseAttributeSpecifier(lexpos) {
            if let Some(attr) = attr {
                attributes.push(&*self.alloc().alloc(attr));
            }
        }
        return attributes;
    }

    /**
     * Parses an optional attribute-specifier
     * attribute-specifier:
     * [ [ ignore-balanced ] ]
     * alignment-specifier
     * alignment-specifier:
     *  alignas ( ignore-balanced )
     */
    fn optParseAttributeSpecifier(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> (Option<AstAttribute>, ParseMacroMatched) {
        let start = self.lexer().get(lexpos);
        match start {
            Some(fileTokPosMatchArm!(Token::LBracket)) => {
                let startSecondBracket = self.lexer().getWithOffset(lexpos, 1);
                if let Some(fileTokPosMatchArm!(Token::LBracket)) = startSecondBracket {
                    // We know that we have a CXX11 attribute
                    self.lexer().next(lexpos);
                    if let Some(contents) = self.parseBalancedPattern(lexpos) {
                        if let Some(endBracket) =
                            self.lexer().getConsumeTokenIfEq(lexpos, Token::RBracket)
                        {
                            return (
                                Some(AstAttribute::new(
                                    ast::Attribute::Kind::CXX11,
                                    SourceRange::newDoubleTok(start.unwrap(), endBracket),
                                    contents,
                                )),
                                ParseMacroMatched::Matched,
                            );
                        } else {
                            self.errors.push(CompileError::fromSourceRange(
                                "This attribute is missing a ']'. Instert a ] at the end.",
                                &SourceRange::newDoubleTok(
                                    start.unwrap(),
                                    self.lexer().getWithOffsetSaturating(lexpos, -1),
                                ),
                            ));
                            return (None, ParseMacroMatched::Matched);
                        }
                    } else {
                        self.errors.push(CompileError::fromSourceRange(
                            "Couldn't find matching ]] for the start of this attribute.",
                            &SourceRange::newDoubleTok(start.unwrap(), startSecondBracket.unwrap()),
                        ));
                        return (None, ParseMacroMatched::Matched);
                    }
                } // Ignore otherwise
            }
            Some(fileTokPosMatchArm!(Token::Alignas)) => {
                self.lexer().next(lexpos);
                if let Some(startSecondParen) = self.lexer().getIfEq(lexpos, Token::LParen) {
                    if let Some(contents) = self.parseBalancedPattern(lexpos) {
                        let endParen = self.lexer().getWithOffsetSaturating(lexpos, -1); // Saturating not needed in theory, but just in case
                        return (
                            Some(AstAttribute::new(
                                ast::Attribute::Kind::CXX11,
                                SourceRange::newDoubleTok(start.unwrap(), endParen),
                                contents,
                            )),
                            ParseMacroMatched::Matched,
                        );
                    } else {
                        self.errors.push(CompileError::fromPreTo(
                            "Couldn't find matching ')' for the start of this alignas attribute.",
                            startSecondParen,
                        ));
                        return (None, ParseMacroMatched::Matched);
                    }
                } else {
                    self.errors.push(CompileError::fromPreTo(
                        "This alignas attribute is missing the '( type or constant expression )'.",
                        start.unwrap(),
                    ));
                    return (None, ParseMacroMatched::Matched);
                }
            }
            _ => (),
        }
        (None, ParseMacroMatched::NotMatched)
    }
}
