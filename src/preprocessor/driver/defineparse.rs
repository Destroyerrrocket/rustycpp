use lalrpop_util::ParseError;

use crate::{
    filePreTokPosMatchArm,
    grammars::{
        define,
        defineast::{DefineAst, IsVariadic, PreTokenDefine, PreTokenDefinePreParse},
    },
    preprocessor::pretoken::{PreToken, PreprocessingOperator},
    utils::{
        funcs::all_unique_elements,
        lalrpoplexerwrapper::LalrPopLexerWrapper,
        structs::{CompileError, CompileMsg, CompileWarning, FilePreTokPos, PreTokPos},
    },
};

use super::Preprocessor;

impl Preprocessor {
    fn parseReplList(
        parse: &DefineAst,
        tokens: Vec<FilePreTokPos<PreToken>>,
    ) -> Result<Vec<PreTokenDefine>, CompileMsg> {
        let variadicStr = if let IsVariadic::True(stri) = &parse.variadic {
            stri.as_str()
        } else {
            ""
        };
        let mut vaOptExpectParen: Vec<i32> = vec![];
        let mut toksPre: Vec<FilePreTokPos<PreTokenDefinePreParse>> = tokens
            .into_iter()
            .map(|tok| FilePreTokPos {
                file: tok.file,
                tokPos: PreTokPos::<PreTokenDefinePreParse> {
                    start: tok.tokPos.start,
                    tok: match tok.tokPos.tok {
                        PreToken::Ident(s) => {
                            if parse.param.is_some_and(|param| param.contains(&s)) {
                                PreTokenDefinePreParse::Arg(s)
                            } else if s.as_str() == variadicStr || s.as_str() == "__VA_ARGS__" {
                                PreTokenDefinePreParse::VariadicArg
                            } else if s.as_str() == "__VA_OPT__" {
                                vaOptExpectParen.push(-1);
                                PreTokenDefinePreParse::VariadicOpt
                            } else {
                                PreTokenDefinePreParse::Normal(PreToken::Ident(s))
                            }
                        }
                        PreToken::OperatorPunctuator("(") => {
                            if let Some(pars) = vaOptExpectParen.last_mut() {
                                *pars += 1;
                                if *pars == 0 {
                                    PreTokenDefinePreParse::VariadicOptParenL
                                } else {
                                    PreTokenDefinePreParse::Normal(tok.tokPos.tok)
                                }
                            } else {
                                PreTokenDefinePreParse::Normal(tok.tokPos.tok)
                            }
                        }

                        PreToken::OperatorPunctuator(")") => {
                            let shouldMut = vaOptExpectParen.last_mut().map_or(false, |pars| {
                                *pars -= 1;
                                *pars == -1
                            });
                            if shouldMut {
                                vaOptExpectParen.pop();
                                PreTokenDefinePreParse::VariadicOptParenR
                            } else {
                                PreTokenDefinePreParse::Normal(tok.tokPos.tok)
                            }
                        }

                        PreToken::PreprocessingOperator(op) => {
                            if op == PreprocessingOperator::Hash {
                                PreTokenDefinePreParse::Hash
                            } else {
                                PreTokenDefinePreParse::HashHash
                            }
                        }
                        _ => PreTokenDefinePreParse::Normal(tok.tokPos.tok),
                    },
                    end: tok.tokPos.end,
                },
            })
            .collect();

        let mut inHashHash = false;

        toksPre = toksPre
            .into_iter()
            .rev()
            .filter(|x| match x.tokPos.tok {
                PreTokenDefinePreParse::Normal(PreToken::Whitespace(_)) => !inHashHash,
                PreTokenDefinePreParse::HashHash => {
                    inHashHash = true;
                    true
                }
                _ => {
                    inHashHash = false;
                    true
                }
            })
            .collect::<Vec<FilePreTokPos<PreTokenDefinePreParse>>>()
            .into_iter()
            .rev()
            .collect::<Vec<FilePreTokPos<PreTokenDefinePreParse>>>();

        let lexer = LalrPopLexerWrapper::new(toksPre.as_slice());
        let res = define::DefineStmtParser::new().parse(lexer);
        return res.map_err(|err| match err {
            ParseError::ExtraToken { token } => CompileError::from_at(
                format!(
                    "Found token {:?} when I wasn't expecting any other tokens",
                    token.1
                ),
                (token.0).1.clone(),
                (token.0).0,
                Some((token.2).0),
            ),
            ParseError::InvalidToken { location } => CompileError::from_at(
                "Found invalid token".to_string(),
                (location.1).clone(),
                location.0,
                None,
            ),
            ParseError::UnrecognizedEOF { location, expected } => CompileError::from_at(
                format!(
                    "Found early end of file while expecting to find: {:?}",
                    expected
                ),
                (location.1).clone(),
                location.0,
                None,
            ),
            ParseError::UnrecognizedToken { token, expected } => CompileError::from_at(
                format!(
                    "Found {:?} while expecting to find: {:?}",
                    token.1, expected
                ),
                (token.0).1.clone(),
                (token.0).0,
                Some((token.2).0),
            ),
            ParseError::User { .. } => {
                unreachable!("I haven't defined a custom parsing error. This is odd")
            }
        });
    }

    fn getAstMacro(
        initialToken: &FilePreTokPos<PreToken>,
        tokens: Vec<FilePreTokPos<PreToken>>,
    ) -> Result<DefineAst, CompileMsg> {
        let mut res = DefineAst {
            id: String::new(),
            param: None,
            variadic: IsVariadic::False,
            replacement: vec![],
            expandFunc: &Self::expand,
        };
        let mut ntok = tokens
            .into_iter()
            .skip_while(|tok| tok.tokPos.tok.isWhitespace());
        res.id = if let Some(tokId) = ntok.next() {
            if let PreToken::Ident(idStr) = &tokId.tokPos.tok {
                idStr.to_string()
            } else {
                return Err(CompileError::from_preTo(
                    "Expected identifier, instead found: ".to_string() + tokId.tokPos.tok.to_str(),
                    &tokId,
                ));
            }
        } else {
            return Err(CompileError::from_preTo(
                "Expected identifier in macro definition",
                initialToken,
            ));
        };
        let mut rlt;
        if let Some(tokLParen) = ntok.next() {
            match &tokLParen.tokPos.tok {
                PreToken::Whitespace(_) =>
                // We have a replacement macro
                {
                    rlt = ntok
                        .skip_while(|tok| tok.tokPos.tok.isWhitespace())
                        .collect::<Vec<FilePreTokPos<PreToken>>>();
                }
                PreToken::OperatorPunctuator("(") => {
                    // We have a function macro
                    let mut paren = ntok
                        .by_ref()
                        .take_while(|tok| {
                            !matches!(tok.tokPos.tok, PreToken::OperatorPunctuator(")"))
                        })
                        .filter(|tok| !tok.tokPos.tok.isWhitespace());
                    let mut param = vec![];
                    loop {
                        let paramData = paren
                            .by_ref()
                            .take_while(|x| {
                                !matches!(x.tokPos.tok, PreToken::OperatorPunctuator(","))
                            })
                            .collect::<Vec<FilePreTokPos<PreToken>>>();
                        let identParamTokens = paramData
                            .iter()
                            .map(|x| &x.tokPos.tok)
                            .collect::<Vec<&PreToken>>();
                        match identParamTokens.as_slice() {
                            [] => {
                                break;
                            }
                            [PreToken::OperatorPunctuator("...")] => {
                                res.variadic = IsVariadic::True(String::new());
                                break;
                            }
                            [PreToken::Ident(id), PreToken::OperatorPunctuator("...")]
                            | [PreToken::OperatorPunctuator("..."), PreToken::Ident(id)] => {
                                res.variadic = IsVariadic::True(id.to_string());
                                break;
                            }
                            [PreToken::Ident(id)] => {
                                param.push(id.to_string());
                            }
                            _ => {
                                return Err(CompileError::from_preTo(
                                    format!(
                                        "Non-valid parameter to function-like macro: {:?}",
                                        identParamTokens
                                    ),
                                    initialToken,
                                ));
                            }
                        }
                    }

                    if let Some(prepro) = paren.next() {
                        return Err(CompileError::from_preTo(
                            "Unparsable extra token in macro parameter",
                            &prepro,
                        ));
                    }
                    if !all_unique_elements(&param) {
                        return Err(CompileError::from_preTo(
                            "Repeated identifiers in parameters",
                            &tokLParen,
                        ));
                    }

                    rlt = ntok
                        .skip_while(|tok| tok.tokPos.tok.isWhitespace())
                        .collect::<Vec<FilePreTokPos<PreToken>>>();

                    res.param = Some(param);
                }
                _ => {
                    // We have a replacement macro, but the first token is not whitespace. This is technically an extension
                    rlt = vec![tokLParen];
                    ntok.collect_into(&mut rlt);
                }
            }

            let mut rl = Self::parseReplList(&res, rlt)?;
            while rl.last().is_some_and(|tok| {
                matches!(
                    tok,
                    PreTokenDefine::Normal(filePreTokPosMatchArm!(PreToken::Whitespace(_)))
                )
            }) {
                rl.pop();
            }
            if res.variadic == IsVariadic::False {
                if rl
                    .iter()
                    .any(|x| matches!(x, PreTokenDefine::VariadicArg(_)))
                {
                    return Err(CompileError::from_preTo(
                        "Non-variadic macro can't use __VA_ARGS__",
                        initialToken,
                    ));
                }
                if rl
                    .iter()
                    .any(|x| matches!(x, PreTokenDefine::VariadicOpt(_, _)))
                {
                    return Err(CompileError::from_preTo(
                        "Non-variadic macro can't use __VA_OPT__",
                        initialToken,
                    ));
                }
            }

            res.replacement = rl;
        }
        return Ok(res);
    }

    fn defineMacroImpl(
        &mut self,
        vecPrepro: Vec<FilePreTokPos<PreToken>>,
        preToken: &FilePreTokPos<PreToken>,
    ) -> Result<(), CompileMsg> {
        let def = {
            match Self::getAstMacro(preToken, vecPrepro) {
                Err(err) => {
                    return Err(err);
                }
                Ok(def) => def,
            }
        };

        match self.definitions.get_mut(&def.id) {
            Some(other) => {
                *other = def;
                return Err(CompileWarning::from_preTo("Redefining macro", preToken));
            }
            None => {
                self.definitions.insert(def.id.clone(), def);
            }
        }
        return Ok(());
    }

    pub fn defineMacro(&mut self, preToken: FilePreTokPos<PreToken>) {
        let vecPrepro = Iterator::take_while(&mut self.multilexer, |pre| {
            pre.tokPos.tok != PreToken::Newline
        })
        .collect::<Vec<FilePreTokPos<PreToken>>>();
        {
            let res = self.defineMacroImpl(vecPrepro, &preToken);
            if let Err(err) = res {
                self.errors.push_back(err);
            };
        }
        log::debug!("Macros:");
        for defi in self.definitions.values() {
            log::debug!("{:?}", defi);
        }
    }
}
