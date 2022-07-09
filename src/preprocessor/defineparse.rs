use std::sync::Arc;

use crate::{
    filePreTokPosMatch,
    grammars::{define, defineast::*},
    utils::{
        funcs::all_unique_elements,
        lalrpoplexerwrapper::LalrPopLexerWrapper,
        pretoken::{PreToken, PreprocessingOperator},
        structs::*,
    },
};

use super::Preprocessor;

impl Preprocessor {
    fn parseReplList(
        &mut self,
        _preToken: &FilePreTokPos<PreToken>,
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
                            } else if s.as_str() == variadicStr {
                                PreTokenDefinePreParse::VariadicArg
                            } else if s.as_str() == "__VA_ARGS__" {
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
                            let shouldMut = if let Some(pars) = vaOptExpectParen.last_mut() {
                                *pars -= 1;
                                *pars == -1
                            } else {
                                false
                            };
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
            .rev()
            .collect::<Vec<FilePreTokPos<PreTokenDefinePreParse>>>();

        println!("DEBUG: {:?}", toksPre);
        let lexer = LalrPopLexerWrapper::new(toksPre.as_slice());
        let res = define::DefineStmtParser::new().parse(lexer);
        let mut at: (usize, Arc<CompileFile>) = (0, Arc::new(CompileFile::default()));
        return res.map_err(|err| {
            let errMsg = format!("Parse error: {:?}", err);
            err.map_location(|e| at = e);
            CompileError::from_at(errMsg, at.1, at.0, None)
        });
    }

    fn getAstMacro(
        &mut self,
        initialToken: &FilePreTokPos<PreToken>,
        tokens: Vec<FilePreTokPos<PreToken>>,
    ) -> Result<DefineAst, CompileMsg> {
        let mut res = DefineAst {
            id: "".to_string(),
            param: None,
            variadic: IsVariadic::False,
            replacement: vec![],
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
                &initialToken,
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
                    res.param = Some(vec![]);
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
                                res.variadic = IsVariadic::True("".to_string());
                                break;
                            }
                            [PreToken::Ident(id), PreToken::OperatorPunctuator("...")]
                            | [PreToken::OperatorPunctuator("..."), PreToken::Ident(id)] => {
                                res.variadic = IsVariadic::True(id.to_string());
                                break;
                            }
                            [PreToken::Ident(id)] => {
                                res.param.as_mut().unwrap().push(id.to_string());
                            }
                            _ => {
                                return Err(CompileError::from_preTo(
                                    format!(
                                        "Non-valid parameter to function-like macro: {:?}",
                                        identParamTokens
                                    ),
                                    paramData.first().unwrap(),
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
                    if !all_unique_elements(res.param.as_ref().unwrap()) {
                        return Err(CompileError::from_preTo(
                            "Repeated identifiers in parameters",
                            &tokLParen,
                        ));
                    }

                    rlt = ntok
                        .skip_while(|tok| tok.tokPos.tok.isWhitespace())
                        .collect::<Vec<FilePreTokPos<PreToken>>>();
                }
                _ => {
                    // We have a replacement macro, but the first token is not whitespace. This is technically an extension
                    rlt = vec![tokLParen];
                    ntok.collect_into(&mut rlt);
                }
            }

            let mut rl = self.parseReplList(initialToken, &res, rlt)?;
            while rl.last().is_some_and(|tok| {
                matches!(
                    tok,
                    PreTokenDefine::Normal(filePreTokPosMatch!(PreToken::Whitespace(_)))
                )
            }) {
                rl.pop();
            }
            if res.variadic == IsVariadic::False {
                if rl.iter().any(|x| matches!(x, PreTokenDefine::VariadicArg)) {
                    return Err(CompileError::from_preTo(
                        "Non-variadic macro can't use __VA_ARGS__",
                        &initialToken,
                    ));
                }
                if rl
                    .iter()
                    .any(|x| matches!(x, PreTokenDefine::VariadicOpt(_)))
                {
                    return Err(CompileError::from_preTo(
                        "Non-variadic macro can't use __VA_OPT__",
                        &initialToken,
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
            match self.getAstMacro(&preToken, vecPrepro) {
                Err(err) => {
                    return Err(err);
                }
                Ok(def) => def,
            }
        };

        match self.definitions.get_mut(&def.id) {
            Some(other) => {
                *other = def;
                return Err(CompileWarning::from_preTo("Redefining macro", &preToken));
            }
            None => {
                self.definitions.insert(def.id.to_string(), def);
            }
        }
        return Ok(());
    }

    pub fn defineMacro(&mut self, preToken: FilePreTokPos<PreToken>) -> () {
        let vecPrepro = Iterator::take_while(&mut self.multilexer, |pre| {
            pre.tokPos.tok != PreToken::Newline
        })
        .collect::<Vec<FilePreTokPos<PreToken>>>();
        {
            let res = self.defineMacroImpl(vecPrepro, &preToken);
            match res {
                Err(err) => {
                    self.errors.push_back(err);
                }
                Ok(_) => {}
            };
        }
        println!("Macros:");
        for (_, defi) in self.definitions.iter() {
            println!("{:?}", defi);
        }
    }
}
