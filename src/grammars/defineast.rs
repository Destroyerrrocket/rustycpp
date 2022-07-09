use std::fmt::Debug;

use crate::utils::{pretoken::PreToken, structs::FilePreTokPos};

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
    Arg(String),
    VariadicArg,
    Hash(Box<PreTokenDefine>),
    HashHash(Box<PreTokenDefine>, Box<PreTokenDefine>),
    VariadicOpt(Vec<Box<PreTokenDefine>>),
}

#[derive(Debug, Clone)]
pub struct DefineAst {
    pub id: String,
    pub param: Option<Vec<String>>,
    pub variadic: IsVariadic,
    pub replacement: Vec<PreTokenDefine>,
}
