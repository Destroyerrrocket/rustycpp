use std::sync::{Arc, Mutex};

use crate::preprocessor::pretoken::PreToken;
use crate::preprocessor::Preprocessor;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::structs::CompileMsg;

use test_log::test;

fn generateFileMap(
    files: &[(&'static str, String)],
) -> (Arc<Parameters>, Arc<Mutex<FileMap>>, &'static str) {
    let testFile = files.first().unwrap().0;
    let mut parameters = Parameters::new();
    let fileMap = Arc::new(Mutex::new(FileMap::new(Arc::new(Parameters::new()))));
    for (filePath, fileContents) in files {
        fileMap
            .lock()
            .unwrap()
            .addTestFile((*filePath).to_string(), (*fileContents).clone());
        parameters.translationUnits.push((*filePath).to_string());
    }
    return (Arc::new(parameters), fileMap, testFile);
}

fn getToksPreprocessed(files: &[(&'static str, String)]) -> Vec<Result<PreToken, CompileMsg>> {
    let prep = Preprocessor::new(generateFileMap(files));
    return prep.map(|x| x.map(|x| x.tokPos.tok)).collect::<Vec<_>>();
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
    return res;
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

fn checkForBorkenEvalOfIfClause(string: &'static str) {
    let info = &[("test", string.to_string() + "\nSUCCESS\n#endif")];
    let tokens = getToksPreprocessedNoWs(info);
    let res = tokens.iter().any(Result::is_err);
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
