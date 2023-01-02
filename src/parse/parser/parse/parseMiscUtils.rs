use crate::{
    lex::token::Token,
    parse::bufferedLexer::{BufferedLexer, StateBufferedLexer},
};

use super::super::Parser;

pub enum ParseMacroMatched {
    /// The macro was matched, and the tokens were consumed. Might return None (or equivalent) in error cases.
    Matched,
    /// The macro was not matched, and the token was not consumed.
    NotMatched,
}

impl Parser {
    /** Balances a pattern starting with either a '(', '{' or '[' (otherwise, UB), and ending with the corresponding closing character.
     * It returns a range containing the contents inside the pattern, without the outer characters (aka, "(hello)" returns "hello")
     * It advances the lexpos past the balanced pattern.
     * If the pattern is not balanced, it returns None, and the lexpos is advanced to the end of its range.
     */
    pub fn parseBalancedPattern(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> Option<StateBufferedLexer> {
        let startTok = self.lexer().getConsumeToken(lexpos);
        let startPos = *lexpos;

        if startTok.is_none() {
            unreachable!();
        }

        let startTok = startTok.unwrap();
        let startTokType = startTok.tokPos.tok;
        let endTokType: Token = match startTokType {
            Token::LParen => Token::RParen,
            Token::LBrace => Token::RBrace,
            Token::LBracket => Token::RBracket,
            _ => {
                unreachable!();
            }
        };

        let mut openParenNum: u32 = 0;
        loop {
            let candidate = self.lexer().getConsumeToken(lexpos)?;
            let candidate = candidate.tokPos.tok;
            if candidate == endTokType {
                if openParenNum > 0 {
                    openParenNum -= 1;
                } else {
                    break;
                }
            } else if candidate == startTokType {
                openParenNum += 1;
            } else {
                continue;
            }
        }
        let mut endPos = *lexpos;
        self.lexer().moveBack(&mut endPos, 2);

        return Some(BufferedLexer::makeProtectedRange(&startPos, &endPos));
    }
}
