//! handle the #include directive

#![allow(non_camel_case_types, clippy::string_to_string)]

use std::collections::{HashMap, VecDeque};

use crate::grammars::defineast::DefineAst;
use crate::preprocessor::prelexer::PreLexer;
use crate::preprocessor::pretoken::PreToken;
use crate::utils::compilerstate::CompilerState;
use crate::utils::structs::TokPos;
use crate::utils::structs::{CompileError, CompileMsg, CompileMsgImpl, FileTokPos};
use crate::{fileTokPosMatchArm, fileTokPosMatches};

use multiset::HashMultiSet;

use super::Preprocessor;
use crate::preprocessor::multilexer::MultiLexer;

impl Preprocessor {
    /// Include a file in the current position of the preprocessor
    pub fn includeFile(
        &mut self,
        preToken: &FileTokPos<PreToken>,
        file: &str,
    ) -> Result<(), CompileMsg> {
        if self.multilexer.hasFileAccess(file) {
            self.multilexer.pushFile(file);
        } else {
            return Err(CompileError::fromPreTo(
                format!("Can't include the file in path: {file}"),
                preToken,
            ));
        }
        Ok(())
    }

    /// Evaluates the #include directive. Finds a candidate and returns the file path
    pub fn consumeMacroInclude(
        &mut self,
        preToken: &FileTokPos<PreToken>,
    ) -> Result<String, CompileMsg> {
        let multilexer = &mut self.multilexer;
        let tokens = multilexer
            .take_while(|x| !fileTokPosMatches!(x, PreToken::Newline))
            .collect::<VecDeque<_>>();
        Self::tokensToValidIncludeablePath(
            &self.compilerState,
            &self.multilexer,
            &self.definitions,
            &self.disabledMacros,
            preToken,
            tokens,
        )
    }

    /// Expands the tokens if necessary, and returns the path found, if any
    pub fn tokensToValidIncludeablePath(
        compilerState: &CompilerState,
        lexer: &MultiLexer,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashMultiSet<String>,
        preToken: &FileTokPos<PreToken>,
        tokensInclude: VecDeque<FileTokPos<PreToken>>,
    ) -> Result<String, CompileMsg> {
        let mut path = String::new();

        if tokensInclude.is_empty() {
            return Err(CompileError::fromPreTo(
                "The empty path can't be opened",
                preToken,
            ));
        }

        if let Some(newPath) = Self::checkForInclude(&tokensInclude) {
            path = newPath;
        }

        let mut paramLexer = MultiLexer::new_def(lexer.fileMapping());
        paramLexer.pushTokensDec(tokensInclude);
        let toks =
            Self::expandASequenceOfTokens(compilerState, paramLexer, definitions, disabledMacros)?;

        if let Some(newPath) = Self::checkForInclude(&toks) {
            path = newPath;
        } else {
            for s in toks.into_iter().map(|t| t.tokPos.tok.to_str().to_owned()) {
                path.push_str(&s);
            }
        }

        Ok(path)
    }

    /// Is the current tokens a valid include path token? Re-lexes them if necessary
    pub fn checkForInclude(toks: &VecDeque<FileTokPos<PreToken>>) -> Option<String> {
        let mut res = String::new();

        let mut iter = toks
            .iter()
            .skip_while(|x| {
                fileTokPosMatches!(
                    x,
                    PreToken::Whitespace(_)
                        | PreToken::Newline
                        | PreToken::ValidNop
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                )
            })
            .peekable();

        let nextTok = iter.peek()?;

        if let fileTokPosMatchArm!(PreToken::HeaderName(pathWithSurroundingChars)) = nextTok {
            let mut chars = pathWithSurroundingChars.chars();
            chars.next();
            chars.next_back();
            return Some(chars.as_str().to_owned());
        } else if {
            fileTokPosMatches!(
                nextTok,
                PreToken::StringLiteral(_)
                    | PreToken::OperatorPunctuator("<" | "<=" | "<<" | "<<=")
                    | PreToken::UdStringLiteral(_)
            )
        } {
            for s in iter.map(|x| x.tokPos.tok.to_str().to_owned()) {
                res.push_str(&s);
            }
            let mut lexer = PreLexer::new(res);
            lexer.expectHeader();
            if let Some(PreToken::HeaderName(pathWithSurroundingChars)) =
                lexer.next().map(|x| x.tok)
            {
                let mut chars = pathWithSurroundingChars.chars();
                chars.next();
                chars.next_back();
                return Some(chars.as_str().to_owned());
            }
        }
        None
    }
}
