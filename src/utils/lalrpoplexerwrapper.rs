//! Wrapper over a token collection for lalrpop consumption.
use std::fmt::Debug;
use std::sync::Arc;

use super::structs::{CompileFile, FilePreTokPos};

#[derive(Debug)]
/// The wrapper over the token collection
pub struct LalrPopLexerWrapper<'slice, T: Clone + Debug> {
    /// The tokens
    tokens: &'slice [FilePreTokPos<T>],
}

impl<'slice, T: Clone + Debug> LalrPopLexerWrapper<'slice, T> {
    /// Create a new wrapper
    pub const fn new(tokens: &'slice [FilePreTokPos<T>]) -> LalrPopLexerWrapper<T> {
        LalrPopLexerWrapper { tokens }
    }
}

impl<'slice, T: Clone + Debug> Iterator for LalrPopLexerWrapper<'slice, T> {
    type Item = Result<((usize, Arc<CompileFile>), T, (usize, Arc<CompileFile>)), ()>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.is_empty() {
            None
        } else {
            let tok = &self.tokens[0];
            self.tokens = &self.tokens[1..];

            Some(Ok((
                (tok.tokPos.start, tok.file.clone()),
                tok.tokPos.tok.clone(),
                (tok.tokPos.end, tok.file.clone()),
            )))
        }
    }
}
