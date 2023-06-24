use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::Preprocessor::Preprocessor;
use crate::Preprocessor::Pretoken::PreToken;
use crate::Utils::CompilerState::CompilerState;
use crate::Utils::FileMap::FileMap;
use crate::Utils::Parameters::Parameters;
use crate::Utils::StateCompileUnit::StateCompileUnit;
use crate::Utils::Structs::{CompileMsg, FileTokPos};

use test_log::test;

fn generateFileMap(files: &[(&'static str, &'static str)]) -> (CompilerState, u64) {
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

fn getToksPreprocessed(files: &[(&'static str, &'static str)]) -> Vec<PreToken> {
    let f = generateFileMap(files);
    let prep = Preprocessor::new(f.clone());
    prep.filter_map(|x: Result<FileTokPos<PreToken>, CompileMsg>| {
        x.map_or_else(
            |err| {
                log::error!("{}", err.to_string(&f.0.compileFiles));
                None
            },
            Some,
        )
    })
    .map(|x| x.tokPos.tok)
    .collect::<Vec<PreToken>>()
}

fn getErrsPreprocessed(files: &[(&'static str, &'static str)]) -> Vec<CompileMsg> {
    let prep = Preprocessor::new(generateFileMap(files));
    prep.filter_map(Result::err).collect::<Vec<CompileMsg>>()
}

fn getToksPreprocessedNoWs(files: &[(&'static str, &'static str)]) -> Vec<PreToken> {
    let mut res = getToksPreprocessed(files);
    res.retain(|x| {
        !matches!(
            x,
            PreToken::Whitespace(_)
                | PreToken::Newline
                | PreToken::ValidNop
                | PreToken::EnableMacro(_)
                | PreToken::DisableMacro(_)
        )
    });
    res
}

fn toksToString(toks: &[PreToken]) -> String {
    let mut res = String::new();
    for s in toks.iter().map /*TODO: FILTER NOPS?*/ (PreToken::to_str) {
        res.push_str(s);
        res.push(' ');
    }
    res
}

fn preprocessAndStringify(string: &'static str) -> String {
    let info = &[("test", string)];
    toksToString(&getToksPreprocessedNoWs(info))
}
#[test]
fn testHeaderOpening1() {
    let toks = getToksPreprocessedNoWs(&[("test", "#include <header.h>\n")]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "SUCCESS");
}

#[test]
fn testHeaderOpening2() {
    let toks = getToksPreprocessedNoWs(&[("test", r#"#include "header.h"\n"#)]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "SUCCESS");
}

#[test]
fn testHeaderOpeningMacro() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        r#"
#define HEADERIZE(arg) <arg.h>
#include HEADERIZE(header)
"#,
    )]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "SUCCESS");
}

#[test]
fn testHeaderOpeningTwice() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        r#"
#include <header.h>
#include <header.h>
"#,
    )]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 2);
    assert_eq!(toks[0].to_str(), "SUCCESS");
    assert_eq!(toks[1].to_str(), "SUCCESS");
}
