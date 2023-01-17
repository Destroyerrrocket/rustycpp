use std::collections::HashMap;

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

macro_rules! testProject {
    () => {{
        testProject(function_name!(), file!())
    }};
}

macro_rules! testSuccessfulProject {
    () => {{
        let (asts, errors, compilerState) = testProject!();
        assertErrors(&errors, &compilerState);
        (asts, compilerState)
    }};
}

macro_rules! testUnsuccessfulProject {
    ($($lines:expr),*) => {{
        testProject!()
    }};
}

fn testProject(
    funcName: &str,
    file: &str,
) -> (HashMap<String, AstTu>, Vec<CompileMsg>, CompilerState) {
    let dirTest = std::path::Path::new(file)
        .canonicalize()
        .unwrap()
        .parent()
        .unwrap()
        .join("testProject")
        .join(funcName);

    let fileTest = dirTest.join("compile_list.json");

    println!("{}", fileTest.to_string_lossy());

    let mut parameters = Parameters::new_file(fileTest.to_str().unwrap()).unwrap();
    parameters
        .includeDirs
        .push(dirTest.to_str().unwrap().to_string());
    let mut tmpRes = (HashMap::new(), Vec::new());
    let crashed = Compiler::new(parameters).parsed_tree_test(&mut tmpRes);
    let (ast, errors) = tmpRes;

    if let Err((compilerState, errors)) = crashed.clone() {
        assertErrors(&errors, &compilerState);
        unreachable!();
    }
    (ast, errors, crashed.unwrap())
}

fn assertErrors(errors: &[CompileMsg], state: &CompilerState) {
    errors.iter().for_each(|e| e.print(&state.compileFiles));
    assert!(!errors.iter().any(|e| e.severity() >= CompileMsgKind::Error));
}

fn checkErrors(
    mut errors: Vec<CompileMsg>,
    state: &CompilerState,
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
            .filter(|e| e.severity() > CompileMsgKind::Warning)
            .count()
            == 0
    );
}

#[test]
#[named]
fn simpleModule() {
    let _ = testSuccessfulProject!();
}
