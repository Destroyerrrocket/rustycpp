use crate::ast;
use crate::ast::Attribute::rustyCppTagDecl::AstRustyCppTagDecl;
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
    ) -> Option<ast::Attribute::AstCXXAttribute> {
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

        Some(ast::Attribute::AstCXXAttribute::AstRustyCppTagDecl(
            AstRustyCppTagDecl::new(*number),
        ))
    }
}
