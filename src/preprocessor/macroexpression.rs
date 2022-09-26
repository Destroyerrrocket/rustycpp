use std::collections::VecDeque;

use crate::utils::pretoken::PreToken;
use crate::utils::structs::{CompileError, CompileMsg, FilePreTokPos, PreTokPos};
use crate::{filePreTokPosMatchArm, filePreTokPosMatches};

use super::multilexer::MultiLexer;
use super::Preprocessor;

impl Preprocessor {
    fn getDefinedName(
        lexer: &mut MultiLexer,
        definedToken: &FilePreTokPos<PreToken>,
    ) -> Result<String, CompileMsg> {
        let mut name = String::new();
        // Skip whitespace. Push back the meta tokens. When we get the first open paren, wait.
        let mut metaToks = vec![];
        let openParenTok: FilePreTokPos<PreToken> = loop {
            if let Some(tok) = lexer.next() {
                match tok.tokPos.tok {
                    PreToken::Whitespace(_) => {}
                    PreToken::Ident(id) => {
                        // Shortcut! :D
                        return Ok(id);
                    }
                    PreToken::ValidNop | PreToken::EnableMacro(_) | PreToken::DisableMacro(_) => {
                        metaToks.push(tok);
                    }
                    PreToken::Newline => {
                        lexer.pushToken(tok.clone());
                        return Err(CompileError::from_preTo(
                            "Expected identifier after the defined. Instead, found a newline"
                                .to_string(),
                            definedToken,
                        ));
                    }
                    PreToken::OperatorPunctuator("(") => {
                        break tok;
                    }
                    _ => {
                        return Err(CompileError::from_preTo(
                            format!(
                                "Expected identifier after the defined. Instead, found: {}",
                                tok.tokPos.tok.to_str()
                            ),
                            definedToken,
                        ));
                    }
                }
            } else {
                return Err(CompileError::from_preTo(
                    "Expected identifier after the defined. Instead, found EOF",
                    definedToken,
                ));
            }
        };

        // Capture everything until the next matching close paren
        let mut openParens: usize = 0;

        let mut tokies: Vec<FilePreTokPos<PreToken>> = vec![];
        loop {
            let tok = lexer.next();
            match (openParens, &tok) {
                (_, Some(filePreTokPosMatchArm!(PreToken::Newline)) | None) => {
                    if let Some(tok) = tok {
                        lexer.pushToken(tok);
                    }
                    return Err(CompileError::from_preTo(
                        "Expected matching closing paren for this opening paren",
                        &openParenTok,
                    ));
                }
                (0, Some(filePreTokPosMatchArm!(PreToken::OperatorPunctuator(")")))) => {
                    break;
                }
                (_, Some(filePreTokPosMatchArm!(PreToken::OperatorPunctuator(")")))) => {
                    tokies.push(tok.unwrap());
                    openParens -= 1;
                }
                (_, Some(filePreTokPosMatchArm!(PreToken::OperatorPunctuator("(")))) => {
                    tokies.push(tok.unwrap());
                    openParens += 1;
                }
                _ => {
                    tokies.push(tok.unwrap());
                }
            };
        }

        // Cleaning all whitespace things, and then looking at first element for a name
        if let Some(tok) = tokies
            .iter()
            .filter(|x| {
                !filePreTokPosMatches!(
                    x,
                    PreToken::Whitespace(_)
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                        | PreToken::ValidNop
                )
            })
            .take(1)
            .filter(|x| filePreTokPosMatches!(x, PreToken::Ident(_)))
            .map(|x| x.tokPos.tok.to_str())
            .into_iter()
            .next()
        {
            name = tok.to_string();
        }

        if name.is_empty() {
            Err(CompileError::from_preTo(
                "Expected identifier after defined",
                definedToken,
            ))
        } else {
            Ok(name)
        }
    }

    pub fn consumeMacroExpr(&mut self) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut paramDisabledMacros = self.disabledMacros.clone();
        paramDisabledMacros.remove(&"__has_include".to_owned());
        paramDisabledMacros.remove(&"__has_cpp_attribute".to_owned());
        let mut preproTokie = VecDeque::new();
        while let Some(tok) = self.multilexer.next() {
            match &tok {
                filePreTokPosMatchArm!(PreToken::Newline) => {
                    break;
                }

                filePreTokPosMatchArm!(PreToken::EnableMacro(nameMacro)) => {
                    paramDisabledMacros.remove(nameMacro);
                    preproTokie.push_back(tok.clone());
                }
                filePreTokPosMatchArm!(PreToken::DisableMacro(nameMacro)) => {
                    paramDisabledMacros.insert(nameMacro.clone());
                    preproTokie.push_back(tok.clone());
                }
                filePreTokPosMatchArm!(PreToken::Ident(name)) => {
                    if name == "defined" {
                        let nameDefined = Self::getDefinedName(&mut self.multilexer, &tok);
                        match nameDefined {
                            Ok(nameDefined) => {
                                preproTokie.push_back(FilePreTokPos::new_meta_c(
                                    PreToken::PPNumber(
                                        if self.definitions.contains_key(&nameDefined) {
                                            "1"
                                        } else {
                                            "0"
                                        }
                                        .to_owned(),
                                    ),
                                    &tok,
                                ));
                            }
                            Err(err) => {
                                self.reachNl();
                                return Err(err);
                            }
                        }
                    } else {
                        let toks = Self::macroExpandInternal(
                            &self.definitions,
                            &paramDisabledMacros,
                            &mut self.multilexer,
                            tok,
                        );
                        match toks {
                            Ok(toks) => {
                                preproTokie.extend(toks);
                            }
                            Err(err) => {
                                self.reachNl();
                                return Err(err);
                            }
                        }
                    }
                }
                _ => {
                    preproTokie.push_back(tok);
                }
            }
        }
        Ok(preproTokie
            .into_iter()
            .filter(|x| {
                !filePreTokPosMatches!(
                    x,
                    PreToken::ValidNop
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                        | PreToken::Whitespace(_)
                        | PreToken::Newline
                )
            })
            .collect())
    }

    pub fn evalIfScope(sequence: VecDeque<FilePreTokPos<PreToken>>) -> bool {
        sequence.front().is_some_and(|x| {
            filePreTokPosMatches!(x, PreToken::PPNumber(_))
                && (x.tokPos.tok.to_str().parse::<i128>().is_ok_and(|x| *x != 0))
        })
    }
}
