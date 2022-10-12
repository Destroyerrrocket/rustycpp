use std::rc::Rc;
use std::sync::Mutex;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;

use crate::grammars::generated::maincpp::mainCpp;
use crate::grammars::generated::maincppparser::Translation_unitContextAll;
use crate::lexer::lexer::Lexer;
use crate::utils::antlrlexerwrapper::{AntlrLexerIteratorWrapper, LexerWrapperErrorStrategy};
use crate::utils::compilerstate::CompilerState;
use crate::utils::structs::CompileMsg;

pub struct Parser {
    lexer: Lexer,
    filePath: String,
    compilerState: CompilerState,
}

impl Parser {
    pub const fn new(lexer: Lexer, filePath: String, compilerState: CompilerState) -> Self {
        Self {
            lexer,
            filePath,
            compilerState,
        }
    }

    // TODO: If all the grammar errors were recoverable, we should still report them with the lexical errors.
    pub fn parse(&mut self) -> Result<Rc<Translation_unitContextAll>, Vec<CompileMsg>> {
        let errors = Rc::new(Mutex::new(vec![]));

        let tree = {
            let tokenStream = CommonTokenStream::new(AntlrLexerIteratorWrapper::new(
                unsafe { &mut *std::ptr::addr_of_mut!(self.lexer) },
                self.filePath.clone(),
            ));

            let compileFile = self
                .compilerState
                .compileFiles
                .lock()
                .unwrap()
                .getFile(&self.filePath);

            let mut basicParser = mainCpp::with_strategy(
                tokenStream,
                LexerWrapperErrorStrategy::new(errors.clone(), compileFile),
            );

            basicParser.translation_unit().unwrap()
        };

        let errorsLexer = self.lexer.errors();
        let errors = errors.lock().unwrap();
        if errorsLexer.is_empty() {
            if errors.is_empty() {
                Ok(tree)
            } else {
                Err(errors.clone())
            }
        } else {
            Err(errorsLexer)
        }
    }

    // TODO: If all the grammar errors were recoverable, we should still report them with the lexical errors.
    pub fn parseStringTree(&mut self) -> Result<String, Vec<CompileMsg>> {
        let errors = Rc::new(Mutex::new(vec![]));

        let tree = {
            let tokenStream = CommonTokenStream::new(AntlrLexerIteratorWrapper::new(
                unsafe { &mut *std::ptr::addr_of_mut!(self.lexer) },
                self.filePath.clone(),
            ));

            let compileFile = self
                .compilerState
                .compileFiles
                .lock()
                .unwrap()
                .getFile(&self.filePath);

            let mut basicParser = mainCpp::with_strategy(
                tokenStream,
                LexerWrapperErrorStrategy::new(errors.clone(), compileFile),
            );

            basicParser
                .translation_unit()
                .unwrap()
                .to_string_tree(&*basicParser)
        };

        let errorsLexer = self.lexer.errors();
        let errors = errors.lock().unwrap();
        if errorsLexer.is_empty() {
            if errors.is_empty() {
                Ok(tree)
            } else {
                Err(errors.clone())
            }
        } else {
            Err(errorsLexer)
        }
    }
}
