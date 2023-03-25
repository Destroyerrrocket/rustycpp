//! Macro constant integer expression evaluation
use std::collections::VecDeque;

use crate::grammars::macrointconstantexpressionast::PreTokenIf;
use crate::grammars::macrointconstantexpressionparser;
use crate::preprocessor::multilexer::MultiLexer;
use crate::preprocessor::pretoken::PreToken;
use crate::utils::structs::{CompileError, CompileMsg, CompileMsgImpl, FileTokPos, TokPos};
use crate::{fileTokPosMatchArm, fileTokPosMatches};

use super::Preprocessor;

impl Preprocessor {
    /// Finds the macro name in a `defined` clause. It is treated separatedly
    /// because it can be either:
    ///
    /// `defined(MACRO)` or `defined MACRO`
    ///
    /// as such, it is not a viable macro definition. Given that we want to
    /// prevent macro expansions in the macro name, it can't be left to the
    /// parser, as at that point the macros are all expanded.
    fn getDefinedName(
        lexer: &mut MultiLexer,
        definedToken: &FileTokPos<PreToken>,
    ) -> Result<String, CompileMsg> {
        let mut name = String::new();
        // Skip whitespace. When we get the first open paren, wait.
        let openParenTok: FileTokPos<PreToken> = loop {
            if let Some(tok) = lexer.next() {
                match tok.tokPos.tok {
                    PreToken::ValidNop
                    | PreToken::EnableMacro(_)
                    | PreToken::DisableMacro(_)
                    | PreToken::Whitespace(_) => {}
                    PreToken::Ident(id) => {
                        // Shortcut! :D
                        return Ok(id);
                    }
                    PreToken::Newline => {
                        lexer.pushToken(tok.clone());
                        return Err(CompileError::fromPreTo(
                            "Expected identifier after the defined. Instead, found a newline"
                                .to_string(),
                            definedToken,
                        ));
                    }
                    PreToken::OperatorPunctuator("(") => {
                        break tok;
                    }
                    _ => {
                        return Err(CompileError::fromPreTo(
                            format!(
                                "Expected identifier after the defined. Instead, found: {}",
                                tok.tokPos.tok.to_str()
                            ),
                            definedToken,
                        ));
                    }
                }
            } else {
                return Err(CompileError::fromPreTo(
                    "Expected identifier after the defined. Instead, found EOF",
                    definedToken,
                ));
            }
        };

        // Capture everything until the next matching close paren
        let mut openParens: usize = 0;

        let mut tokies: Vec<FileTokPos<PreToken>> = vec![];
        loop {
            let tok = lexer.next();
            match (openParens, &tok) {
                (_, Some(fileTokPosMatchArm!(PreToken::Newline)) | None) => {
                    if let Some(tok) = tok {
                        lexer.pushToken(tok);
                    }
                    return Err(CompileError::fromPreTo(
                        "Expected matching closing paren for this opening paren",
                        &openParenTok,
                    ));
                }
                (0, Some(fileTokPosMatchArm!(PreToken::OperatorPunctuator(")")))) => {
                    break;
                }
                (_, Some(fileTokPosMatchArm!(PreToken::OperatorPunctuator(")")))) => {
                    tokies.push(tok.unwrap());
                    openParens -= 1;
                }
                (_, Some(fileTokPosMatchArm!(PreToken::OperatorPunctuator("(")))) => {
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
                !fileTokPosMatches!(
                    x,
                    PreToken::Whitespace(_)
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                        | PreToken::ValidNop
                )
            })
            .take(1)
            .filter(|x| fileTokPosMatches!(x, PreToken::Ident(_)))
            .map(|x| x.tokPos.tok.to_str())
            .next()
        {
            name = tok.to_string();
        }

        if name.is_empty() {
            Err(CompileError::fromPreTo(
                "Expected identifier after defined",
                definedToken,
            ))
        } else {
            Ok(name)
        }
    }

    /// Consumes all the tokens until the next newline, and returns the exanded
    /// version of them. Takes special care of the `defined` operator. It also
    /// makes some minor transformations to handle char literals.
    pub fn consumeMacroExpr(&mut self) -> Result<VecDeque<FileTokPos<PreToken>>, CompileMsg> {
        let mut paramDisabledMacros = self.disabledMacros.clone();
        paramDisabledMacros.remove(&"__has_include".to_owned());
        paramDisabledMacros.remove(&"__has_cpp_attribute".to_owned());
        let mut preproTokie = VecDeque::new();
        while let Some(tok) = self.multilexer.next() {
            match &tok {
                fileTokPosMatchArm!(PreToken::Newline) => {
                    break;
                }

                fileTokPosMatchArm!(PreToken::EnableMacro(nameMacro)) => {
                    paramDisabledMacros.remove(nameMacro);
                    preproTokie.push_back(tok.clone());
                }
                fileTokPosMatchArm!(PreToken::DisableMacro(nameMacro)) => {
                    paramDisabledMacros.insert(nameMacro.clone());
                    preproTokie.push_back(tok.clone());
                }
                fileTokPosMatchArm!(PreToken::Ident(name)) => {
                    if name == "defined" {
                        let nameDefined = Self::getDefinedName(&mut self.multilexer, &tok);
                        match nameDefined {
                            Ok(nameDefined) => {
                                preproTokie.push_back(FileTokPos::new_meta_c(
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
                            &self.compilerState,
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
                !fileTokPosMatches!(
                    x,
                    PreToken::ValidNop
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                        | PreToken::Whitespace(_)
                        | PreToken::Newline
                )
            })
            .filter_map(|x| match x {
                fileTokPosMatchArm!(
                    PreToken::ValidNop
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                        | PreToken::Whitespace(_)
                        | PreToken::Newline
                ) => None,
                fileTokPosMatchArm!(PreToken::CharLiteral(ref char)) => {
                    // TODO: THIS IS NOT CORRECT. We need to evaluate the escape sequences! I'm ignoring this for now
                    let mut chars = char.chars();
                    chars.next();
                    let mut buffer = [0; 4];
                    chars.next().unwrap_or('\0').encode_utf8(&mut buffer);
                    Some(FileTokPos::new_meta_c(
                        PreToken::PPNumber(buffer[0].to_string()),
                        &x,
                    ))
                }
                fileTokPosMatchArm!(PreToken::Keyword(key)) if key == "true" => Some(
                    FileTokPos::new_meta_c(PreToken::PPNumber("1".to_owned()), &x),
                ),
                fileTokPosMatchArm!(PreToken::Keyword(_) | PreToken::Ident(_)) => Some(
                    FileTokPos::new_meta_c(PreToken::PPNumber("0".to_owned()), &x),
                ),
                _ => Some(x),
            })
            .collect())
    }

    /// Transforms preprocessor tokens to tokens for [`PreTokenIf`] for the If evaluator
    fn transformToParserTokens(
        sequence: &VecDeque<FileTokPos<PreToken>>,
        token: &FileTokPos<PreToken>,
    ) -> Result<VecDeque<FileTokPos<PreTokenIf>>, Vec<CompileMsg>> {
        let mut errors = vec![];
        for invalid in sequence.iter().filter(|x| {
            !fileTokPosMatches!(x, PreToken::PPNumber(_) | PreToken::OperatorPunctuator(_))
                || fileTokPosMatches!(
                    x,
                    PreToken::OperatorPunctuator(
                        r"{" | r"}"
                            | r"["
                            | r"]"
                            | r"<:"
                            | r":>"
                            | r"<%"
                            | r"%>"
                            | r";"
                            | r"..."
                            | r"::"
                            | r"."
                            | r".*"
                            | r"->"
                            | r"->*"
                            | r"="
                            | r"+="
                            | r"-="
                            | r"*="
                            | r"/="
                            | r"%="
                            | r"^="
                            | r"&="
                            | r"|="
                            | r"<<="
                            | r">>="
                            | r"and_eq"
                            | r"or_eq"
                            | r"xor_eq"
                            | r"not_eq"
                    )
                )
        }) {
            errors.push(CompileError::fromPreTo(
                format!(
                    "Invalid token in if eval scope: {}",
                    invalid.tokPos.tok.to_str()
                ),
                invalid,
            ));
        }

        // TODO: THIS IS NOT CORRECT. We should evaluate the pretocens to full tokens, but I'll wait for the lexer to be done
        let mut intconstantValues: VecDeque<FileTokPos<_>> = VecDeque::new();
        for token in sequence {
            match token {
                fileTokPosMatchArm!(PreToken::PPNumber(ref num)) => {
                    let mut numClone = num.clone();
                    numClone.retain(char::is_numeric);
                    match numClone.parse::<i128>() {
                        Ok(num) => {
                            intconstantValues
                                .push_back(FileTokPos::new_meta_c(PreTokenIf::Num(num), token));
                        }
                        Err(err) => errors.push(CompileError::fromPreTo(
                            format!("Invalid number in if eval scope: {err}"),
                            token,
                        )),
                    }
                }
                fileTokPosMatchArm!(PreToken::OperatorPunctuator(s)) => {
                    intconstantValues.push_back(FileTokPos::new_meta_c(
                        PreTokenIf::stringToPreTokenIfOperator(s),
                        token,
                    ));
                }
                _ => {}
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        if intconstantValues.front().is_none() {
            return Err(vec![CompileError::fromPreTo(
                "Missing data for integer constant evaluation",
                token,
            )]);
        }
        Ok(intconstantValues)
    }

    /// Evaluates an if statement expression, returning the evaluation result. Does not alter the state of the preprocessor
    pub fn evalIfScope(
        sequence: &VecDeque<FileTokPos<PreToken>>,
        token: &FileTokPos<PreToken>,
    ) -> Result<(bool, Vec<CompileMsg>), Vec<CompileMsg>> {
        let mut numSequence = Self::transformToParserTokens(sequence, token)?;
        let last = numSequence.back().unwrap().clone();
        let (n, err) = macrointconstantexpressionparser::exprRes((&mut numSequence, &last))?;
        Ok((n != 0, err))
    }
}
