use std::collections::{HashMap, VecDeque};

use multiset::HashMultiSet;

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
    fn anyNonMetaToken(toks: &VecDeque<FilePreTokPos<PreToken>>) -> bool {
        for tok in toks {
            if !matches!(
                tok.tokPos.tok,
                PreToken::ValidNop | PreToken::EnableMacro(_) | PreToken::DisableMacro(_)
            ) {
                return true;
            }
        }
        false
    }

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
        disabledMacros: &HashMultiSet<String>,
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
            while let Some(tok) = paramLexer.next() {
                match &tok {
                    filePreTokPosMatchArm!(PreToken::EnableMacro(nameMacro)) => {
                        paramDisabledMacros.remove(nameMacro);
                        preproTokie.push_back(tok.clone());
                    }
                    filePreTokPosMatchArm!(PreToken::DisableMacro(nameMacro)) => {
                        paramDisabledMacros.insert(nameMacro.clone());
                        preproTokie.push_back(tok.clone());
                    }
                    filePreTokPosMatchArm!(PreToken::Ident(_)) => {
                        let toks = Self::macroExpand(
                            definitions,
                            &paramDisabledMacros,
                            &mut paramLexer,
                            tok,
                        )?;
                        toks.into_iter()
                            .collect_into::<VecDeque<FilePreTokPos<PreToken>>>(&mut preproTokie);
                    }
                    _ => {
                        preproTokie.push_back(tok);
                    }
                }
            }
            log::debug!(
                "EXPANDED INTO: {:?}",
                preproTokie
                    .iter()
                    .map(|x| x.tokString())
                    .collect::<Vec<_>>()
            );

            {
                let mut moded = true;
                let mut metas = VecDeque::new();
                while moded {
                    moded = false;
                    if let Some(filePreTokPosMatchArm!(PreToken::Whitespace(_))) =
                        preproTokie.front()
                    {
                        moded = true;
                        preproTokie.pop_front();
                    } else if let Some(
                        filePreTokPosMatchArm!(
                            PreToken::ValidNop
                                | PreToken::EnableMacro(_)
                                | PreToken::DisableMacro(_)
                        ),
                    ) = preproTokie.front()
                    {
                        moded = true;
                        metas.push_front(preproTokie.pop_front().unwrap());
                    }
                }
                for meta in metas {
                    preproTokie.push_front(meta);
                }
            }

            {
                let mut moded = true;
                let mut metas = VecDeque::new();
                while moded {
                    moded = false;
                    if let Some(filePreTokPosMatchArm!(PreToken::Whitespace(_))) =
                        preproTokie.back()
                    {
                        moded = true;
                        preproTokie.pop_back();
                    } else if let Some(
                        filePreTokPosMatchArm!(
                            PreToken::ValidNop
                                | PreToken::EnableMacro(_)
                                | PreToken::DisableMacro(_)
                        ),
                    ) = preproTokie.back()
                    {
                        moded = true;
                        metas.push_front(preproTokie.pop_back().unwrap());
                    }
                }
                for meta in metas {
                    preproTokie.push_back(meta);
                }
            }

            if preproTokie.is_empty() {
                preproTokie.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, a));
            }
            result.append(&mut preproTokie);
        } else {
            namedArgs
                .get(&a.tokPos.tok)
                .unwrap()
                .clone()
                .into_iter()
                .collect_into(&mut result);
            if namedArgs.get(&a.tokPos.tok).unwrap().is_empty() {
                FilePreTokPos::new_meta_c(PreToken::ValidNop, a);
            }
        }
        return Ok(result);
    }

    fn expandVariadicArg(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashMultiSet<String>,
        lexer: &MultiLexer,
        variadic: &Vec<Vec<FilePreTokPos<PreToken>>>,
        expandArg: bool,
        vaTok: &FilePreTokPos<()>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        if expandArg {
            let mut tempResult = VecDeque::new();
            for posVariadic in 0..variadic.len() {
                let mut paramLexer = MultiLexer::new_def(lexer.fileMapping());
                paramLexer.pushTokensVec(variadic[posVariadic].clone());
                let mut preproTokie = VecDeque::new();
                while let Some(tok) = paramLexer.next() {
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
                }
                tempResult.append(&mut preproTokie);

                if posVariadic + 1 != variadic.len() {
                    tempResult.push_back(FilePreTokPos::new_meta_c(
                        PreToken::OperatorPunctuator(","),
                        vaTok,
                    ))
                }
            }
            while let Some(filePreTokPosMatchArm!(PreToken::Whitespace(_))) = tempResult.front() {
                tempResult.pop_front();
            }
            while let Some(filePreTokPosMatchArm!(PreToken::Whitespace(_))) = tempResult.back() {
                tempResult.pop_back();
            }
            if tempResult.is_empty() {
                tempResult.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, vaTok));
            }
            result.append(&mut tempResult);
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
            if variadic.is_empty() {
                result.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, vaTok));
            }
        }
        return Ok(result);
    }

    fn expandHash(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashMultiSet<String>,
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
        let mut text = String::new();
        resChild.pop_back();
        resChild.pop_front();

        {
            let mut lastTokenWasWhitespace = false;
            for el in resChild.iter() {
                match el {
                    filePreTokPosMatchArm!(PreToken::StringLiteral(_))
                    | filePreTokPosMatchArm!(PreToken::UdStringLiteral(_))
                    | filePreTokPosMatchArm!(PreToken::CharLiteral(_))
                    | filePreTokPosMatchArm!(PreToken::UdCharLiteral(_)) => {
                        text.push_str(
                            el.tokPos
                                .tok
                                .to_str()
                                .replace('\\', "\\\\")
                                .replace('\"', "\\\"")
                                .as_str(),
                        );
                        lastTokenWasWhitespace = false;
                    }
                    filePreTokPosMatchArm!(PreToken::Newline)
                    | filePreTokPosMatchArm!(PreToken::Whitespace(_)) => {
                        if lastTokenWasWhitespace {
                            continue;
                        }
                        text.push(' ');
                        lastTokenWasWhitespace = true;
                    }
                    _ => {
                        text.push_str(el.tokPos.tok.to_str());
                        lastTokenWasWhitespace = false;
                    }
                }
            }
        }

        text = text.trim().to_string();
        text.insert(0, '"');
        text.push('"');

        result.push_back(FilePreTokPos::new_meta_c(
            PreToken::RawStringLiteral(text),
            pos,
        ));
        return Ok(result);
    }

    fn expandHashHash(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        _definitions: &HashMap<String, DefineAst>,
        _disabledMacros: &HashMultiSet<String>,
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
                &HashMap::new(),
                &HashMultiSet::new(),
                lexer,
                namedArgs,
                variadic,
                astId,
                right,
                true,
            )?;

            expR.pop_back();
            expR.pop_front();

            if !Self::anyNonMetaToken(&expR) {
                result.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, pos));
                return Ok(result);
            }

            if let PreTokenDefine::Normal(comma) = left.first().unwrap() {
                result.push_back(comma.clone());
                result.append(&mut expR);
            }
        } else {
            // We extract the contents of the left and right side
            let mut expL = Self::expand(
                &HashMap::new(),
                &HashMultiSet::new(),
                lexer,
                namedArgs,
                variadic,
                astId,
                left,
                true,
            )?;
            expL.pop_back();
            expL.pop_front();
            let mut expR = Self::expand(
                &HashMap::new(),
                &HashMultiSet::new(),
                lexer,
                namedArgs,
                variadic,
                astId,
                right,
                true,
            )?;
            expR.pop_back();
            expR.pop_front();
            log::trace!(
                "L: {:?}\nR: {:?}",
                expL.iter().map(|x| x.tokString()).collect::<Vec<_>>(),
                expR.iter().map(|x| x.tokString()).collect::<Vec<_>>(),
            );

            // And we merge them. Note that the resulting token may not be valid
            if !expL.is_empty() || !expR.is_empty() {
                let mut expectedStr = String::new();
                for ele in expL {
                    expectedStr += ele.tokPos.tok.to_str();
                }
                for ele in expR {
                    expectedStr += ele.tokPos.tok.to_str();
                }
                let mut receivedTok = PreLexer::new(expectedStr.clone()).collect::<Vec<_>>();
                receivedTok.pop();
                receivedTok.into_iter().for_each(|x| {
                    result.push_back(FilePreTokPos::new_meta_c(x.tok, pos));
                });
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
        disabledMacros: &HashMultiSet<String>,
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
            x.iter().any(|x| -> bool {
                !filePreTokPosMatches!(
                    x,
                    PreToken::Whitespace(_)
                        | PreToken::Newline
                        | PreToken::EnableMacro(_)
                        | PreToken::DisableMacro(_)
                        | PreToken::ValidNop
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

            if Self::anyNonMetaToken(&res) {
                result.append(
                    &mut res
                        .into_iter()
                        .filter(|x| !filePreTokPosMatches!(x, PreToken::ValidNop))
                        .collect(),
                );
            } else {
                result.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, pos));
            }
        }
        return Ok(result);
    }

    fn expand(
        definitions: &HashMap<String, DefineAst>,
        disabledMacros: &HashMultiSet<String>,
        lexer: &MultiLexer,
        namedArgs: &HashMap<String, Vec<FilePreTokPos<PreToken>>>,
        variadic: &Vec<Vec<FilePreTokPos<PreToken>>>,
        astId: &String,
        replacement: &Vec<PreTokenDefine>,
        expandArg: bool,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut result = VecDeque::new();
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
        result.push_front(FilePreTokPos::new_meta(PreToken::DisableMacro(
            astId.clone(),
        )));
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
        for param in params {
            named.insert(param.clone(), paramRes.remove(0));
        }
        return (named, paramRes);
    }

    fn hasMatchingClosingParen(lexer: &mut MultiLexer) -> bool {
        let mut openParens: usize = 0;

        let mut tokies: Vec<FilePreTokPos<PreToken>> = vec![];

        loop {
            let tok = lexer.next();
            match (openParens, &tok) {
                (_, None) => {
                    lexer.pushTokensVec(tokies);
                    return false;
                }
                (0, Some(filePreTokPosMatchArm!(PreToken::OperatorPunctuator(")")))) => {
                    tokies.push(tok.unwrap());
                    lexer.pushTokensVec(tokies);
                    return true;
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
    }

    fn parseParams(
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

        if tokies.len() == 1 && tokies.last().unwrap().is_empty() && max == 0 {
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
        disabledMacros: &HashMultiSet<String>,
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
                    let mut residual = vec![];
                    let tokParen = loop {
                        let t = lexer.next();
                        if let Some(t) = t {
                            match t.tokPos.tok {
                                PreToken::OperatorPunctuator("(") => {
                                    break t;
                                }
                                PreToken::EnableMacro(_)
                                | PreToken::DisableMacro(_)
                                | PreToken::ValidNop
                                | PreToken::Newline
                                | PreToken::Whitespace(_) => residual.push(t),
                                _ => {
                                    residual.push(t);
                                    lexer.pushTokensVec(residual);
                                    return Ok(vec![newToken]);
                                }
                            }
                        } else {
                            lexer.pushTokensVec(residual);
                            return Ok(vec![newToken]);
                        }
                    };
                    if !Self::hasMatchingClosingParen(lexer) {
                        residual.push(tokParen);
                        lexer.pushTokensVec(residual);
                        return Ok(vec![newToken]);
                    }

                    let paramsRes = Self::parseParams(lexer, min, max, tokParen)?;
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

                    log::debug!(
                        "Macro expansion success: {:?}",
                        success
                            .clone()
                            .into_iter()
                            .map(|x| x.tokPos.tok)
                            .collect::<Vec<_>>()
                    );

                    lexer.pushTokensDec(success);
                    lexer.pushTokensDec(
                        residual
                            .into_iter()
                            .filter(|x| {
                                filePreTokPosMatches!(
                                    x,
                                    PreToken::EnableMacro(_) | PreToken::DisableMacro(_)
                                )
                            })
                            .collect(),
                    );

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
                    log::debug!(
                        "Macro expansion success: {:?}",
                        success
                            .clone()
                            .into_iter()
                            .map(|x| x.tokPos.tok)
                            .collect::<Vec<_>>()
                    );
                    lexer.pushTokensDec(success);
                    return Ok(vec![]);
                }
            }
        }
        return Ok(vec![newToken]);
    }
}
