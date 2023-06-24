use crate::Ast::Common::{AstAttributeCXX, AstAttributeCXXRustyCppTagDecl};
use crate::Parse::BufferedLexer::StateBufferedLexer;
use crate::{
    Lex::Token::Token,
    Utils::Structs::{CompileError, CompileMsgImpl, FileTokPos},
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
