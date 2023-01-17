use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    ast::{common::CommonAst, Tu::AstTu},
    compiler::TranslationUnit,
    lex::token::Token,
    sema::scope::Scope,
    utils::{
        compilerstate::CompilerState,
        structs::{CompileMsg, FileTokPos},
        unsafeallocator::UnsafeAllocator,
    },
};

use super::bufferedLexer::{BufferedLexer, StateBufferedLexer};

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
    lexer: BufferedLexer,
    lexerStart: StateBufferedLexer,
    filePath: TranslationUnit,
    compilerState: CompilerState,

    moduleImportState: ModuleImportState,

    rootScope: Rc<RefCell<Scope>>,
    currentScope: Rc<RefCell<Scope>>,

    errors: Vec<CompileMsg>,

    alloc: Rc<UnsafeAllocator>,
}

impl Parser {
    pub fn new(
        tokens: Vec<FileTokPos<Token>>,
        filePath: TranslationUnit,
        compilerState: CompilerState,
    ) -> Self {
        let (lexer, lexerStart) = BufferedLexer::new(tokens);
        let rootScope = Scope::new_root();
        Self {
            lexer,
            lexerStart,
            filePath,
            compilerState,
            moduleImportState: ModuleImportState::StartFile,
            rootScope: rootScope.clone(),
            currentScope: rootScope,
            errors: vec![],

            alloc: Rc::new(UnsafeAllocator::new()),
        }
    }

    pub fn parse(&mut self) -> (AstTu, Vec<CompileMsg>) {
        let tu = self.parseTu();
        (tu, self.errors.clone())
    }

    pub fn printStringTree(ast: &AstTu) -> String {
        ast.getDebugNode().to_string()
    }

    pub fn lexer(&mut self) -> &mut BufferedLexer {
        &mut self.lexer
    }

    pub fn alloc(&self) -> &'static bumpalo::Bump {
        self.alloc.alloc()
    }

    pub fn addError(&mut self, msg: CompileMsg) {
        self.errors.push(msg);
    }
}
