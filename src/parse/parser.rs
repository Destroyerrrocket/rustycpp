use std::cell::UnsafeCell;
use std::rc::Rc;

use crate::ast::{common::CommonAst, Tu::AstTu};
use crate::compiler::TranslationUnit;
use crate::lex::lexer::Lexer;
use crate::utils::compilerstate::CompilerState;
use crate::utils::structs::CompileMsg;
use crate::utils::unsafeallocator::UnsafeAllocator;

use super::bufferedLexer::{BufferedLexer, StateBufferedLexer};

struct Scope;

mod parserparse;
mod parsersema;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ModuleImportState {
    /// Parsing the first decl in a TU.
    StartFile,
    /// after 'module;' but before 'module X;'.
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
    lexer: UnsafeCell<BufferedLexer>,
    lexerStart: StateBufferedLexer,
    filePath: TranslationUnit,
    compilerState: CompilerState,

    moduleImportState: ModuleImportState,
    scope: Scope,

    errors: Vec<CompileMsg>,

    alloc: Rc<UnsafeAllocator>,
}

impl Parser {
    pub fn new(lexer: Lexer, filePath: TranslationUnit, compilerState: CompilerState) -> Self {
        let (lexer, lexerStart) = BufferedLexer::new(lexer);
        Self {
            lexer: UnsafeCell::new(lexer),
            lexerStart,
            filePath,
            compilerState,
            moduleImportState: ModuleImportState::StartFile,
            scope: Scope {},

            errors: vec![],

            alloc: Rc::new(UnsafeAllocator::new()),
        }
    }

    pub fn parse(&mut self) -> (AstTu, Vec<CompileMsg>) {
        let tu = self.parseTu();
        let mut lexErr = unsafe { &mut *self.lexer.get() }.errors();
        lexErr.extend(self.errors.clone());
        return (tu, lexErr);
    }

    pub fn printStringTree(ast: AstTu) -> String {
        ast.getDebugNode().to_string()
    }

    // Super unsafe, we could get invalid references if we ever destroy the parser. Tread carefully.
    pub fn lexer(&self) -> &'static BufferedLexer {
        unsafe { &*self.lexer.get() }
    }

    pub fn alloc(&self) -> &'static bumpalo::Bump {
        self.alloc.alloc()
    }
}
