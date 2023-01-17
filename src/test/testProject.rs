use std::{collections::HashMap, path::Path};

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
        testProject!()
            .into_iter()
            .inspect(|(_, errors, compilerState)| assertErrors(&errors, &compilerState))
            .map(|(asts, _, compilerState)| (asts, compilerState))
            .collect::<Vec<_>>()
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
) -> Vec<(HashMap<String, AstTu>, Vec<CompileMsg>, CompilerState)> {
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
    let compute = move |parameters| {
        let mut tmpRes = (HashMap::new(), Vec::new());
        let crashed = Compiler::new(parameters).parsed_tree_test(&mut tmpRes);
        let (ast, errors) = tmpRes;

        if let Err((compilerState, errors)) = crashed.clone() {
            assertErrors(&errors, &compilerState);
            unimplemented!();
        }
        (ast, errors, crashed.unwrap())
    };
    let mut res = vec![compute(parameters.clone())];
    parameters.threadNum = Some(8);
    res.push(compute(parameters.clone()));
    parameters.threadNum = Some(4);
    res.push(compute(parameters.clone()));
    parameters.threadNum = Some(2);
    res.push(compute(parameters.clone()));
    parameters.threadNum = Some(1);
    res.push(compute(parameters));
    return res;
}

fn assertErrors(errors: &[CompileMsg], state: &CompilerState) {
    errors.iter().for_each(|e| e.print(&state.compileFiles));
    assert!(!errors.iter().any(|e| e.severity() >= CompileMsgKind::Error));
}

fn checkErrors(
    mut errors: Vec<CompileMsg>,
    state: &CompilerState,
    expectedErrors: &[(usize, String, bool, CompileMsgKind)],
) {
    errors.iter().for_each(|e| e.print(&state.compileFiles));

    for (expectedLocation, path, optional, expectedErrorType) in expectedErrors {
        let mut compileFiles = state.compileFiles.lock().unwrap();
        let pos = errors.iter().position(|e| {
            let (file, at, _) = e.loc();
            at.is_some_and(|at| {
                let fileArc = compileFiles.getOpenedFile(file);
                fileArc.getRowColumn(at).0 == *expectedLocation
                    && Path::new(fileArc.path()).file_name().unwrap()
                        == Path::new(path).file_name().unwrap()
            }) && e.severity() == *expectedErrorType
        });
        if let Some(pos) = pos {
            errors.remove(pos);
            continue;
        }
        if *optional {
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

macro_rules! e {
    ($n:literal, $path:literal, true) => {
        ($n, $path.to_string(), true, CompileMsgKind::Error)
    };
    ($n:literal, $path:literal) => {
        ($n, $path.to_string(), false, CompileMsgKind::Error)
    };
}

macro_rules! w {
    ($n:literal, $path:literal, true) => {
        ($n, $path.to_string(), true, CompileMsgKind::Warning)
    };
    ($n:literal, $path:literal) => {
        ($n, $path.to_string(), false, CompileMsgKind::Warning)
    };
}

#[test]
#[named]
fn simpleModule() {
    let _ = testSuccessfulProject!();
}

#[test]
#[named]
fn headerModuleErr1() {
    testUnsuccessfulProject!()
        .into_iter()
        .for_each(|(_, e, s)| {
            checkErrors(e, &s, &[e!(1, "foo.hpp", true), e!(1, "bar.hpp", true)]);
        });
}
