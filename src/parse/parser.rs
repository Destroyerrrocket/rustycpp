use crate::ast::Tu::AstTu;
use crate::compiler::TranslationUnit;
use crate::lex::lexer::Lexer;
use crate::utils::compilerstate::CompilerState;
use crate::utils::structs::CompileMsg;

use super::bufferedLexer::{BufferedLexer, StateBufferedLexer};

struct Scope;

pub struct Parser {
    lexer: BufferedLexer,
    lexerStart: StateBufferedLexer,
    filePath: TranslationUnit,
    compilerState: CompilerState,
    errors: Vec<CompileMsg>,
    scope: Scope,
}

impl Parser {
    pub fn new(lexer: Lexer, filePath: TranslationUnit, compilerState: CompilerState) -> Self {
        let (lexer, lexerStart) = BufferedLexer::new(lexer);
        Self {
            lexer,
            lexerStart,
            filePath,
            compilerState,
            errors: vec![],
            scope: Scope {},
        }
    }

    pub fn parse(&mut self) -> (AstTu, Vec<CompileMsg>) {
        (AstTu::new_dont_use(), vec![])
    }

    pub fn printStringTree(&self, _: &AstTu) -> String {
        String::new()
    }
}
