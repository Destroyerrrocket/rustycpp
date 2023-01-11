use crate::{
    ast::{self, Attribute::rustyCppCheckSymbolMatchTag::AstRustyCppCheckSymbolMatchTag},
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, FileTokPos},
};

use super::super::super::Parser;

impl Parser {
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub fn parseRustyCppCheckSymbolMatchTag(
        &mut self,
        name: &FileTokPos<Token>,
        contents: Option<StateBufferedLexer>,
    ) -> Option<ast::Attribute::AstCXXAttribute> {
        let lexpos = &mut contents.unwrap();
        let Some(number) = self
            .lexer()
            .getConsumeTokenIf(lexpos, |t| matches!(t, Token::BoolLiteral(_) | Token::IntegerLiteral(_, _))) else {
                self.errors.push(CompileError::fromPreTo(
                    "This attribute expects an integer/bool literal and an identifier as parameters.",
                    name,
                ));
                return None;
            };

        if !self.lexer().consumeTokenIfEq(lexpos, Token::Comma) {
            self.errors.push(CompileError::fromPreTo(
                "missing comma after first parameter",
                name,
            ));
        }

        let (qualifiedNameSpecifier, matchedQualified) = self.optParseNestedNameSpecifier(lexpos);

        let Some(name) = self
            .lexer()
            .getConsumeTokenIfIdentifier(lexpos) else {
                self.errors.push(CompileError::fromPreTo(
                    "This attribute expects an integer/bool literal and an identifier as parameters.",
                    name,
                ));
                return None;
            };

        Some(
            ast::Attribute::AstCXXAttribute::AstRustyCppCheckSymbolMatchTag(
                if matchedQualified.matched() {
                    AstRustyCppCheckSymbolMatchTag::new_qualified(
                        *number,
                        *name,
                        qualifiedNameSpecifier,
                    )
                } else {
                    AstRustyCppCheckSymbolMatchTag::new_unqualified(*number, *name)
                },
            ),
        )
    }
}
