use std::collections::{HashMap, HashSet};

use crate::{
    filePreTokPosMatch,
    grammars::defineast::{DefineAst, IsVariadic},
    utils::{
        pretoken::PreToken,
        structs::{CompileError, CompileMsg, FilePreTokPos, PreTokPos},
    },
};

use super::{multilexer::MultiLexer, Preprocessor};

impl Preprocessor {
    fn expand(
        _lexer: &mut MultiLexer,
        namedArgs: HashMap<String, Vec<FilePreTokPos<PreToken>>>,
        variadic: Vec<Vec<FilePreTokPos<PreToken>>>,
        ast: &DefineAst,
    ) -> Result<Vec<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut result = vec![FilePreTokPos::new_meta(PreToken::DisableMacro(
            ast.id.clone(),
        ))];

        for tok in ast.replacement.iter() {
            match tok {
                crate::grammars::defineast::PreTokenDefine::Normal(t) => {
                    result.push(t.clone());
                }
                crate::grammars::defineast::PreTokenDefine::Arg(a) => {
                    result.push(FilePreTokPos::new_meta(PreToken::EnableMacro(
                        ast.id.clone(),
                    )));
                    result.append(&mut namedArgs.get(a).unwrap().clone());
                    result.push(FilePreTokPos::new_meta(PreToken::DisableMacro(
                        ast.id.clone(),
                    )));
                }
                crate::grammars::defineast::PreTokenDefine::VariadicArg => {
                    result.push(FilePreTokPos::new_meta(PreToken::EnableMacro(
                        ast.id.clone(),
                    )));
                    for _ in variadic.iter() {
                        todo!("I have to implement these")
                    }
                    result.push(FilePreTokPos::new_meta(PreToken::DisableMacro(
                        ast.id.clone(),
                    )));
                }
                crate::grammars::defineast::PreTokenDefine::Hash(_) => todo!(),
                crate::grammars::defineast::PreTokenDefine::HashHash(_, _) => todo!(),
                crate::grammars::defineast::PreTokenDefine::VariadicOpt(_) => todo!(),
            }
        }
        result.push(FilePreTokPos::new_meta(PreToken::EnableMacro(
            ast.id.clone(),
        )));
        return Ok(result);
    }

    fn generateParamMap(
        mut paramRes: Vec<Vec<FilePreTokPos<PreToken>>>,
        params: &Vec<String>,
    ) -> (
        HashMap<String, Vec<FilePreTokPos<PreToken>>>,
        Vec<Vec<FilePreTokPos<PreToken>>>,
    ) {
        let mut named = HashMap::new();
        for i in 0..params.len() {
            named.insert(params[i].clone(), paramRes.remove(0));
        }
        return (named, paramRes);
    }

    fn parseParams(
        _definitions: &HashMap<String, DefineAst>,
        _disabledMacros: &HashSet<String>,
        lexer: &mut MultiLexer,
        min: usize,
        max: usize,
        openParen: FilePreTokPos<PreToken>,
    ) -> Result<Vec<Vec<FilePreTokPos<PreToken>>>, CompileMsg> {
        let mut openParens: usize = 0;

        let mut tokies: Vec<Vec<FilePreTokPos<PreToken>>> = vec![vec![]];

        let closeParen = loop {
            let tok = lexer.next();
            match (openParens, &tok) {
                (_, None) => {
                    return Err(CompileError::from_preTo(
                        "Expected matching closing parentheses for this '('",
                        &openParen,
                    ));
                }
                (0, Some(filePreTokPosMatch!(PreToken::OperatorPunctuator(",")))) => {
                    tokies.push(vec![]);
                }
                (0, Some(filePreTokPosMatch!(PreToken::OperatorPunctuator(")")))) => {
                    break tok.unwrap();
                }
                (_, Some(filePreTokPosMatch!(PreToken::OperatorPunctuator(")")))) => {
                    tokies.last_mut().unwrap().push(tok.unwrap());
                    openParens -= 1;
                }
                (_, Some(filePreTokPosMatch!(PreToken::OperatorPunctuator("(")))) => {
                    tokies.last_mut().unwrap().push(tok.unwrap());
                    openParens += 1;
                }
                (_, Some(filePreTokPosMatch!(PreToken::Whitespace(_))))
                | (_, Some(filePreTokPosMatch!(PreToken::Newline))) => {
                    let tokie = tokies.last_mut().unwrap();
                    if !tokie.is_empty() {
                        tokie.push(tok.unwrap());
                    }
                }
                _ => {
                    tokies.last_mut().unwrap().push(tok.unwrap());
                }
            };
        };
        if tokies.last().unwrap().is_empty() {
            tokies.pop();
        }

        let len = tokies.len();
        if min <= len && len <= max {
            /*
            let mut preproTokies = vec![];

            for tokie in tokies {
                let mut paramLexer = MultiLexer::new_def(lexer.fileMapping());
                paramLexer.pushTokensVec(tokie);
                let mut preproTokie = vec![];
                loop {
                    if let Some(tok) = paramLexer.next() {
                        match &tok {
                            filePreTokPosMatch!(PreToken::Ident(_)) => {
                                let mut toks = Self::macroExpand(definitions, disabledMacros,&mut paramLexer, tok)?;
                                preproTokie.append(&mut toks);
                            }
                            _ => {
                                preproTokie.push(tok);
                            }
                        }
                    } else {break;}
                }
                preproTokies.push(preproTokie);
            }
            */
            return Ok(tokies);
        }

        if min == max {
            return Err(CompileError::from_preTo(
                format!("Expected {} parameters. found {}", min, len),
                &closeParen,
            ));
        }
        if len < min {
            return Err(CompileError::from_preTo(
                format!("Expected at least {} parameters. found {}", min, len),
                &closeParen,
            ));
        } else {
            return Err(CompileError::from_preTo(
                format!("Expected at most {} parameters. found {}", max, len),
                &closeParen,
            ));
        }
    }

    pub fn macroExpand(
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashSet<String>,
        lexer: &mut MultiLexer,
        newToken: FilePreTokPos<PreToken>,
    ) -> Result<Vec<FilePreTokPos<PreToken>>, CompileMsg> {
        if !disabledMacros.contains(newToken.tokPos.tok.to_str()) {
            if let Some(macroAst) = definitions.get(newToken.tokPos.tok.to_str()) {
                if let Some(params) = &macroAst.param {
                    let min = params.len();
                    let max: usize = if macroAst.variadic == IsVariadic::False {
                        min
                    } else {
                        usize::MAX
                    };

                    // Do we have "("? if not, we don't expand
                    let mut res = vec![newToken];
                    let tokParen = loop {
                        let t = lexer.next();
                        if let Some(t) = t {
                            match t.tokPos.tok {
                                PreToken::Whitespace(_) => res.push(t),
                                PreToken::OperatorPunctuator("(") => {
                                    break t;
                                }
                                _ => {
                                    lexer.pushToken(t);
                                    return Ok(res);
                                }
                            }
                        } else {
                            return Ok(res);
                        }
                    };

                    let paramsRes =
                        Self::parseParams(definitions, disabledMacros, lexer, min, max, tokParen)?;
                    let (namedArgs, variadic) = Self::generateParamMap(paramsRes, params);
                    res = Self::expand(lexer, namedArgs, variadic, macroAst)?;
                    lexer.pushTokensVec(res);
                    return Ok(vec![]);
                } else {
                    let res = Self::expand(lexer, HashMap::new(), vec![], macroAst)?;
                    lexer.pushTokensVec(res);
                    return Ok(vec![]);
                }
            }
        }
        return Ok(vec![newToken]);
    }
}
