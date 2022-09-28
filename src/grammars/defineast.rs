use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::Send;

use crate::preprocessor::pretoken::PreToken;
use crate::preprocessor::structs::ExpandData;
use crate::utils::structs::CompileMsg;

use crate::utils::structs::FilePreTokPos;

#[derive(Debug, Clone)]
pub enum IsVariadic {
    True(String),
    False,
}

impl PartialEq for IsVariadic {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::False, Self::False) | (Self::True(_), Self::True(_))
        )
    }
}

#[derive(Debug, Clone)]
pub enum PreTokenDefinePreParse {
    Normal(PreToken),
    Arg(String),
    Hash,
    HashHash,
    VariadicArg,
    VariadicOpt,
    VariadicOptParenL,
    VariadicOptParenR,
}

#[derive(Debug, Clone)]
pub enum PreTokenDefine {
    Normal(FilePreTokPos<PreToken>),
    Arg(FilePreTokPos<String>),
    VariadicArg(FilePreTokPos<()>),
    Hash(FilePreTokPos<()>, Vec<PreTokenDefine>),
    HashHash(FilePreTokPos<()>, Vec<PreTokenDefine>, Vec<PreTokenDefine>),
    VariadicOpt(FilePreTokPos<()>, Vec<PreTokenDefine>),
}
type DefineExpansionFunc =
    dyn Fn(ExpandData) -> Result<VecDeque<FilePreTokPos<PreToken>>, CompileMsg>;
#[derive(Clone)]
pub struct DefineAst {
    pub id: String,
    pub param: Option<Vec<String>>,
    pub variadic: IsVariadic,
    pub replacement: Vec<PreTokenDefine>,
    pub expandFunc: &'static DefineExpansionFunc,
}

impl Debug for DefineAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefineAst")
            .field("id", &self.id)
            .field("param", &self.param)
            .field("variadic", &self.variadic)
            .field("replacement", &self.replacement)
            .finish()
    }
}
unsafe impl Send for DefineAst {}
