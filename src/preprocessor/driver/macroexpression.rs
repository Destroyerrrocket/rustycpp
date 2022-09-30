//! Macro constant integer expression evaluation
use std::collections::VecDeque;

use crate::grammars::generated::macrointconstantexpressionastparser;
use crate::grammars::macrointconstantexpressionast::{PreTokenIf, VisitorEvaluator};
use crate::preprocessor::multilexer::MultiLexer;
use crate::preprocessor::pretoken::PreToken;
use crate::utils::antlrlexerwrapper::AntlrLexerWrapper;
use crate::utils::structs::{CompileError, CompileMsg, FilePreTokPos, PreTokPos};
use crate::{filePreTokPosMatchArm, filePreTokPosMatches};
use antlr_rust::common_token_stream::CommonTokenStream;

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

    /// Consumes all the tokens until the next newline, and returns the exanded
    /// version of them. Takes special care of the `defined` operator. It also
    /// makes some minor transformations to handle char literals.
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
            .filter_map(|x| match x {
                filePreTokPosMatchArm!(
                    PreToken::ValidNop
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                        | PreToken::Whitespace(_)
                        | PreToken::Newline
                ) => None,
                filePreTokPosMatchArm!(PreToken::CharLiteral(ref char)) => {
                    // TODO: THIS IS NOT CORRECT. We need to evaluate the escape sequences! I'm ignoring this for now
                    let mut chars = char.chars();
                    chars.next();
                    let mut buffer = [0; 4];
                    chars.next().unwrap_or('\0').encode_utf8(&mut buffer);
                    Some(FilePreTokPos::new_meta_c(
                        PreToken::PPNumber(buffer[0].to_string()),
                        &x,
                    ))
                }
                filePreTokPosMatchArm!(PreToken::Keyword(key)) if key == "true" => Some(
                    FilePreTokPos::new_meta_c(PreToken::PPNumber("1".to_owned()), &x),
                ),
                filePreTokPosMatchArm!(PreToken::Keyword(_) | PreToken::Ident(_)) => Some(
                    FilePreTokPos::new_meta_c(PreToken::PPNumber("0".to_owned()), &x),
                ),
                _ => Some(x),
            })
            .collect())
    }

    /// Transforms preprocessor tokens to tokens for [`PreTokenIf`] for the If evaluator
    fn transformToParserTokens(
        sequence: &VecDeque<FilePreTokPos<PreToken>>,
        token: &FilePreTokPos<PreToken>,
    ) -> Result<VecDeque<FilePreTokPos<PreTokenIf>>, Vec<CompileMsg>> {
        let mut errors = vec![];
        for invalid in sequence.iter().filter(|x| {
            !filePreTokPosMatches!(x, PreToken::PPNumber(_) | PreToken::OperatorPunctuator(_))
                || filePreTokPosMatches!(
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
            errors.push(CompileError::from_preTo(
                format!(
                    "Invalid token in if eval scope: {}",
                    invalid.tokPos.tok.to_str()
                ),
                invalid,
            ));
        }

        // TODO: THIS IS NOT CORRECT. We should evaluate the pretocens to full tokens, but I'll wait for the lexer to be done
        let mut intconstantValues: VecDeque<FilePreTokPos<_>> = VecDeque::new();
        for token in sequence {
            match token {
                filePreTokPosMatchArm!(PreToken::PPNumber(ref num)) => {
                    let mut numClone = num.clone();
                    numClone.retain(char::is_numeric);
                    match numClone.parse::<i128>() {
                        Ok(num) => {
                            intconstantValues
                                .push_back(FilePreTokPos::new_meta_c(PreTokenIf::Num(num), token));
                        }
                        Err(err) => errors.push(CompileError::from_preTo(
                            format!("Invalid number in if eval scope: {}", err),
                            token,
                        )),
                    }
                }
                filePreTokPosMatchArm!(PreToken::OperatorPunctuator(s)) => {
                    intconstantValues.push_back(FilePreTokPos::new_meta_c(
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
            return Err(vec![CompileError::from_preTo(
                "Missing data for integer constant evaluation",
                token,
            )]);
        }
        return Ok(intconstantValues);
    }

    /// Evaluates an if statement expression, returning the evaluation result. Does not alter the state of the preprocessor
    pub fn evalIfScope(
        sequence: VecDeque<FilePreTokPos<PreToken>>,
        token: &FilePreTokPos<PreToken>,
    ) -> Result<bool, Vec<CompileMsg>> {
        let numSequence = Self::transformToParserTokens(&sequence, token)?;
        let tokenStream = CommonTokenStream::new(AntlrLexerWrapper::new(
            numSequence,
            token.file.path().clone(),
        ));
        let mut basicParser =
            macrointconstantexpressionastparser::macrointconstantexpressionast::new(tokenStream);
        let tree = basicParser.exprRes().unwrap();
        let mut visitor = VisitorEvaluator::new();
        visitor.visit_start(&tree);
        return Ok(visitor.res() != 0);
    }
}
