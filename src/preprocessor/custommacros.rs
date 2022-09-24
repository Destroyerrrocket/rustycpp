#![allow(non_camel_case_types)]

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use crate::grammars::defineast::{DefineAst, IsVariadic};
use crate::utils::pretoken::PreToken;
use crate::utils::structs::{CompileMsg, FilePreTokPos};

use chrono::Local;
use lazy_static::lazy_static;

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
                    expandFunc: &$x::expand,
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
declCMVar! {__cplusplus, |_| "202002L"}
declCMVar! {__DATE__, |_| Local::now().format("%b %e %y")}
declCMVar! {__FILE__, |expandData: &ExpandData| expandData.newToken.file.path().to_string()}
declCMVar! {__LINE__, |expandData: &ExpandData| expandData.newToken.file.getRowColumn(expandData.newToken.tokPos.start).0}
declCMVar! {__STDC_HOSTED__, |_| "1"}
declCMVar! {__STDCPP_DEFAULT_NEW_ALIGNMENT__, |_| "1"}
declCMVar! {__TIME__, |_| Local::now().format("%H:%M:%S")}

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
            __TIME__
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
        self
    }
}
