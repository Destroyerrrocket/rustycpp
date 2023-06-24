use crate::Ast::Common::{AstAttributeCXX, AstAttributeCXXRustyCppUnused};
use crate::Utils::Structs::FileTokPos;
use crate::{Lex::Token::Token, Parse::BufferedLexer::StateBufferedLexer};

use super::super::super::Parser;

impl Parser {
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub fn parseRustyCppUnused(
        &mut self,
        _: &FileTokPos<Token>,
        _: Option<StateBufferedLexer>,
    ) -> Option<AstAttributeCXX> {
        Some(AstAttributeCXXRustyCppUnused::new(self.alloc()).into())
    }
}
