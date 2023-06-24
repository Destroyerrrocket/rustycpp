use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::Preprocessor::Preprocessor;
use crate::Preprocessor::Pretoken::PreToken;
use crate::Utils::CompilerState::CompilerState;
use crate::Utils::FileMap::FileMap;
use crate::Utils::Parameters::Parameters;
use crate::Utils::StateCompileUnit::StateCompileUnit;
use crate::Utils::Structs::CompileMsg;

use test_log::test;

fn generateFileMap(files: &[(&'static str, String)]) -> (CompilerState, u64) {
    let mut params = Parameters::new();
    params.includeDirs.push(
        Path::new(file!())
            .parent()
            .unwrap()
            .join("include")
            .to_str()
            .unwrap()
            .to_string(),
    );

    for (filePath, _) in files {
        params.translationUnits.push((*filePath).to_string());
    }

    let parameters = Arc::new(params);
    let fileMap = Arc::new(Mutex::new(FileMap::new(parameters.clone())));
    let mut compileUnits = HashMap::new();
    for (i, (filePath, fileContents)) in files.iter().enumerate() {
        fileMap
            .lock()
            .unwrap()
            .addTestFile((*filePath).to_string(), fileContents);
        compileUnits.insert(i as u64 + 1, StateCompileUnit::new());
    }

    (
        CompilerState {
            parameters,
            compileFiles: fileMap,
            compileUnits: Arc::new(compileUnits),
            translationUnitsFiles: Arc::new((1..2).collect::<HashSet<_>>()),
            moduleHeaderUnitsFiles: Arc::new(HashSet::new()),
            foundErrors: Arc::default(),
        },
        1,
    )
}

fn getToksPreprocessed(files: &[(&'static str, String)]) -> Vec<Result<PreToken, CompileMsg>> {
    let prep = Preprocessor::new(generateFileMap(files));
    prep.map(|x| x.map(|x| x.tokPos.tok)).collect::<Vec<_>>()
}

fn getToksPreprocessedNoWs(files: &[(&'static str, String)]) -> Vec<Result<PreToken, CompileMsg>> {
    let mut res = getToksPreprocessed(files);
    res.retain(|x| {
        x.as_ref().map_or(true, |x| {
            !matches!(
                x,
                PreToken::Whitespace(_)
                    | PreToken::Newline
                    | PreToken::ValidNop
                    | PreToken::EnableMacro(_)
                    | PreToken::DisableMacro(_)
            )
        })
    });
    res
}

fn checkForCorrectEvalOfIfClause(string: &'static str) {
    let info = &[("test", string.to_string() + "\nSUCCESS\n#endif")];
    let tokens = getToksPreprocessedNoWs(info);
    let res = !tokens.iter().any(Result::is_err)
        && tokens.iter().any(|x| {
            if let Ok(PreToken::Ident(ref val)) = x {
                val == "SUCCESS"
            } else {
                false
            }
        });
    assert!(res, "The expression does not yield a trueish value");
}

fn checkForAnyEvalOfIfClause(string: &'static str) {
    let info = &[(
        "test",
        string.to_string() + "\nSUCCESS\n#else\nSUCCESS\n#endif",
    )];
    let tokens = getToksPreprocessedNoWs(info);
    let res = !tokens.iter().any(Result::is_err)
        && tokens.iter().any(|x| {
            if let Ok(PreToken::Ident(ref val)) = x {
                val == "SUCCESS"
            } else {
                false
            }
        });
    assert!(res, "The expression does not yield a trueish value");
}

fn checkForBorkenEvalOfIfClause(string: &'static str) {
    let info = &[("test", string.to_string() + "\nSUCCESS\n#endif")];
    let tokens = getToksPreprocessedNoWs(info);
    let res = tokens.iter().any(Result::is_err);
    log::debug!("Tokens: {:?}", tokens);
    assert!(res, "The expression does not yield an error");
}

#[test]
fn simpleCase() {
    checkForCorrectEvalOfIfClause(r##"#if 1"##);
}

#[test]
fn checkElse() {
    checkForCorrectEvalOfIfClause(
        r##"
#if 0
#else
"##,
    );
}

#[test]
fn checkElif() {
    checkForCorrectEvalOfIfClause(
        r##"
#if 0
#elif 1
"##,
    );
}

#[test]
fn checkIfdef() {
    checkForCorrectEvalOfIfClause(
        r##"
#define TEST
#ifdef TEST
"##,
    );
}

#[test]
fn checkDefined1() {
    checkForCorrectEvalOfIfClause(
        r##"
#define TEST
#if defined ( TEST ^)
"##,
    );
}

#[test]
fn checkDefined2() {
    checkForCorrectEvalOfIfClause(
        r##"
#define TEST
#if defined TEST
"##,
    );
}

#[test]
fn checkDefined3() {
    checkForBorkenEvalOfIfClause(
        r##"
#if defined
"##,
    );
}

#[test]
fn checkDefined4() {
    checkForBorkenEvalOfIfClause(
        r##"
#if defined(
"##,
    );
}

#[test]
fn checkDefined5() {
    checkForCorrectEvalOfIfClause(
        r##"
        #define L defined(L)
        #if L
"##,
    );
}

#[test]
fn checkDefined6() {
    checkForCorrectEvalOfIfClause(
        r##"
        #define L(defined) defined(L)
        #if L(defined)
"##,
    );
}

#[test]
fn checkSum() {
    checkForCorrectEvalOfIfClause(
        r##"
        #if 1+2+3+4+5
"##,
    );
}

#[test]
fn checkParen1() {
    checkForCorrectEvalOfIfClause(
        r##"
        #if (1+1)+1*5 == 7
"##,
    );
}

#[test]
fn checkStuff() {
    checkForCorrectEvalOfIfClause(
        r##"
        #if __cplusplus / 100 >= 2011
"##,
    );
}

#[test]
fn checkHasInclude() {
    checkForAnyEvalOfIfClause(
        r##"
        #if __has_include(<iostream>)
"##,
    );
}

#[test]
fn checkBrokenParen() {
    checkForBorkenEvalOfIfClause(
        r##"
#if (1+1
"##,
    );
}

#[test]
fn checkBrokenOp() {
    checkForBorkenEvalOfIfClause(
        r##"
#if 1+
"##,
    );
}

#[test]
fn checkValidOp() {
    checkForCorrectEvalOfIfClause(
        r##"
#if -1
"##,
    );
}
