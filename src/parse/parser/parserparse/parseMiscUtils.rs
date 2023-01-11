use crate::{
    lex::token::Token,
    parse::bufferedLexer::{BufferedLexer, StateBufferedLexer},
};

use super::super::Parser;

impl Parser {
    /** Balances a pattern starting with either a '(', '{' or '[' (otherwise, UB), and ending with the corresponding closing character.
     * It returns a range containing the contents inside the pattern, without the outer characters (aka, "(hello)" returns "hello")
     * It advances the lexpos past the balanced pattern.
     * If the pattern is not balanced, it returns None, and the lexpos is advanced to the end of its range or to the character that broke the balance
     * (aka: "(hello]", would break at the ']'. for "[(hello]", we will try and recover by returning the range "(hello")
     */
    pub fn parseAlmostBalancedPattern(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> Option<StateBufferedLexer> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum BalancedPattern {
            Paren,
            Brace,
            Bracket,
        }
        let getMatchingBalancedPattern = |tok| match tok {
            Token::LParen => Some((false, BalancedPattern::Paren)),
            Token::LBrace => Some((false, BalancedPattern::Brace)),
            Token::LBracket => Some((false, BalancedPattern::Bracket)),
            Token::RParen => Some((true, BalancedPattern::Paren)),
            Token::RBrace => Some((true, BalancedPattern::Brace)),
            Token::RBracket => Some((true, BalancedPattern::Bracket)),
            _ => None,
        };

        let startTok = self.lexer().getConsumeToken(lexpos);
        let startPos = *lexpos;

        if startTok.is_none() {
            unreachable!();
        }
        let startTok = startTok.unwrap();
        let startTokType = startTok.tokPos.tok;
        let Some((false, startBalancedPattern)) = getMatchingBalancedPattern(startTokType) else {
            unreachable!();
        };
        let mut stack = vec![startBalancedPattern];

        loop {
            let candidate = self.lexer().getConsumeToken(lexpos)?;
            let Some((isClosing, kind)) = getMatchingBalancedPattern(candidate.tokPos.tok) else {
                continue;
            };
            if isClosing {
                if let Some((i, _)) = stack.iter().enumerate().rev().find(|(_, t)| **t == kind) {
                    stack.resize(i, BalancedPattern::Brace);
                    if stack.is_empty() {
                        break;
                    }
                } else {
                    // We have a completely mismatched pattern, no way to recover. We'll backtrack one position, and let the parser continue from there.
                    self.lexer().moveBack(lexpos, 1);
                    return None;
                }
            } else {
                stack.push(kind);
            }
        }
        let mut endPos = *lexpos;
        self.lexer().moveBack(&mut endPos, 2);

        Some(BufferedLexer::makeProtectedRange(&startPos, &endPos))
    }
}
