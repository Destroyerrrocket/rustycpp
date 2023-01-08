use crate::{
    lex::{lexer::Lexer, token::Token},
    utils::{
        structs::{CompileMsg, FileTokPos},
        unsafeallocator::UnsafeAllocator,
    },
};

#[derive(Clone, Copy)]
pub struct StateBufferedLexer {
    minimumToken: usize,
    currentToken: usize,
    maximumToken: usize,
}

pub struct BufferedLexer {
    lexer: Lexer,
    alloc: UnsafeAllocator,
    tokens: Vec<&'static FileTokPos<Token>>,
}

impl BufferedLexer {
    fn lastTokIndex(&mut self) -> usize {
        self.tokens.len() - 1
    }

    fn tryGetNextToken(&mut self) -> bool {
        match self.lexer.next() {
            Some(token) => self.tokens.push(self.alloc.alloc().alloc(token)),
            None => return false,
        }
        true
    }

    fn internalGetUnchecked(&mut self, index: usize) -> &'static FileTokPos<Token> {
        self.tokens[index]
    }

    fn internalGetChecked(&mut self, index: usize) -> Option<&'static FileTokPos<Token>> {
        self.tokens.get(index).copied()
    }
}

impl BufferedLexer {
    pub fn new(lexer: Lexer) -> (Self, StateBufferedLexer) {
        let s = Self {
            lexer,
            tokens: vec![],
            alloc: UnsafeAllocator::new(),
        };
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
            return true;
        }

        if lexpos.currentToken > lexpos.maximumToken {
            return true;
        }

        if lexpos.currentToken >= self.tokens.len() {
            if self.tryGetNextToken() {
                return false;
            }
            lexpos.maximumToken = lexpos.currentToken.saturating_sub(1);
            return true;
        }
        false
    }

    pub fn consumeToken(&mut self, lexpos: &mut StateBufferedLexer) -> bool {
        if !self.reachedEnd(lexpos) {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn consumeTokenIfEq(&mut self, lexpos: &mut StateBufferedLexer, tok: Token) -> bool {
        if !self.reachedEnd(lexpos)
            && self.internalGetUnchecked(lexpos.currentToken).tokPos.tok == tok
        {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn consumeTokenIf(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        cond: fn(&Token) -> bool,
    ) -> bool {
        if !self.reachedEnd(lexpos)
            && cond(&self.internalGetUnchecked(lexpos.currentToken).tokPos.tok)
        {
            lexpos.currentToken += 1;
            return true;
        }
        false
    }

    pub fn get(&mut self, lexpos: &mut StateBufferedLexer) -> Option<&'static FileTokPos<Token>> {
        if self.reachedEnd(lexpos) {
            return None;
        }
        self.internalGetChecked(lexpos.currentToken)
    }

    pub fn getIfEq(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        tok: Token,
    ) -> Option<&'static FileTokPos<Token>> {
        if !self.reachedEnd(lexpos)
            && self.internalGetUnchecked(lexpos.currentToken).tokPos.tok == tok
        {
            return self.internalGetChecked(lexpos.currentToken);
        }
        None
    }

    pub fn getIf(
        &mut self,

        lexpos: &mut StateBufferedLexer,
        cond: fn(&Token) -> bool,
    ) -> Option<&'static FileTokPos<Token>> {
        if !self.reachedEnd(lexpos)
            && cond(&self.internalGetUnchecked(lexpos.currentToken).tokPos.tok)
        {
            return self.internalGetChecked(lexpos.currentToken);
        }
        None
    }

    pub fn ifEqOffset(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        tok: Token,
        offset: isize,
    ) -> bool {
        if lexpos.maximumToken < lexpos.minimumToken {
            return false;
        }

        match lexpos.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                if pos > lexpos.maximumToken || pos < lexpos.minimumToken {
                    return false;
                }
                while pos >= self.tokens.len() && self.tryGetNextToken() {}
                if pos >= self.tokens.len() {
                    return false;
                }
                return self.internalGetUnchecked(pos).tokPos.tok == tok;
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

    pub fn getConsumeToken(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> Option<&'static FileTokPos<Token>> {
        if self.reachedEnd(lexpos) {
            return None;
        }
        lexpos.currentToken += 1;
        return self.internalGetChecked(lexpos.currentToken - 1);
    }

    pub fn getConsumeTokenIfEq(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        tok: Token,
    ) -> Option<&'static FileTokPos<Token>> {
        if !self.reachedEnd(lexpos)
            && self.internalGetUnchecked(lexpos.currentToken).tokPos.tok == tok
        {
            lexpos.currentToken += 1;
            return self.internalGetChecked(lexpos.currentToken - 1);
        }
        None
    }

    pub fn getConsumeTokenIfIdentifier(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> Option<&'static FileTokPos<Token>> {
        if !self.reachedEnd(lexpos)
            && matches!(
                self.internalGetUnchecked(lexpos.currentToken).tokPos.tok,
                Token::Identifier(_)
            )
        {
            lexpos.currentToken += 1;
            return self.internalGetChecked(lexpos.currentToken - 1);
        }
        None
    }

    pub fn getConsumeTokenIf(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        cond: fn(&Token) -> bool,
    ) -> Option<&'static FileTokPos<Token>> {
        if !self.reachedEnd(lexpos)
            && cond(&self.internalGetUnchecked(lexpos.currentToken).tokPos.tok)
        {
            lexpos.currentToken += 1;
            return self.internalGetChecked(lexpos.currentToken - 1);
        }
        None
    }

    pub fn getWithOffset(
        &mut self,
        lexpos: &StateBufferedLexer,
        offset: isize,
    ) -> Option<&'static FileTokPos<Token>> {
        if lexpos.maximumToken < lexpos.minimumToken {
            return None;
        }

        match lexpos.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                if pos > lexpos.maximumToken || pos < lexpos.minimumToken {
                    return None;
                }
                while pos >= self.tokens.len() && self.tryGetNextToken() {}
                if pos >= self.tokens.len() {
                    return None;
                }
                return self.internalGetChecked(pos);
            }
            None => None,
        }
    }

    pub fn getWithOffsetSaturating(
        &mut self,
        lexpos: &StateBufferedLexer,
        offset: isize,
    ) -> &'static FileTokPos<Token> {
        if lexpos.maximumToken < lexpos.minimumToken {
            return self.internalGetChecked(lexpos.minimumToken).unwrap();
        }

        match lexpos.currentToken.checked_add_signed(offset) {
            Some(mut pos) => {
                pos = pos.clamp(lexpos.minimumToken, lexpos.maximumToken);

                while pos >= self.tokens.len() && self.tryGetNextToken() {}
                if pos >= self.tokens.len() {
                    return self.tokens.last().unwrap();
                }
                self.internalGetChecked(pos).unwrap()
            }
            None => self.internalGetChecked(lexpos.minimumToken).unwrap(),
        }
    }

    // move back lexpos nth positions
    #[allow(clippy::unused_self)]
    pub fn moveBack(&mut self, lexpos: &mut StateBufferedLexer, n: usize) {
        if lexpos.maximumToken < lexpos.minimumToken {
            return;
        }

        lexpos.currentToken = lexpos
            .currentToken
            .saturating_sub(n)
            .clamp(lexpos.minimumToken, lexpos.maximumToken);
    }

    pub fn next(&mut self, lexpos: &mut StateBufferedLexer) -> bool {
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

    pub fn moveForward(&mut self, lexpos: &mut StateBufferedLexer, n: usize) -> bool {
        if lexpos.maximumToken < lexpos.minimumToken {
            return false;
        }

        lexpos.currentToken = lexpos
            .currentToken
            .saturating_add(n)
            .clamp(lexpos.minimumToken, lexpos.maximumToken);
        while lexpos.currentToken > self.tokens.len() && self.tryGetNextToken() {}
        lexpos.currentToken <= self.tokens.len() // Beware to not consume the token of the destination; This way we can alter the lexer correctly if necessary.
    }

    #[allow(clippy::unused_self)]
    pub fn moveStateToOtherState(
        &mut self,
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
