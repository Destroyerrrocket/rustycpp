use std::rc::Rc;
use strum::IntoEnumIterator;

use crate::{
    ast::{
        common::{AstTu, CommonAst},
        Type::{Builtin::BuiltinTypeKind, TypeDict},
    },
    compiler::TranslationUnit,
    lex::token::Token,
    sema::scope::{Scope, ScopeRef},
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

    rootScope: ScopeRef,
    currentScope: ScopeRef,

    errors: Vec<CompileMsg>,

    alloc: Rc<UnsafeAllocator>,
    typeDict: TypeDict,
}

impl Parser {
    pub fn new(
        tokens: Vec<FileTokPos<Token>>,
        filePath: TranslationUnit,
        compilerState: CompilerState,
    ) -> Self {
        let (lexer, lexerStart) = BufferedLexer::new(tokens);
        let rootScope = Scope::new_root();
        let alloc = Rc::new(UnsafeAllocator::new());
        Self {
            lexer,
            lexerStart,
            filePath,
            compilerState,
            moduleImportState: ModuleImportState::StartFile,
            rootScope: rootScope.clone(),
            currentScope: rootScope,
            errors: vec![],

            typeDict: TypeDict::new(alloc.clone()),
            alloc,
        }
    }

    fn initBuiltinTypes(&mut self) {
        for ty in BuiltinTypeKind::iter() {
            self.typeDict.addBuiltinType(ty);
        }
    }

    pub fn parse(&mut self) -> (AstTu, Vec<CompileMsg>) {
        self.initBuiltinTypes();
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
