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

    pub fn errors(&mut self) -> Vec<CompileMsg> {
        self.lexer.errors()
    }

    pub fn reachedEnd(&mut self, lexpos: &mut StateBufferedLexer) -> bool {
        if lexpos.maximumToken < lexpos.minimumToken {
            return false;
        }

        if lexpos.currentToken > lexpos.maximumToken {
            return true;
        }

        if lexpos.currentToken + 1 > self.tokens.len() {
            if self.tryGetNextToken() {
                return false;
            } else {
                lexpos.maximumToken = lexpos.currentToken;
                return true;
            }
        }
        return false;
    }

    pub fn consumeToken(&mut self, lexpos: &mut StateBufferedLexer) -> bool {
        if !self.reachedEnd(lexpos) {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn consumeTokenIfEq(&mut self, lexpos: &mut StateBufferedLexer, tok: Token) -> bool {
        if !self.reachedEnd(lexpos) && self.tokens[lexpos.currentToken].tokPos.tok == tok {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn consumeTokenIf(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        cond: fn(Token) -> bool,
    ) -> bool {
        if !self.reachedEnd(lexpos) && cond(self.tokens[lexpos.currentToken].tokPos.tok) {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn get(&mut self, lexpos: &mut StateBufferedLexer) -> Option<FileTokPos<Token>> {
        if self.reachedEnd(lexpos) {
            return None;
        }
        Some(self.tokens[lexpos.currentToken])
    }

    pub fn makeProtectedRange(
        start: &StateBufferedLexer,
        end: &StateBufferedLexer,
    ) -> StateBufferedLexer {
        let mut newState = *start;
        newState.minimumToken = start.currentToken;
        newState.maximumToken = end.currentToken;
        debug_assert!(newState.minimumToken <= newState.maximumToken);
        newState
    }

    pub fn getConsumeToken(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> Option<FileTokPos<Token>> {
        if self.reachedEnd(lexpos) {
            return None;
        }
        lexpos.currentToken += 1;
        return Some(self.tokens[lexpos.currentToken - 1]);
    }

    pub fn getConsumeTokenIfEq(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        tok: Token,
    ) -> Option<FileTokPos<Token>> {
        if !self.reachedEnd(lexpos) && self.tokens[lexpos.currentToken].tokPos.tok == tok {
            lexpos.currentToken += 1;
            return Some(self.tokens[lexpos.currentToken - 1]);
        }
        None
    }

    pub fn getConsumeTokenIf(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        cond: fn(Token) -> bool,
    ) -> Option<FileTokPos<Token>> {
        if !self.reachedEnd(lexpos) && cond(self.tokens[lexpos.currentToken].tokPos.tok) {
            lexpos.currentToken += 1;
            return Some(self.tokens[lexpos.currentToken - 1]);
        }
        None
    }

    pub fn getWithOffset(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        offset: isize,
    ) -> Option<FileTokPos<Token>> {
        if lexpos.maximumToken < lexpos.minimumToken {
            return None;
        }

        match lexpos.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                if pos > lexpos.maximumToken || pos < lexpos.minimumToken {
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
        lexpos: &mut StateBufferedLexer,
        offset: isize,
    ) -> FileTokPos<Token> {
        if lexpos.maximumToken < lexpos.minimumToken {
            return self.tokens[lexpos.minimumToken];
        }

        match lexpos.currentToken.checked_add_signed(offset) {
            Some(mut pos) => {
                pos = pos.clamp(lexpos.minimumToken, lexpos.maximumToken);

                while pos > self.lastTokIndex() && self.tryGetNextToken() {}
                if pos > self.lastTokIndex() {
                    return *self.tokens.last().unwrap();
                }
                self.tokens[pos]
            }
            None => self.tokens[lexpos.minimumToken],
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

    pub fn moveForward(&mut self, lexpos: &mut StateBufferedLexer, n: usize) -> bool {
        if lexpos.maximumToken < lexpos.minimumToken {
            return false;
        }

        lexpos.currentToken = lexpos
            .currentToken
            .saturating_add(n)
            .clamp(lexpos.minimumToken, lexpos.maximumToken);
        while lexpos.currentToken > self.lastTokIndex() && self.tryGetNextToken() {}
        return lexpos.currentToken <= self.lastTokIndex();
    }

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
