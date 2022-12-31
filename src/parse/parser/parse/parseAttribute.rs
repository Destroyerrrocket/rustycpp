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
    pub fn optIgnoreAttributes(&mut self, lexpos: &mut StateBufferedLexer) {
        while let (_, ParseMacroMatched::Matched) = self.optParseAttributeSpecifier(lexpos) {}
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
        let start = self.lexer.get(lexpos);
        match start {
            Some(fileTokPosMatchArm!(Token::LBracket)) => {
                let startSecondBracket = self.lexer.getWithOffset(lexpos, 1);
                if let Some(fileTokPosMatchArm!(Token::LBracket)) = startSecondBracket {
                    // We know that we have a CXX11 attribute
                    self.lexer.moveForward(lexpos, 1);
                    let contents = self.parseBalancedPattern(lexpos);
                    if let Some(contents) = contents {
                        let endBracket = self.lexer.get(lexpos);
                        if let Some(fileTokPosMatchArm!(Token::LBracket)) = endBracket {
                            return (
                                Some(AstAttribute::new(
                                    ast::Attribute::Kind::CXX11,
                                    SourceRange::newDoubleTok(
                                        &start.unwrap(),
                                        &endBracket.unwrap(),
                                    ),
                                    contents,
                                )),
                                ParseMacroMatched::Matched,
                            );
                        } else {
                            self.errors.push(CompileError::fromSourceRange(
                                "This attribute is missing a ']'. Instert a ] at the end.",
                                &SourceRange::newDoubleTok(
                                    &start.unwrap(),
                                    &self.lexer.getWithOffsetSaturating(lexpos, -1),
                                ),
                            ));
                            return (None, ParseMacroMatched::Matched);
                        }
                    } else {
                        self.errors.push(CompileError::fromSourceRange(
                            "Couldn't find matching ]] for the start of this attribute.",
                            &SourceRange::newDoubleTok(
                                &start.unwrap(),
                                &startSecondBracket.unwrap(),
                            ),
                        ));
                        return (None, ParseMacroMatched::Matched);
                    }
                } // Ignore otherwise
            }
            Some(fileTokPosMatchArm!(Token::Alignas)) => {
                self.lexer.moveForward(lexpos, 1);
                let startSecondParen = self.lexer.get(lexpos);
                if let Some(fileTokPosMatchArm!(Token::LParen)) = startSecondParen {
                    self.lexer.moveForward(lexpos, 1);
                    let contents = self.parseBalancedPattern(lexpos);
                    if let Some(contents) = contents {
                        let endParen = self.lexer.getWithOffsetSaturating(lexpos, -1); // Saturating not needed in theory, but just in case
                        return (
                            Some(AstAttribute::new(
                                ast::Attribute::Kind::CXX11,
                                SourceRange::newDoubleTok(&start.unwrap(), &endParen),
                                contents,
                            )),
                            ParseMacroMatched::Matched,
                        );
                    } else {
                        self.errors.push(CompileError::fromPreTo(
                            "Couldn't find matching ')' for the start of this alignas attribute.",
                            &startSecondParen.unwrap(),
                        ));
                        return (None, ParseMacroMatched::Matched);
                    }
                } else {
                    self.errors.push(CompileError::fromPreTo(
                        "This alignas attribute is missing the '( type or constant expression )'.",
                        &start.unwrap(),
                    ));
                    return (None, ParseMacroMatched::Matched);
                }
            }
            _ => (),
        }
        (None, ParseMacroMatched::NotMatched)
    }
}
