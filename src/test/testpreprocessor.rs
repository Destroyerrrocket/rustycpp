use std::sync::{Arc, Mutex};

use crate::{filemap::FileMap, preprocessor::Preprocessor, utils::pretoken::PreToken};

fn generateFileMap(files: &'static [(&str, &str)]) -> (Arc<Mutex<FileMap>>, &'static str) {
    let testFile = files.first().unwrap().0;
    let fileMap = Arc::new(Mutex::new(FileMap::new()));
    for (filePath, fileContents) in files {
        fileMap
            .lock()
            .unwrap()
            .addTestFile(filePath.to_string(), fileContents.to_string());
    }
    return (fileMap, testFile);
}

fn getToksPreprocessed(files: &'static [(&str, &str)]) -> Vec<PreToken> {
    let prep = Preprocessor::new(generateFileMap(files));
    return prep
        .map(|x| x.unwrap())
        .map(|x| x.tokPos.tok)
        .collect::<Vec<PreToken>>();
}

fn getToksPreprocessedNoWs(files: &'static [(&str, &str)]) -> Vec<PreToken> {
    let mut res = getToksPreprocessed(files);
    res.retain(|x| !matches!(x, PreToken::Whitespace(_) | PreToken::Newline));
    return res;
}

#[test]
fn testMacroReplacement() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E e\nE\n")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "e");
}

#[test]
fn testMacroDontReplaceOnMissingParen() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E() e\n#define A a\nE A")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 2);
    assert_eq!(toks[0].to_str(), "E");
    assert_eq!(toks[1].to_str(), "a");
}

#[test]
fn testMacroFuncReplace() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E(a) a\nE(hola)")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "hola");
}

#[test]
fn testMacroFuncReplaceRec() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E(a) a\nE(E(E(E(hola))))")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "hola");
}

#[test]
fn testMacroFuncReplaceRecComp() {
    let toks =
        getToksPreprocessedNoWs(&[("test", "#define E(a) a\n#define A(a) a\nE(A(E(A(hola))))")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "hola");
}

#[test]
fn testMacroFuncStringify() {
    let toks =
        getToksPreprocessedNoWs(&[("test", "#define E(a) a\n#define A(a) #a\nE(A(E(A(hola))))")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#""E(A(hola))""#);
}
