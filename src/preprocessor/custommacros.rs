#![allow(non_camel_case_types, clippy::string_to_string)]

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use crate::grammars::defineast::{DefineAst, IsVariadic};
use crate::prelexer::PreLexer;
use crate::utils::pretoken::PreToken;
use crate::utils::structs::{CompileError, CompileMsg, FilePreTokPos};

use chrono::Local;
use lazy_static::lazy_static;

use super::multilexer::MultiLexer;
use super::structs::ExpandData;
use super::Preprocessor;

trait CustomMacro {
    fn macroInfo() -> DefineAst;
    fn expand(expandData: ExpandData) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg>;
}

macro_rules! declCMVar {
    ($x:ident, $expand:expr) => {
        struct $x {}
        impl CustomMacro for $x {
            fn macroInfo() -> DefineAst {
                DefineAst {
                    id: stringify!($x).to_string(),
                    param: None,
                    variadic: IsVariadic::False,
                    replacement: vec![],
                    expandFunc: &Self::expand,
                }
            }
            fn expand(
                expandData: ExpandData,
            ) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
                let expanded = $expand;
                let mut res = VecDeque::new();
                res.push_back(FilePreTokPos::new_meta_c(
                    PreToken::DisableMacro(stringify!($x).to_string()),
                    &expandData.newToken,
                ));
                res.push_back(FilePreTokPos::new_meta_c(
                    PreToken::RawStringLiteral((expanded)(&expandData).to_string()),
                    &expandData.newToken,
                ));
                res.push_back(FilePreTokPos::new_meta_c(
                    PreToken::EnableMacro(stringify!($x).to_string()),
                    &expandData.newToken,
                ));
                Ok(res)
            }
        }
    };
}
declCMVar! {__DATE__, |_| Local::now().format("%b %e %y")}
declCMVar! {__FILE__, |expandData: &ExpandData| expandData.newToken.file.path().to_string()}
declCMVar! {__LINE__, |expandData: &ExpandData| expandData.newToken.file.getRowColumn(expandData.newToken.tokPos.start).0}
declCMVar! {__STDC_HOSTED__, |_| "1"}
declCMVar! {__STDCPP_DEFAULT_NEW_ALIGNMENT__, |_| "1"}
declCMVar! {__TIME__, |_| Local::now().format("%H:%M:%S")}

struct __cplusplus;

impl CustomMacro for __cplusplus {
    fn macroInfo() -> DefineAst {
        DefineAst {
            id: stringify!(__cplusplus).to_string(),
            param: None,
            variadic: IsVariadic::False,
            replacement: vec![],
            expandFunc: &__cplusplus::expand,
        }
    }

    fn expand(expandData: ExpandData) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut res = VecDeque::new();
        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::DisableMacro(stringify!(__cplusplus).to_string()),
            expandData.newToken,
        ));

        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::PPNumber("202002L".to_owned()),
            expandData.newToken,
        ));
        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::EnableMacro(stringify!(__cplusplus).to_string()),
            expandData.newToken,
        ));
        Ok(res)
    }
}
struct __has_include;

impl __has_include {
    fn checkForInclude(toks: &VecDeque<FilePreTokPos<PreToken>>) -> Option<String> {
        let mut res = String::new();
        for s in toks.iter().map(|x| x.tokPos.tok.to_str().to_owned()) {
            res.push_str(&s);
        }
        let mut lexer = PreLexer::new(res);
        lexer.expectHeader();
        if let Some(PreToken::HeaderName(pathWithSurroundingChars)) = lexer.next().map(|x| x.tok) {
            let mut chars = pathWithSurroundingChars.chars();
            chars.next();
            chars.next_back();
            Some(chars.as_str().to_owned())
        } else {
            None
        }
    }
}

impl CustomMacro for __has_include {
    fn macroInfo() -> DefineAst {
        DefineAst {
            id: stringify!(__has_include).to_string(),
            param: Some(vec![]),
            variadic: IsVariadic::True(String::new()),
            replacement: vec![],
            expandFunc: &Self::expand,
        }
    }

    fn expand(expandData: ExpandData) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut res = VecDeque::new();
        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::DisableMacro(stringify!(__has_include).to_string()),
            expandData.newToken,
        ));

        let mut path = String::new();
        let mut result = VecDeque::new();
        for posVariadic in 0..expandData.variadic.len() {
            for v in &expandData.variadic[posVariadic] {
                result.push_back(v.clone());
            }
            if posVariadic + 1 != expandData.variadic.len() {
                result.push_back(FilePreTokPos::new_meta_c(
                    PreToken::OperatorPunctuator(","),
                    expandData.newToken,
                ));
            }
        }

        if result.is_empty() {
            return Err(CompileError::from_preTo(
                "The empty path can't be opened",
                expandData.newToken,
            ));
        }

        if let Some(newPath) = Self::checkForInclude(&result) {
            path = newPath;
        }

        let mut paramLexer = MultiLexer::new_def(expandData.lexer.fileMapping());
        paramLexer.pushTokensDec(result);
        let toks = Preprocessor::expandASequenceOfTokens(
            paramLexer,
            expandData.definitions,
            expandData.disabledMacros,
        )?;

        if let Some(newPath) = Self::checkForInclude(&toks) {
            path = newPath;
        } else {
            for s in toks.into_iter().map(|t| t.tokPos.tok.to_str().to_owned()) {
                path.push_str(&s);
            }
        }

        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::PPNumber(if expandData.lexer.hasFileAccess(&path) {
                "1".to_owned()
            } else {
                "0".to_owned()
            }),
            expandData.newToken,
        ));
        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::EnableMacro(stringify!(__has_include).to_string()),
            expandData.newToken,
        ));
        Ok(res)
    }
}

struct __has_cpp_attribute;

impl CustomMacro for __has_cpp_attribute {
    fn macroInfo() -> DefineAst {
        DefineAst {
            id: stringify!(__has_include).to_string(),
            param: Some(vec![]),
            variadic: IsVariadic::True(String::new()),
            replacement: vec![],
            expandFunc: &__has_include::expand,
        }
    }

    fn expand(expandData: ExpandData) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg> {
        let mut res = VecDeque::new();
        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::DisableMacro(stringify!(__has_include).to_string()),
            expandData.newToken,
        ));

        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::PPNumber("0".to_owned()),
            expandData.newToken,
        ));
        res.push_back(FilePreTokPos::new_meta_c(
            PreToken::EnableMacro(stringify!(__has_include).to_string()),
            expandData.newToken,
        ));
        Ok(res)
    }
}

macro_rules! registerMacro_ {
    ($hashMap:ident) => {};
    ($hashMap:ident, $x:ty) => {{
        let info = <$x>::macroInfo();
        $hashMap.insert(info.id.clone(), info);
    }};
    ($hashMap:ident, $x:ty, $($others:ty),*) => {
        registerMacro_!($hashMap, $x);
        registerMacro_!($hashMap, $($others),*);
    };
}
macro_rules! registerMacro {
    ($($o:ty),*) => {{
        let mut hashMap = HashMap::new();
        registerMacro_!(hashMap, $( $o ),*);
        hashMap
    }};
}
impl Preprocessor {
    fn generateCustomMacro() -> HashMap<String, DefineAst> {
        registerMacro!(
            __cplusplus,
            __DATE__,
            __FILE__,
            __LINE__,
            __STDC_HOSTED__,
            __STDCPP_DEFAULT_NEW_ALIGNMENT__,
            __TIME__,
            __has_include,
            __has_cpp_attribute
        )
    }

    pub fn initCustomMacros(mut self) -> Self {
        lazy_static! {
            static ref CUSTOM_MACROS: Mutex<HashMap<String, DefineAst>> =
                Mutex::new(Preprocessor::generateCustomMacro());
        }
        self.definitions.extend(
            CUSTOM_MACROS
                .lock()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.disabledMacros.insert("__has_include".to_string());
        self.disabledMacros
            .insert("__has_cpp_attribute".to_string());
        self
    }
}
