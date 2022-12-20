use crate::{
    lex::{lexer::Lexer, token::Token},
    utils::structs::FileTokPos,
};

pub struct BufferedLexer {
    lexer: Lexer,
    tokens: Vec<FileTokPos<Token>>,
    currentToken: usize,
}

impl BufferedLexer {
    fn new(lexer: Lexer) -> Self {
        let mut s = Self {
            lexer,
            tokens: vec![],
            currentToken: 0,
        };
        s.tryGetNextToken();
        s
    }

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
    pub fn reachedEnd(&self) -> bool {
        self.currentToken > self.lastTokIndex()
    }

    pub fn consumeToken(&mut self) -> bool {
        if self.reachedEnd() {
            self.currentToken += 1;
            if self.reachedEnd() {
                return self.tryGetNextToken();
            } else {
                false
            }
        } else {
            unreachable!("Tried to consume token when there was no token left to consume");
        }
    }

    pub fn getWithOffset(&mut self, offset: isize) -> Option<FileTokPos<Token>> {
        match self.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                while pos > self.lastTokIndex() && self.tryGetNextToken() {}
                if pos > self.lastTokIndex() {
                    return None;
                }
                Some(self.tokens[pos].clone())
            }
            None => None,
        }
    }

    pub fn getWithOffsetSaturating(&mut self, offset: isize) -> FileTokPos<Token> {
        match self.currentToken.checked_add_signed(offset) {
            Some(pos) => {
                while pos > self.lastTokIndex() && self.tryGetNextToken() {}
                if pos > self.lastTokIndex() {
                    return self.tokens.last().unwrap().clone();
                }
                self.tokens[pos].clone()
            }
            None => self.tokens.first().unwrap().clone(),
        }
    }
}
