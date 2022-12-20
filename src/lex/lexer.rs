//! Transforms `PreToken`s into `Token`s.
use std::collections::VecDeque;

use crate::fileTokPosMatches;
use crate::preprocessor::Preprocessor;
use crate::utils::structs::{CompileError, CompileMsg, FileTokPos, TokPos};

use super::token::{EncodingPrefix, Token};

/// Transforms `PreToken`s into `Token`s.
pub struct Lexer {
    /// The preprocessor that will be used to get the `PreToken`s
    preprocessor: Preprocessor,
    /// The last tokens generated
    lastTokens: VecDeque<FileTokPos<Token>>,
    /// Errors generated
    errors: Vec<CompileMsg>,

    greaterInLastToken: bool,
}

impl Lexer {
    /// Creates a new `Lexer`
    pub fn new(preprocessor: Preprocessor) -> Self {
        Self {
            preprocessor,
            lastTokens: VecDeque::new(),
            errors: vec![],
            greaterInLastToken: false,
        }
    }

    /// Returns the resulting preffix
    fn calcPrefix(
        e1: EncodingPrefix,
        e2: EncodingPrefix,
        file: u64,
        at: usize,
        atEnd: usize,
    ) -> Result<EncodingPrefix, CompileMsg> {
        match (e1, e2) {
            (EncodingPrefix::None, EncodingPrefix::None) => Ok(EncodingPrefix::None),

            (EncodingPrefix::u8 | EncodingPrefix::None, EncodingPrefix::u8)
            | (EncodingPrefix::u8, EncodingPrefix::None) => Ok(EncodingPrefix::u8),

            (EncodingPrefix::None | EncodingPrefix::u, EncodingPrefix::u)
            | (EncodingPrefix::u, EncodingPrefix::None) => Ok(EncodingPrefix::u),

            (EncodingPrefix::U | EncodingPrefix::None, EncodingPrefix::U)
            | (EncodingPrefix::U, EncodingPrefix::None) => Ok(EncodingPrefix::U),

            (EncodingPrefix::None | EncodingPrefix::L, EncodingPrefix::L)
            | (EncodingPrefix::L, EncodingPrefix::None) => Ok(EncodingPrefix::L),
            (e1, e2) => Err(CompileError::from_at(
                format!("Incompatible encoding prefixes between '{e1}' and '{e2}'"),
                file,
                at,
                Some(atEnd),
            )),
        }
    }

    /// Merge the two string literals
    fn doMergeStringLiterals(
        tok1: FileTokPos<Token>,
        tok2: FileTokPos<Token>,
    ) -> Result<FileTokPos<Token>, CompileMsg> {
        let (start, end) = if tok1.file == tok2.file {
            (tok1.tokPos.start, tok2.tokPos.end)
        } else {
            (tok1.tokPos.start, tok1.tokPos.end)
        };
        let file = tok1.file;
        let (tok1, tok2) = (tok1.tokPos.tok, tok2.tokPos.tok);
        match (tok1, tok2) {
            (Token::StringLiteral(enc1, text1), Token::StringLiteral(enc2, text2)) => {
                let prefix = Self::calcPrefix(enc1, enc2, file.clone(), start, end)?;
                return Ok(FileTokPos::new(
                    file,
                    TokPos {
                        start,
                        tok: Token::StringLiteral(prefix, text1 + &text2),
                        end,
                    },
                ));
            }
            (
                Token::UdStringLiteral(enc1, text1, ud1),
                Token::UdStringLiteral(enc2, text2, ud2),
            ) => {
                let prefix = Self::calcPrefix(enc1, enc2, file.clone(), start, end)?;
                return if ud1 == ud2 {
                    Ok(FileTokPos::new(
                        file,
                        TokPos {
                            start,
                            tok: Token::UdStringLiteral(prefix, text1 + &text2, ud1),
                            end,
                        },
                    ))
                } else {
                    Err(CompileError::from_at(
                        format!(
                            "Incompatible user defined string literals between '{ud1}' and '{ud2}'"
                        ),
                        file,
                        start,
                        Some(end),
                    ))
                };
            }
            (Token::StringLiteral(enc1, text1), Token::UdStringLiteral(enc2, text2, ud))
            | (Token::UdStringLiteral(enc1, text1, ud), Token::StringLiteral(enc2, text2)) => {
                let prefix = Self::calcPrefix(enc1, enc2, file.clone(), start, end)?;
                return Ok(FileTokPos::new(
                    file,
                    TokPos {
                        start,
                        tok: Token::UdStringLiteral(prefix, text1 + &text2, ud),
                        end,
                    },
                ));
            }
            _ => unreachable!(),
        }
    }

    /// Merges all the string literals buffered. In case of error, will still generate the last valid string literal.
    fn mergeStringLiterals(&mut self) {
        while self.lastTokens.len() >= 2
            && fileTokPosMatches!(
                self.lastTokens.front().unwrap(),
                Token::StringLiteral(_, _) | Token::UdStringLiteral(_, _, _)
            )
            && fileTokPosMatches!(
                self.lastTokens.get(1).unwrap(),
                Token::StringLiteral(_, _) | Token::UdStringLiteral(_, _, _)
            )
        {
            let tok1 = self.lastTokens.pop_front().unwrap();
            let tok2 = self.lastTokens.pop_front().unwrap();
            if let Err(err) = Self::doMergeStringLiterals(tok1, tok2.clone())
                .map(|x| self.lastTokens.push_front(x))
            {
                self.errors.push(err);
                self.lastTokens.push_front(tok2);
            }
        }
    }

    /// Generates the next token, performing concatenation of string literals.
    fn generateNextToken(&mut self) -> bool {
        while self.lastTokens.is_empty()
            || fileTokPosMatches!(
                self.lastTokens.back().unwrap(),
                Token::StringLiteral(_, _) | Token::UdStringLiteral(_, _, _)
            )
        {
            match self.preprocessor.next() {
                None => break,
                Some(Err(err)) => self.errors.push(err),
                Some(Ok(preTok)) => {
                    match Token::from_preToken(preTok).map(|x| self.lastTokens.extend(x)) {
                        Err(None) => {
                            self.greaterInLastToken = false;
                        }
                        Ok(_) => {
                            if self.greaterInLastToken {
                                for tok in &mut self.lastTokens {
                                    if matches!(
                                        tok.tokPos.tok,
                                        Token::SingleGreater | Token::FirstGreater
                                    ) {
                                        tok.tokPos.tok = Token::SecondGreater;
                                    } else if matches!(tok.tokPos.tok, Token::Equal) {
                                        tok.tokPos.tok = Token::StrippedGreaterEqual;
                                    }
                                }
                            }
                            self.greaterInLastToken = fileTokPosMatches!(
                                self.lastTokens.back().unwrap(),
                                Token::SecondGreater | Token::SingleGreater
                            );
                        }
                        Err(Some(err)) => self.errors.push(err),
                    }
                }
            }
        }

        if self.lastTokens.is_empty() {
            return false;
        }

        if fileTokPosMatches!(
            self.lastTokens.front().unwrap(),
            Token::StringLiteral(_, _) | Token::UdStringLiteral(_, _, _)
        ) {
            self.mergeStringLiterals();
        }

        return true;
    }

    /// Returns the errors of this [`Lexer`].
    pub fn errors(&mut self) -> Vec<CompileMsg> {
        self.errors.drain(..).collect()
    }
}

impl Iterator for Lexer {
    type Item = FileTokPos<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.generateNextToken() {
            self.lastTokens.pop_front()
        } else {
            None
        }
    }
}
