use crate::Ast::Common::AstAttribute;
use crate::Utils::Structs::{CompileError, CompileMsgImpl};

use super::super::Parser;

impl Parser {
    /**
     * We parsed some attributes at a safe location, but after further parsing we concluded that this was the wrong place for them.
     */
    pub fn actWrongAttributeLocation(&mut self, attr: &[AstAttribute]) {
        for a in attr {
            self.errors.push(CompileError::fromSourceRange(
                "Attribute is not allowed here",
                &a.getSourceRange(),
            ));
        }
    }
}
