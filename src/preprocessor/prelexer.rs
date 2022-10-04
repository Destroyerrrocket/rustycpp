//! Prelexer. Tokenizes very laxly.
use lazy_regex::{regex_captures, regex_find};

use crate::utils::structs::TokPos;
use logos::Logos;

use super::pretoken::{PreToken, PreTokenLexer, WhiteCom};

// Control chars [ \n\x0B\t\x0C]

#[derive(Debug)]
#[doc(hidden)]
pub struct PreLexer {
    enableHeader: u32,
    currentNonSpliced: String,
    current: String,
    diff: usize,
    line: u32,
    column: u32,
    lastNl: bool,
}

#[doc(hidden)]
impl PreLexer {
    pub fn new(content: String) -> Self {
        Self {
            enableHeader: 0,
            currentNonSpliced: content.clone(),
            current: content,
            diff: 0,
            line: 1,
            column: 1,
            lastNl: false,
        }
    }

    pub fn expectHeader(&mut self) {
        self.enableHeader += 1;
    }

    pub fn doNotExpectHeader(&mut self) {
        if self.enableHeader > 0 {
            self.enableHeader -= 1;
        }
    }

    fn spliceNewlinePosition(&self) -> Option<usize> {
        /* This is going to be the next nl found. IT DOES NOT DELIMIT TOKENS.
            if the next one is \n and the previous is a "\", it needs to be spliced.
            If we're unable to generate the token, or the token generated reaches
            the "\", then we splice and try again.
        */
        let mut maybe_remove: Option<usize> = None;
        match self.current.chars().position(|x: char| x == '\n') {
            None => return None,
            Some(salt_pos) => {
                if salt_pos > 0 && self.current.chars().nth(salt_pos - 1) == Some('\\') {
                    maybe_remove = Some(salt_pos - 1);
                }
            }
        }
        return maybe_remove;
    }

    fn getNextTokenNonSpliced(&mut self) -> (Option<PreToken>, usize) {
        if self.enableHeader > 0 {
            if let Some(res) = regex_find!(r#"^(?:<[^\n>]+>|"[^\n"]+")"#, &self.current) {
                return (Some(PreToken::HeaderName(res.to_string())), res.len());
            }
        }
        let mut lexer = PreTokenLexer::lexer(&self.current);
        if let Some(idxLex) = lexer.next() {
            let content = lexer.slice().to_string();
            let len = content.len();
            match idxLex {
                PreTokenLexer::RawStringLiteral => {
                    if let Some((_, key)) = regex_captures!(r#"R"(.*)\("#, &content) {
                        if let Some(position) =
                            self.current.find((")".to_owned() + key + "\"").as_str())
                        {
                            let additionalChars = if let Some(res) = regex_find!(
                                r#"^(?:[\\w_][\\w\\d_]*)"#,
                                &self.current[..position + key.len() + 2]
                            ) {
                                2 + res.len()
                            } else {
                                2
                            };
                            return (
                                Some(PreToken::RawStringLiteral(
                                    self.current[0..position + key.len() + additionalChars]
                                        .to_string(),
                                )),
                                position + key.len() + additionalChars,
                            );
                        }
                    }
                }
                PreTokenLexer::Error => {
                    let errContent = lexer.slice().to_string();
                    let len = errContent.len();
                    return (Some(PreToken::Unknown(errContent)), len);
                }
                _ => {
                    return (Some(PreToken::new(idxLex, content)), len);
                }
            }
        }
        return (None, 0);
    }

    fn applySplice(&mut self, splice_point: usize) {
        self.current.remove(splice_point);
        self.current.remove(splice_point);
    }

    fn getNextTokenData(&mut self) -> (Option<PreToken>, usize, usize) {
        let (mut kind, mut idx, mut splices) = (None, 0, 0);
        let prevCurrent = self.current.clone();
        loop {
            if self.current.is_empty() {
                break;
            }
            if regex_find!(r#"^<::[^:>]"#, &self.current).is_some() {
                (kind, idx) = (
                    Some(PreToken::new(
                        PreTokenLexer::OperatorPunctuator,
                        "<".to_string(),
                    )),
                    1,
                );
                break;
            }
            let splice_point_slash_nl = self.spliceNewlinePosition();
            (kind, idx) = self.getNextTokenNonSpliced();
            if splice_point_slash_nl.contains(&idx)
                || (matches!(kind, Some(PreToken::Unknown(_))) && splice_point_slash_nl.is_some())
            {
                self.applySplice(splice_point_slash_nl.unwrap());
                splices += 1;
                continue;
            } else if matches!(kind, Some(PreToken::Unknown(_))) {
                self.current = prevCurrent;
                splices = 0;
                let splice_point_slash_nl = self.spliceNewlinePosition();
                if splice_point_slash_nl.contains(&idx) {
                    self.applySplice(splice_point_slash_nl.unwrap());
                    splices += 1;
                }
                break;
            } else if kind.is_some() {
                break;
            }
            eprintln!(
                "Encountered unmachable preprocessing token at: {} {}",
                self.line, self.column
            );
            return (None, 0, 0);
        }
        return (kind, idx, splices);
    }

    fn doNext(&mut self) -> Option<TokPos<PreToken>> {
        let mut res: Option<TokPos<PreToken>> = None;
        let (kind, mut idx, splices) = self.getNextTokenData();

        if let Some(mut kind) = kind {
            if let PreToken::Whitespace(WhiteCom::Comment(comment)) = &mut kind {
                if comment.ends_with('\n') {
                    comment.pop();
                    idx = idx.checked_sub(1)?;
                }
            }

            let (mut lineEnd, mut columnEnd) = (self.line, self.column);
            {
                let (mut idxCpy, mut splicesCpy) = (idx as i64, splices as i64);
                for charGud in self.currentNonSpliced.chars() {
                    idxCpy -= 1;
                    columnEnd += 1;
                    if charGud == '\n' {
                        columnEnd = 1;
                        lineEnd += 1;
                        if splicesCpy > 0 {
                            splicesCpy -= 1;
                            idxCpy += 2;
                        }
                    }
                    if splicesCpy == 0 && idxCpy == 0 {
                        break;
                    }
                }
            }
            let mut originalString = &self.currentNonSpliced[0..idx + splices * 2];
            if originalString.ends_with("\\\n") {
                originalString = &self.currentNonSpliced[0..idx + splices * 2 - 2];
            }
            res = Some(TokPos {
                tok: kind,
                start: self.diff,
                end: self.diff + originalString.len(),
            });
            self.diff += idx + splices * 2;
            self.currentNonSpliced = self.currentNonSpliced[idx + splices * 2..].to_string();
            self.current = self.current[idx..].to_string();
            (self.line, self.column) = (lineEnd, columnEnd);
        }
        return res;
    }
}
impl Iterator for PreLexer {
    type Item = TokPos<PreToken>;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.doNext();
        if res.is_some_and(|x| matches!(x.tok, PreToken::Newline)) {
            self.lastNl = true;
            return res;
        }
        if res.is_some() {
            self.lastNl = false;
            return res;
        }
        if !self.lastNl {
            self.lastNl = true;
            return Some(TokPos {
                start: self.diff,
                tok: PreToken::Newline,
                end: self.diff + 1,
            });
        }
        return None;
    }
}
