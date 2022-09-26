use std::fmt::Debug;
use std::sync::Arc;

use super::structs::{CompileFile, FilePreTokPos};

// Unsuported. Haven't removed it as it may be useful someday
#[derive(Debug)]
pub enum LalrPopLexerWrapperState {
    Normal,
    SkipWhitespace,
    SkipWsNl,
}

#[derive(Debug)]
pub struct LalrPopLexerWrapper<'slice, T: Clone + Debug> {
    tokens: &'slice [FilePreTokPos<T>],
    state: LalrPopLexerWrapperState,
    idx: usize,
}

impl<'slice, T: Clone + Debug> LalrPopLexerWrapper<'slice, T> {
    pub const fn new(tokens: &'slice [FilePreTokPos<T>]) -> LalrPopLexerWrapper<T> {
        LalrPopLexerWrapper {
            tokens,
            state: LalrPopLexerWrapperState::Normal,
            idx: 0,
        }
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
            self.idx += 1;
            /*
            if tok.kind.isWhitespace() && matches!(self.state, LalrPopLexerWrapperState::SkipWhitespace | LalrPopLexerWrapperState::SkipWsNl) {
                continue;
            }
            if matches!(tok.kind, PreToken::Newline) && matches!(self.state,LalrPopLexerWrapperState::SkipWsNl) {
                continue;
            }
            */
            Some(Ok((
                (tok.tokPos.start, tok.file.clone()),
                tok.tokPos.tok.clone(),
                (tok.tokPos.end, tok.file.clone()),
            )))
        }
    }
}
