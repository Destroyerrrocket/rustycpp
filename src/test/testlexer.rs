use std::sync::{Arc, Mutex};
use std::vec;

use crate::lexer::tokens::{EncodingPrefix, FloatSuffix, IntegerSuffix, Token};
use crate::preprocessor::pretoken::PreToken;
use crate::preprocessor::Preprocessor;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::structs::{CompileMsg, FileTokPos};

use ::f128::{f128, f128_inner};
use test_log::test;

fn generateFileMap(
    files: &[(&'static str, &'static str)],
) -> (Arc<Parameters>, Arc<Mutex<FileMap>>, &'static str) {
    let testFile = files.first().unwrap().0;
    let fileMap = Arc::new(Mutex::new(FileMap::new(Arc::new(Parameters::new()))));
    let mut parameters = Parameters::new();
    for (filePath, fileContents) in files {
        fileMap
            .lock()
            .unwrap()
            .addTestFile((*filePath).to_string(), (*fileContents).to_string());
        parameters.translationUnits.push((*filePath).to_string());
    }
    return (Arc::new(parameters), fileMap, testFile);
}

fn getToksPreprocessed(files: &[(&'static str, &'static str)]) -> Vec<PreToken> {
    let prep = Preprocessor::new(generateFileMap(files));
    return prep
        .filter_map(|x: Result<FileTokPos<PreToken>, CompileMsg>| {
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

fn toTok(text: &'static str) -> Vec<Token> {
    getToksPreprocessedNoWs(&[("test.c", text)])
        .into_iter()
        .map(|x| {
            Token::from_preToken(FileTokPos::new_meta(x))
                .unwrap()
                .tokPos
                .tok
        })
        .collect::<Vec<Token>>()
}

#[test]
fn characterLiteral1() {
    assert_eq!(
        vec![Token::CharacterLiteral(EncodingPrefix::None, 'a')],
        toTok("'a'")
    );
}

#[test]
fn characterLiteralPrefixU() {
    assert_eq!(
        vec![Token::CharacterLiteral(EncodingPrefix::U, 'a')],
        toTok("U'a'")
    );
}

#[test]
fn characterLiteralPrefixu8() {
    assert_eq!(
        vec![Token::CharacterLiteral(EncodingPrefix::u8, 'a')],
        toTok("u8'a'")
    );
}

#[test]
fn characterLiteralPrefixu() {
    assert_eq!(
        vec![Token::CharacterLiteral(EncodingPrefix::u, 'a')],
        toTok("u'a'")
    );
}

#[test]
fn characterLiteralScapeSeq() {
    assert_eq!(
        vec![Token::CharacterLiteral(EncodingPrefix::u, '\'')],
        toTok("u'\\''")
    );
}

#[test]
fn stringLiteral() {
    assert_eq!(
        vec![Token::StringLiteral(
            EncodingPrefix::u,
            "Hello world!".to_string()
        )],
        toTok(r#"u"Hello world!""#)
    );
}

#[test]
fn udStringLiteral() {
    assert_eq!(
        vec![Token::UdStringLiteral(
            EncodingPrefix::u,
            "Hello world!".to_string(),
            "_Formatter".to_string()
        )],
        toTok(r#"u"Hello world!"_Formatter"#)
    );
}

#[test]
fn udCharLiteral() {
    assert_eq!(
        vec![Token::UdCharacterLiteral(
            EncodingPrefix::u,
            'H',
            "_Formatter".to_string()
        )],
        toTok(r#"u'H'_Formatter"#)
    );
}

#[test]
fn stringLiteralQuestionScape() {
    assert_eq!(
        vec![Token::StringLiteral(EncodingPrefix::None, "?".to_string(),)],
        toTok(r#""\?""#)
    );
}

#[test]
fn udStringLiteralAllEasyScapes() {
    assert_eq!(
        vec![Token::UdStringLiteral(
            EncodingPrefix::u,
            "escape:\n\t\x0B\x08\r\x0C\x07\\?'\"".to_string(),
            "_Formatter".to_string()
        )],
        toTok(r#"u"escape:\n\t\v\b\r\f\a\\\?\'\""_Formatter"#)
    );
}

#[test]
fn udStringLiteralOctalScape1() {
    assert_eq!(
        vec![Token::UdStringLiteral(
            EncodingPrefix::u,
            "escape:\x08".to_string(),
            "_Formatter".to_string()
        )],
        toTok(r#"u"escape:\10"_Formatter"#)
    );
}

#[test]
fn udStringLiteralOctalScape2() {
    assert_eq!(
        vec![Token::UdStringLiteral(
            EncodingPrefix::u,
            "escape:\x00".to_string(),
            "_Formatter".to_string()
        )],
        toTok(r#"u"escape:\0"_Formatter"#)
    );
}

#[test]
fn udStringLiteralHexScape1() {
    assert_eq!(
        vec![Token::UdStringLiteral(
            EncodingPrefix::u,
            "escape:\x08".to_string(),
            "_Formatter".to_string()
        )],
        toTok(r#"u"escape:\x08"_Formatter"#)
    );
}

#[test]
fn udStringLiteralHexScape2() {
    assert_eq!(
        vec![Token::UdStringLiteral(
            EncodingPrefix::u,
            "escape:①".to_string(),
            "_Formatter".to_string()
        )],
        toTok(r#"u"escape:\x2460"_Formatter"#)
    );
}

#[test]
fn integerLiteral() {
    assert_eq!(vec![Token::IntegerLiteral(123, vec![],)], toTok(r#"123"#));
}

#[test]
fn integerLiteralSuffix() {
    assert_eq!(
        vec![Token::IntegerLiteral(
            123,
            vec![IntegerSuffix::LongLong, IntegerSuffix::Unsigned],
        )],
        toTok(r#"123ull"#)
    );
}

#[test]
fn integerLiteralHex() {
    assert_eq!(
        vec![Token::IntegerLiteral(
            0x1A23,
            vec![IntegerSuffix::Long, IntegerSuffix::Unsigned],
        )],
        toTok(r#"0x0'1A'23ul"#)
    );
}

#[allow(clippy::unusual_byte_groupings)]
#[test]
fn integerLiteralOct1() {
    assert_eq!(
        vec![Token::IntegerLiteral(
            0b111_111,
            vec![IntegerSuffix::Long, IntegerSuffix::Unsigned],
        )],
        toTok(r#"077ul"#)
    );
}

#[allow(clippy::unusual_byte_groupings)]
#[test]
fn integerLiteralOct2() {
    assert_eq!(
        vec![Token::IntegerLiteral(
            0,
            vec![IntegerSuffix::Long, IntegerSuffix::Unsigned],
        )],
        toTok(r#"0ul"#)
    );
}

#[allow(clippy::unusual_byte_groupings)]
#[test]
fn integerLiteralBinary() {
    assert_eq!(
        vec![Token::IntegerLiteral(
            0b1010_1010,
            vec![IntegerSuffix::Long, IntegerSuffix::Unsigned],
        )],
        toTok(r#"0b1010'1010ul"#)
    );
}

#[allow(clippy::unusual_byte_groupings)]
#[test]
fn udIntegerLiteral() {
    assert_eq!(
        vec![Token::UdIntegerLiteral(
            0b1010_1010,
            vec![IntegerSuffix::Long, IntegerSuffix::Unsigned],
            "Hours".to_string()
        )],
        toTok(r#"0b1010'1010ulHours"#)
    );
}

#[test]
fn floatLiteral1() {
    assert_eq!(
        vec![Token::FloatingPointLiteral(f128!(0.0), FloatSuffix::None)],
        toTok(r#"0.0"#)
    );
}

#[test]
fn floatLiteral2() {
    assert_eq!(
        vec![Token::FloatingPointLiteral(f128!(0.0), FloatSuffix::L)],
        toTok(r#".0L"#)
    );
}

#[test]
fn floatLiteral3() {
    assert_eq!(
        vec![Token::UdFloatingPointLiteral(
            f128!(0.0),
            FloatSuffix::L,
            "Hours".to_string()
        )],
        toTok(r#".0e-13LHours"#)
    );
}

#[test]
fn floatLiteral4() {
    assert_eq!(
        vec![Token::UdFloatingPointLiteral(
            f128!(0.0),
            FloatSuffix::F,
            "Hours".to_string()
        )],
        toTok(r#"0x0.000P-1FHours"#)
    );
}

#[test]
fn boolLiteral0() {
    assert_eq!(vec![Token::BoolLiteral(false)], toTok(r#"false"#));
}

#[test]
fn boolLiteral1() {
    assert_eq!(vec![Token::BoolLiteral(true)], toTok(r#"true"#));
}
