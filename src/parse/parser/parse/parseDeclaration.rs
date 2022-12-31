use crate::{
    ast::{Attribute::AstAttribute, Tu::AstTu},
    fileTokPosMatchArm,
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, FileTokPos, SourceRange, TokPos},
};

use super::super::Parser;

impl Parser {
    pub fn parseDeclaration(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        attr: Vec<AstAttribute>,
    ) -> Vec<()> {
        self.lexer.consumeToken(lexpos);
        vec![]
    }
}
