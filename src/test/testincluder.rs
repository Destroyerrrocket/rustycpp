use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::preprocessor::pretoken::PreToken;
use crate::preprocessor::Preprocessor;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::structs::{CompileMsg, FilePreTokPos};

use test_log::test;

fn generateFileMap(
    files: &[(&'static str, &'static str)],
) -> (Arc<Parameters>, Arc<Mutex<FileMap>>, &'static str) {
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
    let testFile = files.first().unwrap().0;
    let fileMap = Arc::new(Mutex::new(FileMap::new(parameters.clone())));
    for (filePath, fileContents) in files {
        fileMap
            .lock()
            .unwrap()
            .addTestFile((*filePath).to_string(), (*fileContents).to_string());
    }
    return (parameters, fileMap, testFile);
}

fn getToksPreprocessed(files: &[(&'static str, &'static str)]) -> Vec<PreToken> {
    let prep = Preprocessor::new(generateFileMap(files));
    return prep
        .filter_map(|x: Result<FilePreTokPos<PreToken>, CompileMsg>| {
            x.map_or_else(
                |err| {
                    log::error!("{}", err.to_string());
                    None
                },
                Some,
            )
        })
        .map(|x| x.tokPos.tok)
        .collect::<Vec<PreToken>>();
}

fn getErrsPreprocessed(files: &[(&'static str, &'static str)]) -> Vec<CompileMsg> {
    let prep = Preprocessor::new(generateFileMap(files));
    return prep.filter_map(Result::err).collect::<Vec<CompileMsg>>();
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
    return res;
}

fn toksToString(toks: &[PreToken]) -> String {
    let mut res = String::new();
    for s in toks.iter().map /*TODO: FILTER NOPS?*/ (PreToken::to_str) {
        res.push_str(s);
        res.push(' ');
    }
    return res;
}

fn preprocessAndStringify(string: &'static str) -> String {
    let info = &[("test", string)];
    toksToString(&getToksPreprocessedNoWs(info))
}
#[test]
fn testHeaderOpening1() {
    let toks = getToksPreprocessedNoWs(&[("test", "#include <header.h>\n")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "SUCCESS");
}

#[test]
fn testHeaderOpening2() {
    let toks = getToksPreprocessedNoWs(&[("test", r#"#include "header.h"\n"#)]);
    println!("{:?}", toks);
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
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "SUCCESS");
}