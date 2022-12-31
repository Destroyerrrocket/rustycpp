//! Parser of `#define` directives.

use std::collections::VecDeque;

use crate::{
    fileTokPosMatchArm,
    grammars::{
        defineast::{DefineAst, IsVariadic, PreTokenDefine, PreTokenDefinePreParse},
        defineparser,
    },
    preprocessor::pretoken::{PreToken, PreprocessingOperator},
    utils::{
        funcs::all_unique_elements,
        structs::{CompileError, CompileMsg, CompileMsgImpl, CompileWarning, FileTokPos, TokPos},
    },
};

use super::Preprocessor;

impl Preprocessor {
    /// Parse a the replacement list of the define. It preprocesses the tokens
    /// to aid lalrpop, and collects the resulting tokens from it.
    fn parseReplList(
        parse: &DefineAst,
        tokens: Vec<FileTokPos<PreToken>>,
    ) -> Result<Vec<PreTokenDefine>, CompileMsg> {
        let variadicStr = if let IsVariadic::True(stri) = &parse.variadic {
            stri.as_str()
        } else {
            ""
        };
        let mut vaOptExpectParen: Vec<i32> = vec![];
        #[allow(clippy::needless_collect)]
        let toksPre: Vec<FileTokPos<PreTokenDefinePreParse>> = tokens
            .into_iter()
            .map(|tok| FileTokPos {
                file: tok.file,
                tokPos: TokPos::<PreTokenDefinePreParse> {
                    start: tok.tokPos.start,
                    tok: match tok.tokPos.tok {
                        PreToken::Ident(s) => {
                            if parse.param.as_ref().is_some_and(|param| param.contains(&s)) {
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
            .collect() /* DO NOT REMOVE. NEEDED FOR CORRECT REVERSE */;

        let mut inHashHash = false;

        let mut toksPre = toksPre
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
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<VecDeque<_>>();

        let res = defineparser::parseMacroDefinition(&mut toksPre);
        return res;
    }

    /// Generate the definition info of the macro, mainly the name and the
    /// parameters. Delegates the replacement list to `parseReplList`
    fn getAstMacro(
        initialToken: &FileTokPos<PreToken>,
        tokens: Vec<FileTokPos<PreToken>>,
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
                return Err(CompileError::fromPreTo(
                    "Expected identifier, instead found: ".to_string() + tokId.tokPos.tok.to_str(),
                    &tokId,
                ));
            }
        } else {
            return Err(CompileError::fromPreTo(
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
                        .collect::<Vec<FileTokPos<PreToken>>>();
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
                            .collect::<Vec<FileTokPos<PreToken>>>();
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
                                return Err(CompileError::fromPreTo(
                                    format!(
                                        "Non-valid parameter to function-like macro: {identParamTokens:?}"
                                    ),
                                    initialToken,
                                ));
                            }
                        }
                    }

                    if let Some(prepro) = paren.next() {
                        return Err(CompileError::fromPreTo(
                            "Unparsable extra token in macro parameter",
                            &prepro,
                        ));
                    }
                    if !all_unique_elements(&param) {
                        return Err(CompileError::fromPreTo(
                            "Repeated identifiers in parameters",
                            &tokLParen,
                        ));
                    }

                    rlt = ntok
                        .skip_while(|tok| tok.tokPos.tok.isWhitespace())
                        .collect::<Vec<FileTokPos<PreToken>>>();

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
                    PreTokenDefine::Normal(fileTokPosMatchArm!(PreToken::Whitespace(_)))
                )
            }) {
                rl.pop();
            }
            if res.variadic == IsVariadic::False {
                if rl
                    .iter()
                    .any(|x| matches!(x, PreTokenDefine::VariadicArg(_)))
                {
                    return Err(CompileError::fromPreTo(
                        "Non-variadic macro can't use __VA_ARGS__",
                        initialToken,
                    ));
                }
                if rl
                    .iter()
                    .any(|x| matches!(x, PreTokenDefine::VariadicOpt(_, _)))
                {
                    return Err(CompileError::fromPreTo(
                        "Non-variadic macro can't use __VA_OPT__",
                        initialToken,
                    ));
                }
            }

            res.replacement = rl;
        }
        return Ok(res);
    }

    /// Parse a macro definition and add it to the list of definitions.
    fn defineMacroImpl(
        &mut self,
        vecPrepro: Vec<FileTokPos<PreToken>>,
        preToken: &FileTokPos<PreToken>,
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
                return Err(CompileWarning::fromPreTo("Redefining macro", preToken));
            }
            None => {
                self.definitions.insert(def.id.clone(), def);
            }
        }
        return Ok(());
    }

    /// Parse a macro definition and add it to the list of definitions
    pub fn defineMacro(&mut self, preToken: FileTokPos<PreToken>) {
        let vecPrepro = Iterator::take_while(&mut self.multilexer, |pre| {
            pre.tokPos.tok != PreToken::Newline
        })
        .collect::<Vec<FileTokPos<PreToken>>>();
        {
            let res = self.defineMacroImpl(vecPrepro, &preToken);
            if let Err(err) = res {
                self.errors.push_back(err);
            };
        }
        log::trace!("Macros:");
        for defi in self.definitions.values() {
            log::trace!("{:?}", defi);
        }
    }
}
