use std::{collections::HashMap, ops::Index};

use ::function_name::named;
use test_log::test;

use crate::{
    ast::Tu::AstTu,
    compiler::Compiler,
    utils::{
        compilerstate::CompilerState,
        parameters::Parameters,
        structs::{CompileMsg, CompileMsgKind},
    },
};

macro_rules! testSingleFile {
    () => {{
        testSingleFile(function_name!(), file!())
    }};
}

macro_rules! testSuccessfulFile {
    () => {{
        let (ast, errors, compilerState) = testSingleFile!();
        assertErrors(errors, compilerState);
        ast
    }};
}

macro_rules! testUnsuccessfulFile {
    ($($lines:expr),*) => {{
        testSingleFile!()
    }};
}

fn testSingleFile(
    funcName: &str,
    file: &str,
) -> (HashMap<String, AstTu>, Vec<CompileMsg>, CompilerState) {
    let fileTest = std::path::Path::new(file)
        .canonicalize()
        .unwrap()
        .parent()
        .unwrap()
        .join("testSingleFile")
        .join(funcName.to_owned() + ".cpp");
    let fileTest2 = fileTest.to_str().unwrap().to_string();
    println!("{fileTest2}");
    assert!(fileTest.is_file());
    let fileTest = fileTest.to_str().unwrap().to_string();
    let mut params = Parameters::new();
    params.translationUnits.push(fileTest);
    let mut tmpRes = (HashMap::new(), Vec::new());
    let crashed = Compiler::new(params).parsed_tree_test(&mut tmpRes);
    let (ast, errors) = tmpRes;

    if let Err((compilerState, errors)) = crashed.clone() {
        assertErrors(errors, compilerState);
        unreachable!();
    }
    return (ast, errors, crashed.unwrap());
}

fn assertErrors(errors: Vec<CompileMsg>, state: CompilerState) {
    errors.iter().for_each(|e| e.print(&state.compileFiles));
    assert!(!errors.iter().any(|e| e.severity() >= CompileMsgKind::Error));
}

fn checkErrors(
    mut errors: Vec<CompileMsg>,
    state: CompilerState,
    expectedErrors: &[(usize, CompileMsgKind)],
) {
    errors.iter().for_each(|e| e.print(&state.compileFiles));

    for (expectedLocation, expectedErrorType) in expectedErrors {
        let mut compileFiles = state.compileFiles.lock().unwrap();
        let pos = errors.iter().position(|e| {
            let (file, at, _) = e.loc();
            at.is_some_and(|at| {
                compileFiles.getOpenedFile(file).getRowColumn(at).0 == *expectedLocation
            }) && e.severity() == *expectedErrorType
        });
        if let Some(pos) = pos {
            errors.remove(pos);
            continue;
        }
        panic!("Expected error not found");
    }

    assert!(
        errors
            .into_iter()
            .filter(|e| e.severity() <= CompileMsgKind::Warning)
            .count()
            == 0
    );
}

macro_rules! e {
    ($n:literal) => {
        ($n, CompileMsgKind::Error)
    };
}

#[test]
#[named]
fn parsesEmpty() {
    let _ = testSuccessfulFile!();
}

#[test]
#[named]
fn parsesModule() {
    let _ = testSuccessfulFile!();
}

#[test]
#[named]
fn parsesModuleError1() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, s, &[e!(1)]);
}

#[test]
#[named]
fn parsesModuleError2() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, s, &[e!(1)]);
}

#[test]
#[named]
fn parsesModuleError3() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, s, &[e!(1)]);
}
