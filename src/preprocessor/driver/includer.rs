#![allow(non_camel_case_types, clippy::string_to_string)]

use std::collections::{HashMap, VecDeque};

use crate::filePreTokPosMatches;
use crate::grammars::defineast::DefineAst;
use crate::preprocessor::prelexer::PreLexer;
use crate::preprocessor::pretoken::PreToken;
use crate::utils::structs::PreTokPos;
use crate::utils::structs::{CompileError, CompileMsg, FilePreTokPos};

use multiset::HashMultiSet;

use super::Preprocessor;
use crate::preprocessor::multilexer::MultiLexer;

impl Preprocessor {
    pub fn includeFile(
        &mut self,
        preToken: &FilePreTokPos<PreToken>,
        file: String,
    ) -> Result<(), CompileMsg> {
        if self.multilexer.hasFileAccess(&file) {
            self.multilexer.pushFile(file);
        } else {
            return Err(CompileError::from_preTo("", preToken));
        }
        Ok(())
    }

    pub fn consumeMacroInclude(
        &mut self,
        preToken: &FilePreTokPos<PreToken>,
    ) -> Result<String, CompileMsg> {
        let multilexer = &mut self.multilexer;
        let tokens = multilexer
            .take_while(|x| !filePreTokPosMatches!(x, PreToken::Newline))
            .collect::<VecDeque<_>>();
        Self::tokensToValidIncludeablePath(
            &self.multilexer,
            &self.definitions,
            &self.disabledMacros,
            preToken,
            tokens,
        )
    }

    pub fn tokensToValidIncludeablePath(
        lexer: &MultiLexer,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashMultiSet<String>,
        preToken: &FilePreTokPos<PreToken>,
        tokensInclude: VecDeque<FilePreTokPos<PreToken>>,
    ) -> Result<String, CompileMsg> {
        let mut path = String::new();

        if tokensInclude.is_empty() {
            return Err(CompileError::from_preTo(
                "The empty path can't be opened",
                preToken,
            ));
        }

        if let Some(newPath) = Self::checkForInclude(tokensInclude.clone()) {
            path = newPath;
        }

        let mut paramLexer = MultiLexer::new_def(lexer.fileMapping());
        paramLexer.pushTokensDec(tokensInclude);
        let toks = Self::expandASequenceOfTokens(paramLexer, definitions, disabledMacros)?;

        if let Some(newPath) = Self::checkForInclude(toks.clone()) {
            path = newPath;
        } else {
            for s in toks.into_iter().map(|t| t.tokPos.tok.to_str().to_owned()) {
                path.push_str(&s);
            }
        }

        Ok(path)
    }

    fn checkForInclude(mut toks: VecDeque<FilePreTokPos<PreToken>>) -> Option<String> {
        let mut res = String::new();

        while toks.front().is_some_and(|x| {
            filePreTokPosMatches!(
                x,
                PreToken::Whitespace(_)
                    | PreToken::Newline
                    | PreToken::ValidNop
                    | PreToken::EnableMacro(_)
                    | PreToken::DisableMacro(_)
            )
        }) {
            toks.pop_front();
        }

        for s in toks.iter().map(|x| x.tokPos.tok.to_str().to_owned()) {
            res.push_str(&s);
        }
        let mut lexer = PreLexer::new(res);
        lexer.expectHeader();
        if let Some(PreToken::HeaderName(pathWithSurroundingChars)) = lexer.next().map(|x| x.tok) {
            let mut chars = pathWithSurroundingChars.chars();
            chars.next();
            chars.next_back();
            Some(chars.as_str().to_owned())
        } else {
            None
        }
    }
}
