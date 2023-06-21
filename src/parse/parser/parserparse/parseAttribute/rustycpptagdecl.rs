use crate::ast::common::*;
use crate::parse::bufferedLexer::StateBufferedLexer;
use crate::{
    lex::token::Token,
    utils::structs::{CompileError, CompileMsgImpl, FileTokPos},
};

use super::super::super::Parser;

impl Parser {
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub fn parseRustyCppTagDecl(
        &mut self,
        name: &FileTokPos<Token>,
        contents: Option<StateBufferedLexer>,
    ) -> Option<AstAttributeCXX> {
        let lexpos = &mut contents.unwrap();
        let Some(number) = self
            .lexer()
            .getConsumeTokenIf(lexpos, |t| matches!(t, Token::IntegerLiteral(_, _))) else {
                self.errors.push(CompileError::fromPreTo(
                    "This attribute expects an integer literal as a parameter.",
                    name,
                ));
                return None;
            };

        Some(AstAttributeCXXRustyCppTagDecl::new(self.alloc(), *number).into())
    }
}
