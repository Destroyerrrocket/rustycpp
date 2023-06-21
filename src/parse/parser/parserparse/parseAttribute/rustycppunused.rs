use crate::ast::common::{AstAttributeCXX, AstAttributeCXXRustyCppUnused};
use crate::utils::structs::FileTokPos;
use crate::{lex::token::Token, parse::bufferedLexer::StateBufferedLexer};

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
