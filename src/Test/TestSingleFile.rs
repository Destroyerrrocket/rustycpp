use std::collections::HashMap;

use ::function_name::named;
use test_log::test;

use crate::{
    debugTree,
    Ast::{Common::AstTu, Common::CommonAst},
    Compiler::Compiler,
    Utils::{
        CompilerState::CompilerState,
        Parameters::Parameters,
        Structs::{CompileMsg, CompileMsgKind},
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
        assertErrors(&errors, &compilerState);
        ast.into_iter().next().unwrap().1
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
    let stateCompiler = Compiler::new(params).parsed_tree_test(&mut tmpRes);
    let (ast, errors) = tmpRes;

    (ast, errors, stateCompiler)
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

macro_rules! e {
    ($n:literal) => {
        ($n, CompileMsgKind::Error)
    };
}

macro_rules! w {
    ($n:literal) => {
        ($n, CompileMsgKind::Warning)
    };
}

macro_rules! assert_tree_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = ($a, $b);
        if a != b {
            let (a, b) = (a.to_string(), b.to_string());
            println!("left:\n{a}");
            println!("right:\n{b}");
            assert!(false, "assertion failed: `left == right` trees");
        }
    }};
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
    checkErrors(e, &s, &[e!(2)]);
}

#[test]
#[named]
fn parsesModuleError2() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(2)]);
}

#[test]
#[named]
fn parsesModuleError3() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(2)]);
}

#[test]
#[named]
fn parsesModuleError4() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(1)]);
}

#[test]
#[named]
fn parsesModuleError5() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(1)]);
}

#[test]
#[named]
fn parsesAttrError1() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(1), e!(1), e!(1), e!(1)]);
}

#[test]
#[named]
fn parsesAttrError2() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(1)]);
}

#[test]
#[named]
fn parsesAttrDecl() {
    let ast = testSuccessfulFile!();
    assert_tree_eq!(
        ast.getDebugNode(),
        debugTree!(
            "AstTuStruct",
            (
                "AstDeclEmptyStruct",
                ("flags: "),
                ("AstAttribute", ("AstAttributeCXXRustyCppUnusedStruct")),
                (
                    "AstAttribute",
                    ("AstAttributeCXXRustyCppUnusedStruct"),
                    ("AstAttributeCXXRustyCppUnusedStruct")
                ),
                ("AstAttribute", ("AstAttributeCXXRustyCppUnusedStruct")),
                ("AstAttribute")
            )
        )
    );
}

#[test]
#[named]
fn parsesAttrDeclError1() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(1)]);
}

#[test]
#[named]
fn parsesAttrDeclError2() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(1), e!(2), e!(3), e!(4), e!(5), e!(6)]);
}

#[test]
#[named]
fn parsesNamedNamespace() {
    let ast = testSuccessfulFile!();
    assert_tree_eq!(
        ast.getDebugNode(),
        debugTree!(
            "AstTuStruct",
            (
                "AstDeclNamespaceStruct",
                ("name: A"),
                ("isInline: false"),
                (
                    "AstDeclNamespaceStruct",
                    ("name: B"),
                    ("isInline: false"),
                    (
                        "AstDeclNamespaceStruct",
                        ("name: C"),
                        ("isInline: false"),
                        ("flags: ")
                    ),
                    ("flags: ")
                ),
                (
                    "AstDeclNamespaceStruct",
                    ("name: C"),
                    ("isInline: false"),
                    ("flags: ")
                ),
                (
                    "AstDeclNamespaceStruct",
                    ("name: C"),
                    ("isInline: false"),
                    ("flags: ")
                ),
                (
                    "AstDeclNamespaceStruct",
                    ("name: D"),
                    ("isInline: true"),
                    (
                        "AstDeclNamespaceStruct",
                        ("name: E"),
                        ("isInline: false"),
                        ("flags: ")
                    ),
                    ("flags: ")
                ),
                ("flags: ")
            )
        )
    );
}

#[test]
#[named]
fn parsesNamedNamespaceError1() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(2), e!(3), e!(3)]);
}

#[test]
#[named]
fn parses__rustycpp__enum() {
    let ast = testSuccessfulFile!();
    assert_tree_eq!(
        ast.getDebugNode(),
        debugTree!(
            "AstTuStruct",
            (
                "AstDeclNamespaceStruct",
                ("name: Enum"),
                ("isInline: false"),
                ("AstDeclCustomRustyCppEnumStruct", ("name: A"), ("flags: ")),
                ("flags: ")
            )
        )
    );
}

#[test]
#[named]
fn parses__rustycpp__enumError1() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(1), e!(2), e!(3), e!(5), e!(5)]);
}

#[test]
#[named]
fn parsesAsmDecl() {
    let ast = testSuccessfulFile!();
    assert_tree_eq!(
        ast.getDebugNode(),
        debugTree!(
            "AstTuStruct",
            ("AstDeclAsmStruct", ("asm: hello"), ("flags: "))
        )
    );
}

#[test]
#[named]
fn parsesAsmDeclError1() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(1), e!(2), w!(3), e!(4), e!(5), e!(6)]);
}

#[test]
#[named]
fn unqualifiedNameResolutionGlobal() {
    let _ = testSuccessfulFile!();
}

#[test]
#[named]
fn unqualifiedNameResolutionGlobalErr1() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(13), e!(14), e!(15), e!(16), e!(17)]);
}

#[test]
#[named]
fn unqualifiedNameResolutionNamespace() {
    let _ = testSuccessfulFile!();
}

#[test]
#[named]
fn qualifiedNameResolution() {
    let _ = testSuccessfulFile!();
}

#[test]
#[named]
fn parsesUsingNamespaceDirective() {
    let _ = testSuccessfulFile!();
}

#[test]
#[named]
fn parsesUsingNamespaceDirectiveError1() {
    let (_, e, s) = testUnsuccessfulFile!();
    checkErrors(e, &s, &[e!(4), e!(5), e!(6), e!(7)]);
}
