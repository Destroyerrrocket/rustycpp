use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::preprocessor::pretoken::PreToken;
use crate::preprocessor::Preprocessor;
use crate::utils::compilerstate::CompilerState;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::statecompileunit::StateCompileUnit;
use crate::utils::structs::{CompileMsg, FileTokPos};
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
    let compileUnits = Arc::new(Mutex::new(HashMap::new()));
    for (i, (filePath, fileContents)) in files.iter().enumerate() {
        fileMap
            .lock()
            .unwrap()
            .addTestFile((*filePath).to_string(), (*fileContents).to_string());
        compileUnits
            .lock()
            .unwrap()
            .insert(i as u64 + 1, StateCompileUnit::new());
    }

    return (
        CompilerState {
            parameters,
            compileFiles: fileMap,
            compileUnits,
        },
        1,
    );
}

fn getToksPreprocessed(files: &[(&'static str, &'static str)]) -> Vec<PreToken> {
    let f = generateFileMap(files);
    let prep = Preprocessor::new(f.clone());
    return prep
        .filter_map(|x: Result<FileTokPos<PreToken>, CompileMsg>| {
            x.map_or_else(
                |err| {
                    log::error!("{}", err.to_string(&f.0.compileFiles));
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
fn testMacroReplacement() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E e\nE\n")]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "e");
}

#[test]
fn testMacroDontReplaceOnMissingParen() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E() e\n#define A a\nE A")]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 2);
    assert_eq!(toks[0].to_str(), "E");
    assert_eq!(toks[1].to_str(), "a");
}

#[test]
fn testMacroFuncReplace() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E(a) a\nE(hola)")]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "hola");
}

#[test]
fn testMacroFuncReplaceRec() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E(a) a\nE(E(E(E(hola))))")]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "hola");
}

#[test]
fn testMacroFuncReplaceRecComp() {
    let toks =
        getToksPreprocessedNoWs(&[("test", "#define E(a) a\n#define A(a) a\nE(A(E(A(hola))))")]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), "hola");
}

#[test]
fn testMacroFuncVariadic() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E(   hello...  ) hello \nE(a,b,c)")]);
    println!("{toks:?}");
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
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#""E(A(hola))""#);
}

#[test]
fn testMacroFuncStringifyVariable() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        "#define E(...) #   __VA_ARGS__\n\nE(elephant,num,6,class,thing)",
    )]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#""elephant,num,6,class,thing""#);
}

#[test]
fn testMacroFuncStringifyVaOpt() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        "#define E(...) #   __VA_OPT__  ( , __VA_ARGS__  ) \n\nE(elephant ,num,6,class,thing)",
    )]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#"", elephant,num,6,class,thing""#);
}

#[test]
fn testMacroFuncConcat() {
    let toks =
        getToksPreprocessedNoWs(&[("test", "#define E( a , b  )  a  ##   b\nE( =   , =  )")]);
    println!("{toks:?}");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].to_str(), r#"=="#);
}

#[test]
fn testMacroFuncConcatVariable() {
    let toks = getToksPreprocessedNoWs(&[(
        "test",
        "#define E( a , ...  )  a  ##   __VA_OPT__(__VA_ARGS__ nameBoi)\nE( =   , =  )",
    )]);
    println!("{toks:?}");
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
    println!("{toks:?}");
    assert_eq!(toks.len(), 4);
    assert_eq!(toks[0].to_str(), r#"a"#);
    assert_eq!(toks[1].to_str(), r#","#);
    assert_eq!(toks[2].to_str(), r#"b"#);
    assert_eq!(toks[3].to_str(), r#"a"#);
}

#[test]
fn testMacroDisable() {
    let toks = getToksPreprocessedNoWs(&[("test", "#define E( a )  E(a) \nE(E(a))")]);
    println!("{toks:?}");
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
    println!("{toks:?}");
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

#[test]
fn standard_15_6_3_1_1() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define str(s) # s
        #define xstr(s) str(s)
        #define debug(s, t) printf("x" # s "= %d, x" # t "= %s", \
        x ## s, x ## t)
        #define INCFILE(n) vers ## n
        #define glue(a, b) a ## b
        #define xglue(a, b) glue(a, b)
        #define HIGHLOW "hello"
        #define LOW LOW ", world"
        debug(1, 2);
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        printf("x" "1" "= %d, x" "2" "= %s", x1, x2);
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_3_1_2() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define str(s) # s
        #define xstr(s) str(s)
        #define debug(s, t) printf("x" # s "= %d, x" # t "= %s", \
        x ## s, x ## t)
        #define INCFILE(n) vers ## n
        #define glue(a, b) a ## b
        #define xglue(a, b) glue(a, b)
        #define HIGHLOW "hello"
        #define LOW LOW ", world"
        fputs(str(strncmp("abc\0d", "abc", '\4') // this goes away
        == 0) str(: @\n), s);
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        fputs("strncmp(\"abc\\0d\", \"abc\", '\\4') == 0" ": @\n", s);
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_3_1_3() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define str(s) # s
        #define xstr(s) str(s)
        #define debug(s, t) printf("x" # s "= %d, x" # t "= %s", \
        x ## s, x ## t)
        #define INCFILE(n) vers ## n
        #define glue(a, b) a ## b
        #define xglue(a, b) glue(a, b)
        #define HIGHLOW "hello"
        #define LOW LOW ", world"
        xstr(INCFILE(2).h)
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        "vers2.h"
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_3_1_4() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define str(s) # s
        #define xstr(s) str(s)
        #define debug(s, t) printf("x" # s "= %d, x" # t "= %s", \
        x ## s, x ## t)
        #define INCFILE(n) vers ## n
        #define glue(a, b) a ## b
        #define xglue(a, b) glue(a, b)
        #define HIGHLOW "hello"
        #define LOW LOW ", world"
        glue(HIGH, LOW);
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        "hello";
"#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_3_1_5() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define str(s) # s
        #define xstr(s) str(s)
        #define debug(s, t) printf("x" # s "= %d, x" # t "= %s", \
        x ## s, x ## t)
        #define INCFILE(n) vers ## n
        #define glue(a, b) a ## b
        #define xglue(a, b) glue(a, b)
        #define HIGHLOW "hello"
        #define LOW LOW ", world"
        xglue(HIGH, LOW)
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        "hello" ", world"
"#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_3_2() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define hash_hash # ## #
        #define mkstr(a) # a
        #define in_between(a) mkstr(a)
        #define join(c, d) in_between(c hash_hash d)
        char p[] = join(x, y); // equivalent to char p[] = "x ## y";
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        char p[] = "x ## y";
"#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_3_3() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define t(x,y,z) x ## y ## z
        int j[] = { t(1,2,3), t(,4,5), t(6,,7), t(8,9,),
        t(10,,), t(,11,), t(,,12), t(,,) };
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        int j[] = { 123, 45, 67, 89,
            10, 11, 12, };
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_4_1_1() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define x 2
        #define f(a) f(x * (a))
        #define z z[0]
        f(z);
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        f(2 * (z[0]));
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_4_1_2() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define x 3
        #define f(a) f(x * (a))
        #undef x
        #define x 2
        #define g f
        #define z z[0]
        #define h g(~
        #define m(a) a(w)
        #define w 0,1
        #define t(a) a
        #define p() int
        #define q(x) x
        #define r(x,y) x ## y
        #define str(x) # x
        | h 5) & m
        (f)^m(m);
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        | f(2 * (~ 5)) & f(2 * (0,1))^m(0,1);
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_4_1_3() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define x 3
        #define f(a) f(x * (a))
        #undef x
        #define x 2
        #define g f
        #define z z[0]
        #define h g(~
        #define m(a) a(w)
        #define w 0,1
        #define t(a) a
        #define p() int
        #define q(x) x
        #define r(x,y) x ## y
        #define str(x) # x
        g h
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        f f(~
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_4_1_4() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define x 3
        #define f(a) f(x * (a))
        #undef x
        #define x 2
        #define g f
        #define z z[0]
        #define h g(~
        #define m(a) a(w)
        #define w 0,1
        #define t(a) a
        #define p() int
        #define q(x) x
        #define r(x,y) x ## y
        #define str(x) # x
        g() | h 5)
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        f(2 * ()) | f(2 * (~ 5))
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_4_1_5() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define x 3
        #define f(a) f(x * (a))
        #undef x
        #define x 2
        #define g f
        #define z z[0]
        #define h g(~
        #define m(a) a(w)
        #define w 0,1
        #define t(a) a
        #define p() int
        #define q(x) x
        #define r(x,y) x ## y
        #define str(x) # x
        t(t(g)() + t)(1);
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        f(2 * ()) + t(1);
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn standard_15_6_4_1_6() {
    let res = toksToString(&getToksPreprocessedNoWs(&[(
        "test",
        r###"
        #define x 3
        #define f(a) f(x * (a))
        #undef x
        #define x 2
        #define g f
        #define z z[0]
        #define h g(~
        #define m(a) a(w)
        #define w 0,1
        #define t(a) a
        #define p() int
        #define q(x) x
        #define r(x,y) x ## y
        #define str(x) # x
        f(y+1) + f(f(z)) % t(t(g)(0) + t)(1);
        g(x+(3,4)-w) | h 5) & m
        (f)^m(m);
        p() i[q()] = { q(1), r(2,3), r(4,), r(,5), r(,) };
        char c[2][6] = { str(hello), str() };
        "###,
    )]));

    let expected = preprocessAndStringify(
        r#"
        f(2 * (y+1)) + f(2 * (f(2 * (z[0])))) % f(2 * (0)) + t(1);
        f(2 * (2+(3,4)-0,1)) | f(2 * (~ 5)) & f(2 * (0,1))^m(0,1);
        int i[] = { 1, 23, 4, 5, };
        char c[2][6] = { "hello", "" };
        "#,
    );
    assert_eq!(res, expected);
}

#[test]
fn mandatoryDefinedMacros() {
    let macros = vec![
        "__cplusplus",
        "__DATE__",
        "__FILE__",
        "__LINE__",
        "__STDC_HOSTED__",
        "__STDCPP_DEFAULT_NEW_ALIGNMENT__",
        "__TIME__",
    ];
    for m in macros {
        let res = toksToString(&getToksPreprocessedNoWs(&[("test", m)]));
        log::debug!("{}", res);
        assert!(!res.is_empty());
    }
}
