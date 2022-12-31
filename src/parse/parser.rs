use crate::ast::{common::CommonAst, Tu::AstTu};
use crate::compiler::TranslationUnit;
use crate::lex::lexer::Lexer;
use crate::utils::compilerstate::CompilerState;
use crate::utils::structs::CompileMsg;

use super::bufferedLexer::{BufferedLexer, StateBufferedLexer};

struct Scope;

mod parse;
mod sema;

pub enum ModuleImportState {
    /// Parsing the first decl in a TU.
    StartFile,
    /// after 'module;' but before 'module X;'
    GlobalSection,
    /// after 'module X;' but before any non-import decl.
    ImportSection,
    /// after any non-import decl.
    CodeSection,
    /// after 'module :private;'.
    PrivateSection,
    /// Not a C++20 TU, or an invalid state was found.
    GlobalFile,
}

pub struct Parser {
    lexer: BufferedLexer,
    lexerStart: StateBufferedLexer,
    filePath: TranslationUnit,
    compilerState: CompilerState,

    moduleImportState: ModuleImportState,
    scope: Scope,

    errors: Vec<CompileMsg>,

    alloc: bumpalo::Bump,
}

impl Parser {
    pub fn new(lexer: Lexer, filePath: TranslationUnit, compilerState: CompilerState) -> Self {
        let (lexer, lexerStart) = BufferedLexer::new(lexer);
        Self {
            lexer,
            lexerStart,
            filePath,
            compilerState,
            moduleImportState: ModuleImportState::StartFile,
            scope: Scope {},

            errors: vec![],

            alloc: bumpalo::Bump::new(),
        }
    }

    pub fn parse(&mut self) -> (AstTu, Vec<CompileMsg>) {
        let tu = self.parseTu();
        let mut lexErr = self.lexer.errors();
        lexErr.extend(self.errors.clone());
        return (tu, lexErr);
    }

    pub fn printStringTree(ast: AstTu) -> String {
        ast.getDebugNode().to_string()
    }
}
