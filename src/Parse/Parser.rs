use strum::IntoEnumIterator;

use crate::{
    Ast::{
        Common::{AstTu, CommonAst},
        Type::Builtin::BuiltinTypeKind,
    },
    Compiler::TranslationUnit,
    Lex::Token::Token,
    Sema::AstContext::AstContext,
    Utils::{
        CompilerState::CompilerState,
        Structs::{CompileMsg, FileTokPos},
    },
};

use super::BufferedLexer::{BufferedLexer, StateBufferedLexer};

mod ParserParse;
mod ParserSema;

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

    errors: Vec<CompileMsg>,
    astContext: AstContext,
}

impl Parser {
    pub fn new(
        tokens: Vec<FileTokPos<Token>>,
        filePath: TranslationUnit,
        compilerState: CompilerState,
    ) -> Self {
        let (lexer, lexerStart) = BufferedLexer::new(tokens);
        Self {
            lexer,
            lexerStart,
            filePath,
            compilerState,
            moduleImportState: ModuleImportState::StartFile,
            errors: vec![],
            astContext: AstContext::new(),
        }
    }

    fn initBuiltinTypes(&mut self) {
        for ty in BuiltinTypeKind::iter() {
            self.astContext.typeDict.addBuiltinType(ty);
        }
    }

    pub fn parse(&mut self) -> (AstTu, Vec<CompileMsg>) {
        self.initBuiltinTypes();
        let tu = self.parseTu();
        (tu, self.errors.clone())
    }

    pub fn printStringTree(ast: AstTu) -> String {
        ast.getDebugNode().to_string()
    }

    pub fn lexer(&mut self) -> &mut BufferedLexer {
        &mut self.lexer
    }

    pub fn alloc(&self) -> &'static bumpalo::Bump {
        self.astContext.alloc.alloc()
    }

    pub fn addError(&mut self, msg: CompileMsg) {
        self.errors.push(msg);
    }
}
