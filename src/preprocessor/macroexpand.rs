use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    filePreTokPosMatchArm, filePreTokPosMatches,
    grammars::defineast::{DefineAst, IsVariadic, PreTokenDefine},
    prelexer::PreLexer,
    utils::{
        pretoken::PreToken,
        structs::{CompileError, CompileMsg, FilePreTokPos, PreTokPos},
    },
};

use super::{multilexer::MultiLexer, Preprocessor};

impl Preprocessor {
    fn expandNormal(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        t: &FilePreTokPos<PreToken>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        result.push_back(t.clone());
        return Ok(result);
    }

    fn expandArg(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashSet<String>,
        lexer: &MultiLexer,
        namedArgs: &HashMap<String, Vec<FilePreTokPos<PreToken>>>,
        expandArg: bool,
        a: &FilePreTokPos<String>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        if expandArg {
            let mut paramLexer = MultiLexer::new_def(lexer.fileMapping());
            let mut paramDisabledMacros = disabledMacros.clone();
            paramLexer.pushTokensVec(namedArgs.get(&a.tokPos.tok).unwrap().clone());
            let mut preproTokie = VecDeque::new();
            loop {
                if let Some(tok) = paramLexer.next() {
                    match &tok {
                        filePreTokPosMatchArm!(PreToken::EnableMacro(nameMacro)) => {
                            paramDisabledMacros.remove(nameMacro);
                        }
                        filePreTokPosMatchArm!(PreToken::DisableMacro(nameMacro)) => {
                            paramDisabledMacros.insert(nameMacro.clone());
                        }
                        filePreTokPosMatchArm!(PreToken::Ident(_)) => {
                            let toks = Self::macroExpand(
                                definitions,
                                &paramDisabledMacros,
                                &mut paramLexer,
                                tok,
                            )?;
                            toks.into_iter()
                                .collect_into::<VecDeque<FilePreTokPos<PreToken>>>(
                                    &mut preproTokie,
                                );
                        }
                        _ => {
                            preproTokie.push_back(tok);
                        }
                    }
                } else {
                    break;
                }
            }
            log::debug!(
                "EXPANDED INTO: {:?}",
                preproTokie
                    .iter()
                    .map(|x| x.tokString())
                    .collect::<Vec<String>>()
            );
            result.append(&mut preproTokie);
        } else {
            namedArgs
                .get(&a.tokPos.tok)
                .unwrap()
                .clone()
                .into_iter()
                .collect_into(&mut result);
        }
        return Ok(result);
    }

    fn expandVariadicArg(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashSet<String>,
        lexer: &MultiLexer,
        variadic: &Vec<Vec<FilePreTokPos<PreToken>>>,
        expandArg: bool,
        vaTok: &FilePreTokPos<()>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        if expandArg {
            for posVariadic in 0..variadic.len() {
                let mut paramLexer = MultiLexer::new_def(lexer.fileMapping());
                paramLexer.pushTokensVec(variadic[posVariadic].clone());
                let mut preproTokie = VecDeque::new();
                loop {
                    if let Some(tok) = paramLexer.next() {
                        match &tok {
                            filePreTokPosMatchArm!(PreToken::Ident(_)) => {
                                let toks = Self::macroExpand(
                                    definitions,
                                    disabledMacros,
                                    &mut paramLexer,
                                    tok,
                                )?;
                                toks.into_iter()
                                    .collect_into::<VecDeque<FilePreTokPos<PreToken>>>(
                                        &mut preproTokie,
                                    );
                            }
                            _ => {
                                preproTokie.push_back(tok);
                            }
                        }
                    } else {
                        break;
                    }
                }
                result.append(&mut preproTokie);

                if posVariadic + 1 != variadic.len() {
                    result.push_back(FilePreTokPos::new_meta_c(
                        PreToken::OperatorPunctuator(","),
                        vaTok,
                    ))
                }
            }
        } else {
            for posVariadic in 0..variadic.len() {
                for v in variadic[posVariadic].iter() {
                    result.push_back(v.clone());
                }
                if posVariadic + 1 != variadic.len() {
                    result.push_back(FilePreTokPos::new_meta_c(
                        PreToken::OperatorPunctuator(","),
                        vaTok,
                    ))
                }
            }
        }
        return Ok(result);
    }

    fn expandHash(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashSet<String>,
        lexer: &MultiLexer,
        namedArgs: &HashMap<String, Vec<FilePreTokPos<PreToken>>>,
        variadic: &Vec<Vec<FilePreTokPos<PreToken>>>,
        astId: &String,
        pos: &FilePreTokPos<()>,
        tokie: &Vec<PreTokenDefine>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut resChild = Self::expand(
            definitions,
            disabledMacros,
            lexer,
            namedArgs,
            variadic,
            astId,
            tokie,
            false,
        )?;
        let mut text = "\"".to_string();
        resChild.pop_back();
        resChild.pop_front();

        while let Some(filePreTokPosMatchArm!(PreToken::Whitespace(_))) = resChild.back() {
            resChild.pop_back();
        }
        while let Some(filePreTokPosMatchArm!(PreToken::Whitespace(_))) = resChild.front() {
            resChild.pop_front();
        }

        for el in resChild.iter() {
            if filePreTokPosMatches!(el, PreToken::StringLiteral(_)) {
                text.push_str(
                    el.tokPos
                        .tok
                        .to_str()
                        .replace("\\", "\\\\")
                        .replace("\"", "\\\"")
                        .as_str(),
                );
            } else {
                text.push_str(&el.tokPos.tok.to_str());
            }
            log::debug!("CURRENT TEXT: {} WITH TOKEN: {:?}", text, el);
        }
        text.push('"');
        result.push_back(FilePreTokPos::new_meta_c(
            PreToken::RawStringLiteral(text),
            pos,
        ));
        return Ok(result);
    }

    fn expandHashHash(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashSet<String>,
        lexer: &MultiLexer,
        namedArgs: &HashMap<String, Vec<FilePreTokPos<PreToken>>>,
        variadic: &Vec<Vec<FilePreTokPos<PreToken>>>,
        astId: &String,
        pos: &FilePreTokPos<()>,
        left: &Vec<PreTokenDefine>,
        right: &Vec<PreTokenDefine>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        // We support the GNU extension  ",##__VA_ARGS__
        if matches!(
            left.first().unwrap(),
            PreTokenDefine::Normal(filePreTokPosMatchArm!(PreToken::OperatorPunctuator(",")))
        ) && matches!(right.first().unwrap(), PreTokenDefine::VariadicArg(_))
        {
            let mut expR = Self::expand(
                definitions,
                disabledMacros,
                lexer,
                namedArgs,
                variadic,
                astId,
                right,
                false,
            )?;
            expR.pop_back();
            expR.pop_front();

            if expR.is_empty() {
                return Ok(result);
            }

            if let PreTokenDefine::Normal(comma) = left.first().unwrap() {
                result.push_back(comma.clone());
                result.append(&mut expR);
            }
        } else {
            // We extract the contents of the left and right side
            let mut expL = Self::expand(
                definitions,
                disabledMacros,
                lexer,
                namedArgs,
                variadic,
                astId,
                left,
                false,
            )?;
            expL.pop_back();
            expL.pop_front();
            let mut expR = Self::expand(
                definitions,
                disabledMacros,
                lexer,
                namedArgs,
                variadic,
                astId,
                right,
                false,
            )?;
            expR.pop_back();
            expR.pop_front();

            // We remove the whitespace to the ## operator
            while !expL.is_empty() {
                if expL
                    .back()
                    .is_some_and(|x| filePreTokPosMatches!(x, PreToken::Whitespace(_)))
                {
                    expL.pop_back();
                } else {
                    break;
                }
            }

            while !expR.is_empty() {
                if expR
                    .front()
                    .is_some_and(|x| filePreTokPosMatches!(x, PreToken::Whitespace(_)))
                {
                    expR.pop_front();
                } else {
                    break;
                }
            }

            // And we merge them. Note that the resulting token may not be valid
            if !expL.is_empty() && !expR.is_empty() {
                let leftTok = expL.pop_back().unwrap();
                let rightTok = expR.pop_front().unwrap();
                let expectedStr =
                    leftTok.tokPos.tok.to_str().to_string() + rightTok.tokPos.tok.to_str();
                let receivedTok = PreLexer::new(expectedStr.clone()).next().unwrap();

                if receivedTok.tok.to_str().len() == expectedStr.len() {
                    result.append(&mut expL);
                    result.push_back(FilePreTokPos::new_meta_c(receivedTok.tok, pos));
                    result.append(&mut expR);
                } else {
                    return Err(CompileError::from_preTo(
                        format!("During macro expansion, I couldn't create a valid preprocessing token with \"{}\" and \"{}\".\nThe resulting token \"{}\" is not valid. Maybe the ## operator is unecessary?",
                            leftTok.tokPos.tok.to_str(),
                            rightTok.tokPos.tok.to_str(),
                            expectedStr,
                        ),
                            pos
                    ));
                }
            } else if !expL.is_empty() || !expR.is_empty() {
                // One of the sides is empty. We just append them
                result.append(&mut expL);
                result.append(&mut expR);
            } else {
                // Both sides are empty. We add a ValidNop token
                result.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, pos));
            }
        }
        return Ok(result);
    }

    fn expandVariadicOpt(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashSet<String>,
        lexer: &MultiLexer,
        namedArgs: &HashMap<String, Vec<FilePreTokPos<PreToken>>>,
        variadic: &Vec<Vec<FilePreTokPos<PreToken>>>,
        astId: &String,
        expandArg: bool,
        pos: &FilePreTokPos<()>,
        tokies: &Vec<PreTokenDefine>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let vaOptEnabled = Self::expand(
            definitions,
            disabledMacros,
            lexer,
            namedArgs,
            variadic,
            astId,
            &vec![PreTokenDefine::VariadicArg(pos.clone())],
            true,
        )
        .is_ok_and(|x| {
            x.into_iter().any(|x| -> bool {
                !filePreTokPosMatches!(
                    x,
                    PreToken::Whitespace(_)
                        | PreToken::Newline
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                )
            })
        });
        if vaOptEnabled {
            let mut res = Self::expand(
                definitions,
                disabledMacros,
                lexer,
                namedArgs,
                variadic,
                astId,
                tokies,
                expandArg,
            )?;
            res.pop_back();
            res.pop_front();
            result.append(&mut res);
        }
        return Ok(result);
    }

    fn expand(
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashSet<String>,
        lexer: &MultiLexer,
        namedArgs: &HashMap<String, Vec<FilePreTokPos<PreToken>>>,
        variadic: &Vec<Vec<FilePreTokPos<PreToken>>>,
        astId: &String,
        replacement: &Vec<PreTokenDefine>,
        expandArg: bool,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut result = VecDeque::new();
        result.push_back(FilePreTokPos::new_meta(PreToken::DisableMacro(
            astId.clone(),
        )));

        for tok in replacement {
            match tok {
                PreTokenDefine::Normal(t) => {
                    result = Self::expandNormal(result, t)?;
                }
                PreTokenDefine::Arg(a) => {
                    result = Self::expandArg(
                        result,
                        definitions,
                        disabledMacros,
                        lexer,
                        namedArgs,
                        expandArg,
                        a,
                    )?;
                }
                PreTokenDefine::VariadicArg(vaTok) => {
                    result = Self::expandVariadicArg(
                        result,
                        definitions,
                        disabledMacros,
                        lexer,
                        variadic,
                        expandArg,
                        vaTok,
                    )?;
                }
                PreTokenDefine::Hash(pos, tokie) => {
                    result = Self::expandHash(
                        result,
                        definitions,
                        disabledMacros,
                        lexer,
                        namedArgs,
                        variadic,
                        astId,
                        pos,
                        tokie,
                    )?;
                }
                PreTokenDefine::HashHash(pos, left, right) => {
                    result = Self::expandHashHash(
                        result,
                        definitions,
                        disabledMacros,
                        lexer,
                        namedArgs,
                        variadic,
                        astId,
                        pos,
                        left,
                        right,
                    )?;
                }
                PreTokenDefine::VariadicOpt(pos, tokies) => {
                    result = Self::expandVariadicOpt(
                        result,
                        definitions,
                        disabledMacros,
                        lexer,
                        namedArgs,
                        variadic,
                        astId,
                        expandArg,
                        pos,
                        tokies,
                    )?;
                }
            }
        }
        result.push_back(FilePreTokPos::new_meta(PreToken::EnableMacro(
            astId.clone(),
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
                (0, Some(filePreTokPosMatchArm!(PreToken::OperatorPunctuator(",")))) => {
                    tokies.push(vec![]);
                }
                (0, Some(filePreTokPosMatchArm!(PreToken::OperatorPunctuator(")")))) => {
                    break tok.unwrap();
                }
                (_, Some(filePreTokPosMatchArm!(PreToken::OperatorPunctuator(")")))) => {
                    tokies.last_mut().unwrap().push(tok.unwrap());
                    openParens -= 1;
                }
                (_, Some(filePreTokPosMatchArm!(PreToken::OperatorPunctuator("(")))) => {
                    tokies.last_mut().unwrap().push(tok.unwrap());
                    openParens += 1;
                }
                (_, Some(filePreTokPosMatchArm!(PreToken::Whitespace(_))))
                | (_, Some(filePreTokPosMatchArm!(PreToken::Newline))) => {
                    let tokie = tokies.last_mut().unwrap();
                    tokie.push(tok.unwrap());
                    /*if !tokie.is_empty() {
                        tokie.push(tok.unwrap());
                    }*/
                }
                _ => {
                    tokies.last_mut().unwrap().push(tok.unwrap());
                }
            };
        };

        for tokie in tokies.iter_mut() {
            while tokie
                .last()
                .is_some_and(|x| filePreTokPosMatches!(x, PreToken::Whitespace(_)))
            {
                tokie.pop();
            }
        }

        if !tokies.is_empty() && tokies.last().unwrap().is_empty() {
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
                    let success = Self::expand(
                        definitions,
                        disabledMacros,
                        lexer,
                        &namedArgs,
                        &variadic,
                        &macroAst.id,
                        &macroAst.replacement,
                        true,
                    )?;
                    lexer.pushTokensDec(success);
                    return Ok(vec![]);
                } else {
                    let success = Self::expand(
                        definitions,
                        disabledMacros,
                        lexer,
                        &HashMap::new(),
                        &vec![],
                        &macroAst.id,
                        &macroAst.replacement,
                        true,
                    )?;
                    lexer.pushTokensDec(success);
                    return Ok(vec![]);
                }
            }
        }
        return Ok(vec![newToken]);
    }
}
