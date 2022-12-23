use crate::{
    lex::{lexer::Lexer, token::Token},
    utils::structs::FileTokPos,
};

#[derive(Clone, Copy)]
pub struct StateBufferedLexer {
    minimumToken: usize,
    currentToken: usize,
    maximumToken: usize,
}

pub struct BufferedLexer {
    lexer: Lexer,
    tokens: Vec<FileTokPos<Token>>,
}

impl BufferedLexer {
    fn lastTokIndex(&self) -> usize {
        self.tokens.len() - 1
    }

    fn tryGetNextToken(&mut self) -> bool {
        match self.lexer.next() {
            Some(token) => self.tokens.push(token),
            None => return false,
        }
        true
    }
}

impl BufferedLexer {
    pub fn new(lexer: Lexer) -> (Self, StateBufferedLexer) {
        let mut s = Self {
            lexer,
            tokens: vec![],
        };
        s.tryGetNextToken();
        (
            s,
            StateBufferedLexer {
                minimumToken: 0,
                currentToken: 0,
                maximumToken: usize::MAX,
            },
        )
    }

    pub fn reachedEnd(&mut self, state: &mut StateBufferedLexer) -> bool {
        if state.currentToken > state.maximumToken {
            return true;
        }
        if state.currentToken > self.lastTokIndex() {
            if self.tryGetNextToken() {
                return false;
            } else {
                state.maximumToken = state.currentToken;
                return true;
            }
        }
        return false;
    }

    pub fn consumeToken(&mut self, state: &mut StateBufferedLexer) -> bool {
        if !self.reachedEnd(state) {
            state.currentToken += 1;
            return true;
        }
        false
    }

    pub fn consumeTokenIfEq(&mut self, state: &mut StateBufferedLexer, tok: Token) -> bool {
        if !self.reachedEnd(state) && self.tokens[state.currentToken].tokPos.tok == tok {
            state.currentToken += 1;
            return true;
        }
        false
    }

    pub fn consumeTokenIf(
        &mut self,
        state: &mut StateBufferedLexer,
        cond: fn(Token) -> bool,
    ) -> bool {
        if !self.reachedEnd(state) && cond(self.tokens[state.currentToken].tokPos.tok) {
            state.currentToken += 1;
            return true;
        }
        false
    }

    pub fn get(&mut self, state: &mut StateBufferedLexer) -> Option<FileTokPos<Token>> {
        if self.reachedEnd(state) {
            return None;
        }
        Some(self.tokens[state.currentToken])
    }

    pub fn makeProtectedRange(
        start: &mut StateBufferedLexer,
        end: &mut StateBufferedLexer,
    ) -> StateBufferedLexer {
        let mut newState = *start;
        newState.minimumToken = start.currentToken;
        newState.maximumToken = end.currentToken;
        debug_assert!(newState.minimumToken <= newState.maximumToken);
        newState
    }

    pub fn getConsumeToken(&mut self, state: &mut StateBufferedLexer) -> Option<FileTokPos<Token>> {
        if self.reachedEnd(state) {
            return None;
        }
        state.currentToken += 1;
        return Some(self.tokens[state.currentToken - 1]);
    }

    pub fn getConsumeTokenIfEq(
        &mut self,
        state: &mut StateBufferedLexer,
        tok: Token,
    ) -> Option<FileTokPos<Token>> {
        if !self.reachedEnd(state) && self.tokens[state.currentToken].tokPos.tok == tok {
            state.currentToken += 1;
            return Some(self.tokens[state.currentToken - 1]);
        }
        None
    }

    pub fn getConsumeTokenIf(
        &mut self,
        state: &mut StateBufferedLexer,
        cond: fn(Token) -> bool,
    ) -> Option<FileTokPos<Token>> {
        if !self.reachedEnd(state) && cond(self.tokens[state.currentToken].tokPos.tok) {
            state.currentToken += 1;
            return Some(self.tokens[state.currentToken - 1]);
        }
        None
    }

    pub fn getWithOffset(
        &mut self,
        state: &mut StateBufferedLexer,
        offset: isize,
    ) -> Option<FileTokPos<Token>> {
        match state.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                if pos > state.maximumToken || pos < state.minimumToken {
                    return None;
                }
                while pos > self.lastTokIndex() && self.tryGetNextToken() {}
                if pos > self.lastTokIndex() {
                    return None;
                }
                Some(self.tokens[pos])
            }
            None => None,
        }
    }

    pub fn getWithOffsetSaturating(
        &mut self,
        state: &mut StateBufferedLexer,
        offset: isize,
    ) -> FileTokPos<Token> {
        match state.currentToken.checked_add_signed(offset) {
            Some(mut pos) => {
                pos = pos.clamp(state.minimumToken, state.maximumToken);

                while pos > self.lastTokIndex() && self.tryGetNextToken() {}
                if pos > self.lastTokIndex() {
                    return *self.tokens.last().unwrap();
                }
                self.tokens[pos]
            }
            None => self.tokens[state.minimumToken],
        }
    }
}
