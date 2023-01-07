use std::cell::UnsafeCell;

use crate::{
    lex::{lexer::Lexer, token::Token},
    utils::structs::{CompileMsg, FileTokPos},
};

#[derive(Clone, Copy)]
pub struct StateBufferedLexer {
    minimumToken: usize,
    currentToken: usize,
    maximumToken: usize,
}

pub struct BufferedLexer {
    lexer: UnsafeCell<Lexer>,
    tokens: UnsafeCell<Vec<FileTokPos<Token>>>,
}

impl BufferedLexer {
    #[allow(clippy::mut_from_ref)]
    fn tokens(&self) -> &mut Vec<FileTokPos<Token>> {
        unsafe { &mut *self.tokens.get() }
    }

    fn lastTokIndex(&self) -> usize {
        self.tokens().len() - 1
    }

    fn tryGetNextToken(&self) -> bool {
        match unsafe { &mut *self.lexer.get() }.next() {
            Some(token) => self.tokens().push(token),
            None => return false,
        }
        true
    }
}

impl BufferedLexer {
    pub fn new(lexer: Lexer) -> (Self, StateBufferedLexer) {
        let s = Self {
            lexer: UnsafeCell::new(lexer),
            tokens: UnsafeCell::new(vec![]),
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

    pub fn errors(&mut self) -> Vec<CompileMsg> {
        unsafe { &mut *self.lexer.get() }.errors()
    }

    pub fn reachedEnd(&self, lexpos: &mut StateBufferedLexer) -> bool {
        if lexpos.maximumToken < lexpos.minimumToken {
            return true;
        }

        if lexpos.currentToken > lexpos.maximumToken {
            return true;
        }

        if lexpos.currentToken >= self.tokens().len() {
            if self.tryGetNextToken() {
                return false;
            } else {
                lexpos.maximumToken = lexpos.currentToken.saturating_sub(1);
                return true;
            }
        }
        false
    }

    pub fn consumeToken(&self, lexpos: &mut StateBufferedLexer) -> bool {
        if !self.reachedEnd(lexpos) {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn consumeTokenIfEq(&self, lexpos: &mut StateBufferedLexer, tok: Token) -> bool {
        if !self.reachedEnd(lexpos) && self.tokens()[lexpos.currentToken].tokPos.tok == tok {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn consumeTokenIf(
        &self,
        lexpos: &mut StateBufferedLexer,
        cond: fn(&Token) -> bool,
    ) -> bool {
        if !self.reachedEnd(lexpos) && cond(&self.tokens()[lexpos.currentToken].tokPos.tok) {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn get(&self, lexpos: &mut StateBufferedLexer) -> Option<&FileTokPos<Token>> {
        if self.reachedEnd(lexpos) {
            return None;
        }
        self.tokens().get(lexpos.currentToken)
    }

    pub fn getIfEq(
        &self,
        lexpos: &mut StateBufferedLexer,
        tok: Token,
    ) -> Option<&FileTokPos<Token>> {
        if !self.reachedEnd(lexpos) && self.tokens()[lexpos.currentToken].tokPos.tok == tok {
            return self.tokens().get(lexpos.currentToken);
        }
        None
    }

    pub fn getIf(
        &self,
        lexpos: &mut StateBufferedLexer,
        cond: fn(&Token) -> bool,
    ) -> Option<&FileTokPos<Token>> {
        if !self.reachedEnd(lexpos) && cond(&self.tokens()[lexpos.currentToken].tokPos.tok) {
            return self.tokens().get(lexpos.currentToken);
        }
        None
    }

    pub fn ifEqOffset(&self, lexpos: &mut StateBufferedLexer, tok: Token, offset: isize) -> bool {
        if lexpos.maximumToken < lexpos.minimumToken {
            return false;
        }

        match lexpos.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                if pos > lexpos.maximumToken || pos < lexpos.minimumToken {
                    return false;
                }
                while pos >= self.tokens().len() && self.tryGetNextToken() {}
                if pos >= self.tokens().len() {
                    return false;
                }
                return self.tokens()[pos].tokPos.tok == tok;
            }
            None => false,
        }
    }

    pub const fn makeProtectedRange(
        start: &StateBufferedLexer,
        end: &StateBufferedLexer,
    ) -> StateBufferedLexer {
        let mut newState = *start;
        newState.minimumToken = start.currentToken;
        newState.maximumToken = end.currentToken;
        newState
    }

    pub fn getConsumeToken(&self, lexpos: &mut StateBufferedLexer) -> Option<&FileTokPos<Token>> {
        if self.reachedEnd(lexpos) {
            return None;
        }
        lexpos.currentToken += 1;
        return self.tokens().get(lexpos.currentToken - 1);
    }

    pub fn getConsumeTokenIfEq(
        &self,
        lexpos: &mut StateBufferedLexer,
        tok: Token,
    ) -> Option<&FileTokPos<Token>> {
        if !self.reachedEnd(lexpos) && self.tokens()[lexpos.currentToken].tokPos.tok == tok {
            lexpos.currentToken += 1;
            return self.tokens().get(lexpos.currentToken - 1);
        }
        None
    }

    pub fn getConsumeTokenIf(
        &self,
        lexpos: &mut StateBufferedLexer,
        cond: fn(&Token) -> bool,
    ) -> Option<&FileTokPos<Token>> {
        if !self.reachedEnd(lexpos) && cond(&self.tokens()[lexpos.currentToken].tokPos.tok) {
            lexpos.currentToken += 1;
            return self.tokens().get(lexpos.currentToken - 1);
        }
        None
    }

    pub fn getWithOffset(
        &self,
        lexpos: &StateBufferedLexer,
        offset: isize,
    ) -> Option<&FileTokPos<Token>> {
        if lexpos.maximumToken < lexpos.minimumToken {
            return None;
        }

        match lexpos.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                if pos > lexpos.maximumToken || pos < lexpos.minimumToken {
                    return None;
                }
                while pos >= self.tokens().len() && self.tryGetNextToken() {}
                if pos >= self.tokens().len() {
                    return None;
                }
                return self.tokens().get(pos);
            }
            None => None,
        }
    }

    pub fn getWithOffsetSaturating(
        &self,
        lexpos: &StateBufferedLexer,
        offset: isize,
    ) -> &FileTokPos<Token> {
        if lexpos.maximumToken < lexpos.minimumToken {
            return self.tokens().get(lexpos.minimumToken).unwrap();
        }

        match lexpos.currentToken.checked_add_signed(offset) {
            Some(mut pos) => {
                pos = pos.clamp(lexpos.minimumToken, lexpos.maximumToken);

                while pos >= self.tokens().len() && self.tryGetNextToken() {}
                if pos >= self.tokens().len() {
                    return self.tokens().last().unwrap();
                }
                self.tokens().get(pos).unwrap()
            }
            None => self.tokens().get(lexpos.minimumToken).unwrap(),
        }
    }

    // move back lexpos nth positions
    #[allow(clippy::unused_self)]
    pub fn moveBack(&self, lexpos: &mut StateBufferedLexer, n: usize) {
        if lexpos.maximumToken < lexpos.minimumToken {
            return;
        }

        lexpos.currentToken = lexpos
            .currentToken
            .saturating_sub(n)
            .clamp(lexpos.minimumToken, lexpos.maximumToken);
    }

    pub fn next(&self, lexpos: &mut StateBufferedLexer) -> bool {
        if lexpos.maximumToken < lexpos.minimumToken {
            return false;
        }

        if self.reachedEnd(lexpos) {
            return false;
        }

        lexpos.currentToken =
            (lexpos.currentToken + 1).clamp(lexpos.minimumToken, lexpos.maximumToken);
        true
    }

    pub fn moveForward(&self, lexpos: &mut StateBufferedLexer, n: usize) -> bool {
        if lexpos.maximumToken < lexpos.minimumToken {
            return false;
        }

        lexpos.currentToken = lexpos
            .currentToken
            .saturating_add(n)
            .clamp(lexpos.minimumToken, lexpos.maximumToken);
        while lexpos.currentToken > self.tokens().len() && self.tryGetNextToken() {}
        return lexpos.currentToken <= self.tokens().len(); // Beware to not consume the token of the destination; This way we can alter the lexer correctly if necessary.
    }

    #[allow(clippy::unused_self)]
    pub fn moveStateToOtherState(
        &self,
        lexpos: &mut StateBufferedLexer,
        otherpos: &mut StateBufferedLexer,
    ) -> bool {
        // We know that the states are valid, so we just check for ranges
        if otherpos.currentToken > lexpos.maximumToken
            || otherpos.currentToken < lexpos.minimumToken
        {
            return false;
        }
        lexpos.currentToken = otherpos.currentToken;
        true
    }
}
