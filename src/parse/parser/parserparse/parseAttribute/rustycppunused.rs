use crate::ast::Attribute::rustyCppUnused::AstRustyCppUnused;
use crate::parse::bufferedLexer::StateBufferedLexer;
use crate::{ast, utils::stringref::StringRef};

use super::super::super::Parser;

impl Parser {
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub fn parseRustyCppUnused(
        &mut self,
        _: StringRef,
        _: Option<StateBufferedLexer>,
    ) -> Option<ast::Attribute::AstCXXAttribute> {
        Some(ast::Attribute::AstCXXAttribute::AstRustyCppUnused(
            AstRustyCppUnused::new(),
        ))
    }
}
