use std::collections::{HashMap, VecDeque};

use multiset::HashMultiSet;

use crate::{
    filePreTokPosMatchArm, filePreTokPosMatches,
    grammars::defineast::{DefineAst, IsVariadic, PreTokenDefine},
    preprocessor::{
        multilexer::MultiLexer, prelexer::PreLexer, pretoken::PreToken, structs::ExpandData,
    },
    utils::structs::{CompileError, CompileMsg, FilePreTokPos, PreTokPos},
};

use super::Preprocessor;

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

    pub fn expandASequenceOfTokens(
        mut selfContainedLexer: MultiLexer,
        macros: &HashMap<String, DefineAst>,
        disabledMacros: &HashMultiSet<String>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut paramDisabledMacros = disabledMacros.clone();
        let mut preproTokie = VecDeque::new();
        while let Some(tok) = selfContainedLexer.next() {
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
                    let toks = Self::macroExpandInternal(
                        macros,
                        &paramDisabledMacros,
                        &mut selfContainedLexer,
                        tok,
                    )?;
                    preproTokie.extend(toks);
                }
                _ => {
                    preproTokie.push_back(tok);
                }
            }
        }
        Ok(preproTokie)
    }

    fn expandNormal(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        t: &FilePreTokPos<PreToken>,
    ) -> VecDeque<FilePreTokPos<PreToken>> {
        result.push_back(t.clone());
        return result;
    }

    fn expandArg(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        expandData: ExpandData,
        a: &FilePreTokPos<String>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        if expandData.expandArg {
            let mut paramLexer = MultiLexer::new_def(expandData.lexer.fileMapping());
            paramLexer.pushTokensVec(expandData.namedArgs.get(&a.tokPos.tok).unwrap().clone());
            let mut preproTokie = Self::expandASequenceOfTokens(
                paramLexer,
                expandData.definitions,
                expandData.disabledMacros,
            )?;
            log::debug!(
                "EXPANDED INTO: {:?}",
                preproTokie
                    .iter()
                    .map(FilePreTokPos::tokStringDebug)
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

            if preproTokie.is_empty() {
                preproTokie.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, a));
            }
            result.append(&mut preproTokie);
        } else {
            expandData
                .namedArgs
                .get(&a.tokPos.tok)
                .unwrap()
                .clone()
                .into_iter()
                .collect_into(&mut result);
            if expandData.namedArgs.get(&a.tokPos.tok).unwrap().is_empty() {
                FilePreTokPos::new_meta_c(PreToken::ValidNop, a);
            }
        }
        return Ok(result);
    }

    fn expandVariadicArg(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        expandData: ExpandData,
        vaTok: &FilePreTokPos<()>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        if expandData.expandArg {
            let mut tempResult = VecDeque::new();
            for posVariadic in 0..expandData.variadic.len() {
                let mut paramLexer = MultiLexer::new_def(expandData.lexer.fileMapping());
                paramLexer.pushTokensVec(expandData.variadic[posVariadic].clone());
                let mut preproTokie = Self::expandASequenceOfTokens(
                    paramLexer,
                    expandData.definitions,
                    expandData.disabledMacros,
                )?;
                tempResult.append(&mut preproTokie);

                if posVariadic + 1 != expandData.variadic.len() {
                    tempResult.push_back(FilePreTokPos::new_meta_c(
                        PreToken::OperatorPunctuator(","),
                        vaTok,
                    ));
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
            for posVariadic in 0..expandData.variadic.len() {
                for v in &expandData.variadic[posVariadic] {
                    result.push_back(v.clone());
                }
                if posVariadic + 1 != expandData.variadic.len() {
                    result.push_back(FilePreTokPos::new_meta_c(
                        PreToken::OperatorPunctuator(","),
                        vaTok,
                    ));
                }
            }
            if expandData.variadic.is_empty() {
                result.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, vaTok));
            }
        }
        return Ok(result);
    }

    fn expandHash(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        expandData: ExpandData,
        pos: &FilePreTokPos<()>,
        tokie: &Vec<PreTokenDefine>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut resChild = Self::expand(ExpandData {
            definitions: expandData.definitions,
            disabledMacros: expandData.disabledMacros,
            lexer: expandData.lexer,
            namedArgs: expandData.namedArgs,
            variadic: expandData.variadic,
            astId: expandData.astId,
            replacement: tokie,
            expandArg: false,
            newToken: expandData.newToken,
        })?;
        let mut text = String::new();
        resChild.pop_back();
        resChild.pop_front();

        {
            let mut lastTokenWasWhitespace = false;
            for el in &resChild {
                match el {
                    filePreTokPosMatchArm!(
                        PreToken::StringLiteral(_)
                            | PreToken::UdStringLiteral(_)
                            | PreToken::CharLiteral(_)
                            | PreToken::UdCharLiteral(_)
                    ) => {
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
                    filePreTokPosMatchArm!(PreToken::Newline | PreToken::Whitespace(_)) => {
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
        expandData: ExpandData,
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
            let mut expR = Self::expand(ExpandData {
                definitions: &HashMap::new(),
                disabledMacros: &HashMultiSet::new(),
                lexer: expandData.lexer,
                namedArgs: expandData.namedArgs,
                variadic: expandData.variadic,
                astId: expandData.astId,
                replacement: right,
                expandArg: true,
                newToken: expandData.newToken,
            })?;

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
            let mut expL = Self::expand(ExpandData {
                definitions: &HashMap::new(),
                disabledMacros: &HashMultiSet::new(),
                lexer: expandData.lexer,
                namedArgs: expandData.namedArgs,
                variadic: expandData.variadic,
                astId: expandData.astId,
                replacement: left,
                expandArg: true,
                newToken: expandData.newToken,
            })?;
            expL.pop_back();
            expL.pop_front();
            let mut expR = Self::expand(ExpandData {
                definitions: &HashMap::new(),
                disabledMacros: &HashMultiSet::new(),
                lexer: expandData.lexer,
                namedArgs: expandData.namedArgs,
                variadic: expandData.variadic,
                astId: expandData.astId,
                replacement: right,
                expandArg: true,
                newToken: expandData.newToken,
            })?;
            expR.pop_back();
            expR.pop_front();
            log::trace!(
                "L: {:?}\nR: {:?}",
                expL.iter()
                    .map(FilePreTokPos::tokStringDebug)
                    .collect::<Vec<_>>(),
                expR.iter()
                    .map(FilePreTokPos::tokStringDebug)
                    .collect::<Vec<_>>(),
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
                for x in receivedTok {
                    result.push_back(FilePreTokPos::new_meta_c(x.tok, pos));
                }
            } else {
                // Both sides are empty. We add a ValidNop token
                result.push_back(FilePreTokPos::new_meta_c(PreToken::ValidNop, pos));
            }
        }
        return Ok(result);
    }

    fn expandVariadicOpt(
        mut result: VecDeque<FilePreTokPos<PreToken>>,
        expandData: ExpandData,
        pos: &FilePreTokPos<()>,
        tokies: &Vec<PreTokenDefine>,
    ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let vaOptEnabled = Self::expand(ExpandData {
            definitions: expandData.definitions,
            disabledMacros: expandData.disabledMacros,
            lexer: expandData.lexer,
            namedArgs: expandData.namedArgs,
            variadic: expandData.variadic,
            astId: expandData.astId,
            replacement: &vec![PreTokenDefine::VariadicArg(pos.clone())],
            expandArg: true,
            newToken: expandData.newToken,
        })
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
            let mut res = Self::expand(ExpandData {
                definitions: expandData.definitions,
                disabledMacros: expandData.disabledMacros,
                lexer: expandData.lexer,
                namedArgs: expandData.namedArgs,
                variadic: expandData.variadic,
                astId: expandData.astId,
                replacement: tokies,
                expandArg: expandData.expandArg,
                newToken: expandData.newToken,
            })?;
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

    pub fn expand(expandData: ExpandData) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut result = VecDeque::new();
        for tok in expandData.replacement {
            match tok {
                PreTokenDefine::Normal(t) => {
                    result = Self::expandNormal(result, t);
                }
                PreTokenDefine::Arg(a) => {
                    result = Self::expandArg(result, expandData.clone(), a)?;
                }
                PreTokenDefine::VariadicArg(vaTok) => {
                    result = Self::expandVariadicArg(result, expandData.clone(), vaTok)?;
                }
                PreTokenDefine::Hash(pos, tokie) => {
                    result = Self::expandHash(result, expandData.clone(), pos, tokie)?;
                }
                PreTokenDefine::HashHash(pos, left, right) => {
                    result = Self::expandHashHash(result, expandData.clone(), pos, left, right)?;
                }
                PreTokenDefine::VariadicOpt(pos, tokies) => {
                    result = Self::expandVariadicOpt(result, expandData.clone(), pos, tokies)?;
                }
            }
        }
        result.push_front(FilePreTokPos::new_meta(PreToken::DisableMacro(
            expandData.astId.clone(),
        )));
        result.push_back(FilePreTokPos::new_meta(PreToken::EnableMacro(
            expandData.astId.clone(),
        )));
        return Ok(result);
    }
}

struct ParamMapResult {
    namedParameters: HashMap<String, Vec<FilePreTokPos<PreToken>>>,
    varadicParameters: Vec<Vec<FilePreTokPos<PreToken>>>,
}

impl Preprocessor {
    fn generateParamMap(
        mut paramRes: Vec<Vec<FilePreTokPos<PreToken>>>,
        params: &Vec<String>,
    ) -> ParamMapResult {
        let mut named = HashMap::new();
        for param in params {
            named.insert(param.clone(), paramRes.remove(0));
        }
        return ParamMapResult {
            namedParameters: named,
            varadicParameters: paramRes,
        };
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
                (_, Some(filePreTokPosMatchArm!(PreToken::Whitespace(_) | PreToken::Newline))) => {
                    let tokie = tokies.last_mut().unwrap();
                    tokie.push(tok.unwrap());
                }
                _ => {
                    tokies.last_mut().unwrap().push(tok.unwrap());
                }
            };
        };

        for tokie in &mut tokies {
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

    pub fn macroExpandInternal(
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
                    let ParamMapResult {
                        namedParameters: namedArgs,
                        varadicParameters: variadic,
                    } = Self::generateParamMap(paramsRes, params);
                    let success = Self::expand(ExpandData {
                        definitions,
                        disabledMacros,
                        lexer,
                        namedArgs: &namedArgs,
                        variadic: &variadic,
                        astId: &macroAst.id,
                        replacement: &macroAst.replacement,
                        expandArg: true,
                        newToken: &newToken,
                    })?;

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
                } else {
                    let success = (macroAst.expandFunc)(ExpandData {
                        definitions,
                        disabledMacros,
                        lexer,
                        namedArgs: &HashMap::new(),
                        variadic: &vec![],
                        astId: &macroAst.id,
                        replacement: &macroAst.replacement,
                        expandArg: true,
                        newToken: &newToken,
                    })?;
                    log::debug!(
                        "Macro expansion success: {:?}",
                        success
                            .clone()
                            .into_iter()
                            .map(|x| x.tokPos.tok)
                            .collect::<Vec<_>>()
                    );
                    lexer.pushTokensDec(success);
                }
                return Ok(vec![]);
            }
        }
        return Ok(vec![newToken]);
    }

    pub fn macroExpand(
        &mut self,
        newToken: FilePreTokPos<PreToken>,
    ) -> Result<Vec<FilePreTokPos<PreToken>>, CompileMsg> {
        Self::macroExpandInternal(
            &self.definitions,
            &self.disabledMacros,
            &mut self.multilexer,
            newToken,
        )
    }
}