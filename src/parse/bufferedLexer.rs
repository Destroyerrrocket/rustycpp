use crate::{
    lex::{lexer::Lexer, token::Token},
    utils::structs::{CompileError, CompileMsg, FileTokPos},
};

pub struct StateBufferedLexer {
    currentToken: usize,
}

pub struct BufferedLexer {
    lexer: Lexer,
    tokens: Vec<FileTokPos<Token>>,
    currentToken: usize,
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
    pub fn new(lexer: Lexer) -> Self {
        let mut s = Self {
            lexer,
            tokens: vec![],
            currentToken: 0,
        };
        s.tryGetNextToken();
        s
    }

    pub fn reachedEnd(&self) -> bool {
        self.currentToken > self.lastTokIndex()
    }

    pub fn consumeToken(&mut self) -> bool {
        if !self.reachedEnd() {
            self.currentToken += 1;
            if self.reachedEnd() {
                self.tryGetNextToken();
            }
            return true;
        }
        false
    }

    pub fn consumeTokenIfEq(&mut self, tok: Token) -> bool {
        if !self.reachedEnd() && self.tokens[self.currentToken].tokPos.tok == tok {
            self.currentToken += 1;
            if self.reachedEnd() {
                self.tryGetNextToken();
            }
            return true;
        }
        false
    }

    pub fn consumeTokenIf(&mut self, cond: fn(Token) -> bool) -> bool {
        if !self.reachedEnd() && cond(self.tokens[self.currentToken].tokPos.tok) {
            self.currentToken += 1;
            if self.reachedEnd() {
                self.tryGetNextToken();
            }
            return true;
        }
        false
    }

    pub fn saveState(&self) -> StateBufferedLexer {
        StateBufferedLexer {
            currentToken: self.currentToken,
        }
    }

    pub fn loadState(&mut self, state: StateBufferedLexer) {
        self.currentToken = state.currentToken;
    }

    pub fn movePos(&mut self, offset: isize) -> Result<(), CompileMsg> {
        if let Some(pos) = self.currentToken.checked_add_signed(offset) {
            while pos > self.lastTokIndex() && self.tryGetNextToken() {}
            if pos > self.lastTokIndex() {
                return Err(CompileError::from_preTo(
                "Internal lexer error. We tried to perform a move lexing point operation that goes beyond the EOF.",
                &self.tokens[self.currentToken],
            ));
            }
        } else {
            return Err(CompileError::from_preTo(
                "Internal lexer error. We tried to perform a move lexing point operation that goes before the start of the file.",
                &self.tokens[self.currentToken],
            ));
        }
        return Ok(());
    }

    pub fn get(&mut self) -> Option<FileTokPos<Token>> {
        if self.reachedEnd() {
            return None;
        }
        Some(self.tokens[self.currentToken])
    }

    pub fn getConsumeToken(&mut self) -> Option<FileTokPos<Token>> {
        if self.reachedEnd() {
            return None;
        }
        self.currentToken += 1;
        if self.reachedEnd() {
            self.tryGetNextToken();
        }
        return Some(self.tokens[self.currentToken - 1]);
    }

    pub fn getConsumeTokenIfEq(&mut self, tok: Token) -> Option<FileTokPos<Token>> {
        if !self.reachedEnd() && self.tokens[self.currentToken].tokPos.tok == tok {
            self.currentToken += 1;
            if self.reachedEnd() {
                self.tryGetNextToken();
            }
            return Some(self.tokens[self.currentToken - 1]);
        }
        None
    }

    pub fn getConsumeTokenIf(&mut self, cond: fn(Token) -> bool) -> Option<FileTokPos<Token>> {
        if !self.reachedEnd() && cond(self.tokens[self.currentToken].tokPos.tok) {
            self.currentToken += 1;
            if self.reachedEnd() {
                self.tryGetNextToken();
            }
            return Some(self.tokens[self.currentToken - 1]);
        }
        None
    }

    pub fn getWithOffset(&mut self, offset: isize) -> Option<FileTokPos<Token>> {
        match self.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                while pos > self.lastTokIndex() && self.tryGetNextToken() {}
                if pos > self.lastTokIndex() {
                    return None;
                }
                Some(self.tokens[pos])
            }
            None => None,
        }
    }

    pub fn getWithOffsetSaturating(&mut self, offset: isize) -> FileTokPos<Token> {
        match self.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                while pos > self.lastTokIndex() && self.tryGetNextToken() {}
                if pos > self.lastTokIndex() {
                    return *self.tokens.last().unwrap();
                }
                self.tokens[pos]
            }
            None => *self.tokens.first().unwrap(),
        }
    }
}
