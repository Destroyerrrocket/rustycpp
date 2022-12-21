use crate::ast::Tu::AstTu;
use crate::compiler::TranslationUnit;
use crate::lex::lexer::Lexer;
use crate::utils::compilerstate::CompilerState;
use crate::utils::structs::CompileMsg;

use super::bufferedLexer::BufferedLexer;

struct Scope;

pub struct Parser {
    lexer: BufferedLexer,
    filePath: TranslationUnit,
    compilerState: CompilerState,
    errors: Vec<CompileMsg>,
    scope: Scope,
}

impl Parser {
    pub fn new(lexer: Lexer, filePath: TranslationUnit, compilerState: CompilerState) -> Self {
        Self {
            lexer: BufferedLexer::new(lexer),
            filePath,
            compilerState,
            errors: vec![],
            scope: Scope {},
        }
    }

    pub fn parse(&mut self) -> (AstTu, Vec<CompileMsg>) {
        (AstTu, vec![])
    }

    pub fn printStringTree(&self, _: &AstTu) -> String {
        String::new()
    }
}
