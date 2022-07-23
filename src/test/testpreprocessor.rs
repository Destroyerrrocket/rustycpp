use std::sync::{Arc, Mutex};

use crate::utils::structs::CompileMsg;
use crate::{filemap::FileMap, preprocessor::Preprocessor, utils::pretoken::PreToken};
use test_log::test;

fn generateFileMap(files: &[(&'static str, &'static str)]) -> (Arc<Mutex<FileMap>>, &'static str) {
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

fn getToksPreprocessed(files: &[(&'static str, &'static str)]) -> Vec<PreToken> {
    let prep = Preprocessor::new(generateFileMap(files));
    return prep
        .filter_map(|x| x.ok())
        .map(|x| x.tokPos.tok)
        .collect::<Vec<PreToken>>();
}

fn getErrsPreprocessed(files: &[(&'static str, &'static str)]) -> Vec<CompileMsg> {
    let prep = Preprocessor::new(generateFileMap(files));
    return prep.filter_map(|x| x.err()).collect::<Vec<CompileMsg>>();
}

fn getToksPreprocessedNoWs(files: &[(&'static str, &'static str)]) -> Vec<PreToken> {
    let mut res = getToksPreprocessed(files);
    res.retain(|x| !matches!(x, PreToken::Whitespace(_) | PreToken::Newline));
    return res;
}

fn toksToString(toks: &[PreToken]) -> String {
    let mut res = String::new();
    for s in toks.iter().map(|x| x.to_str()) {
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
fn testMacroFuncVariadic() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E(   hello...  ) hello \nE(a,b,c)")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 5);
    assert_eq!(toks[0].to_str(), r#"a"#);
    assert_eq!(toks[1].to_str(), r#","#);
    assert_eq!(toks[2].to_str(), r#"b"#);
    assert_eq!(toks[3].to_str(), r#","#);
    assert_eq!(toks[4].to_str(), r#"c"#);
}

#[test]
fn testMacroFuncStringify() {
    let toks =
        getToksPreprocessedNoWs(&[("test", "#define E(a) a\n#define A(a) #a\nE(A(E(A(hola))))")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#""E(A(hola))""#);
}

#[test]
fn testMacroFuncStringifyVariable() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        "#define E(...) #   __VA_ARGS__\n\nE(elephant,num,6,class,thing)",
    )]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#""elephant,num,6,class,thing""#);
}

#[test]
fn testMacroFuncStringifyVaOpt() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        "#define E(...) #   __VA_OPT__  ( , __VA_ARGS__  ) \n\nE(elephant ,num,6,class,thing)",
    )]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#"", elephant,num,6,class,thing""#);
}

#[test]
fn testMacroFuncConcat() {
    let toks =
        getToksPreprocessedNoWs(&[("test", "#define E( a , b  )  a  ##   b\nE( =   , =  )")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#"=="#);
}

#[test]
fn testMacroFuncConcatVariable() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        "#define E( a , ...  )  a  ##   __VA_OPT__(__VA_ARGS__ nameBoi)\nE( =   , =  )",
    )]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 2);
    assert_eq!(toks[0].to_str(), r#"=="#);
    assert_eq!(toks[1].to_str(), r#"nameBoi"#);
}

#[test]
fn testMacroExtensionGccComma() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        "#define E( a , ...  )  a ,  ##   __VA_ARGS__\nE( a ,b)E(a)",
    )]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 4);
    assert_eq!(toks[0].to_str(), r#"a"#);
    assert_eq!(toks[1].to_str(), r#","#);
    assert_eq!(toks[2].to_str(), r#"b"#);
    assert_eq!(toks[3].to_str(), r#"a"#);
}

#[test]
fn testMacroDisable() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E( a )  E(a) \nE(E(a))")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 7);
    assert_eq!(toks[0].to_str(), r#"E"#);
    assert_eq!(toks[1].to_str(), r#"("#);
    assert_eq!(toks[2].to_str(), r#"E"#);
    assert_eq!(toks[3].to_str(), r#"("#);
    assert_eq!(toks[4].to_str(), r#"a"#);
    assert_eq!(toks[5].to_str(), r#")"#);
    assert_eq!(toks[6].to_str(), r#")"#);
}

#[test]
fn testMacroDisable2() {
    let toks =
        getToksPreprocessedNoWs(&[("test", "#define E( a )  A(E(a))\n #define A(a) a\nE(a)")]);
    println!("{:?}", toks);
    assert_eq!(toks.len(), 4);
    assert_eq!(toks[0].to_str(), r#"E"#);
    assert_eq!(toks[1].to_str(), r#"("#);
    assert_eq!(toks[2].to_str(), r#"a"#);
    assert_eq!(toks[3].to_str(), r#")"#);
}

#[test]
fn standard_15_6_1_1() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r"
        #define LPAREN() (
        #define G(Q) 42
        #define F(R, X, ...) __VA_OPT__(G R X) )
        int x = F(LPAREN(), 0, <:-); // replaced by int x = 42;
        ",
    )]));
    assert_eq!(res, preprocessAndStringify(r#"int x=42;"#));
}

#[test]
fn standard_15_6_1_3() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define debug(...) fprintf(stderr, __VA_ARGS__)
        #define showlist(...) puts(#__VA_ARGS__)
        #define report(test, ...) ((test) ? puts(#test) : printf(__VA_ARGS__))
        debug("Flag");
        debug("X = %d\n", x);
        showlist(The first, second, and third items.);
        report(x>y, "x is %d but y is %d", x, y);
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"fprintf ( stderr, "Flag"); fprintf(stderr, "X = %d\n", x);
    puts("The first, second, and third items.");
    ((x>y) ? puts("x>y") : printf("x is %d but y is %d", x, y));
    "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_1_4_0() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define F(...) f(0 __VA_OPT__(,) __VA_ARGS__)
        #define EMP
        F(EMP) // replaced by f(0)
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
f(0)
    "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_1_4_1() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define F(...) f(0 __VA_OPT__(,) __VA_ARGS__)
        #define G(X, ...) f(0, X __VA_OPT__(,) __VA_ARGS__)
        #define SDEF(sname, ...) S sname __VA_OPT__(= { __VA_ARGS__ })
        #define EMP
        F(a, b, c) // replaced by f(0, a, b, c)
        F() // replaced by f(0)
        F(EMP) // replaced by f(0)
        G(a, b, c) // replaced by f(0, a, b, c)
        G(a, ) // replaced by f(0, a)
        G(a) // replaced by f(0, a)
        SDEF(foo); // replaced by S foo;
        SDEF(bar, 1, 2); // replaced by S bar = { 1, 2 };
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
f(0, a, b, c)
f(0)
f(0)
f(0, a, b, c)
f(0, a)
f(0, a)
S foo;
S bar = { 1, 2 };
    "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_1_4_2() {
    let res = getErrsPreprocessed(&[(
        "test",
        r###"
        #define H1(X, ...) X __VA_OPT__(##) __VA_ARGS__ // error: ## may not appear at
        "###,
    )]);
    assert!(!res.is_empty());
}

#[test]
fn standard_15_6_1_4_3() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define H2(X, Y, ...) __VA_OPT__(X ## Y,) __VA_ARGS__
        H2(a, b, c, d)
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
ab, c, d
    "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_1_4_4() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define H3(X, ...) #__VA_OPT__(X##X X##X)
        H3(, 0) // replaced by ""
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
""
    "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_1_4_5() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define H4(X, ...) __VA_OPT__(a X ## X) ## b
        H4(, 1) // replaced by a b
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
a b
    "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_1_4_6() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define H5A(...) __VA_OPT__()/**/__VA_OPT__()
        #define H5B(X) a ## X ## b
        #define H5C(X) H5B(X)
        H5C(H5A()) // replaced by ab
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
ab
    "#,
    );
    assert_eq!(res, expected);
}
