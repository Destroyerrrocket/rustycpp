use std::collections::HashMap;

use multiset::HashMultiSet;

use crate::grammars::defineast::{DefineAst, PreTokenDefine};
use crate::utils::structs::FilePreTokPos;

use super::multilexer::MultiLexer;
use super::pretoken::PreToken;

#[derive(Debug, Clone)]
pub struct ExpandData<'a> {
    pub definitions: &'a HashMap<String, DefineAst>,
    pub disabledMacros: &'a HashMultiSet<String>,
    pub lexer: &'a MultiLexer,
    pub namedArgs: &'a HashMap<String, Vec<FilePreTokPos<PreToken>>>,
    pub variadic: &'a Vec<Vec<FilePreTokPos<PreToken>>>,
    pub astId: &'a String,
    pub replacement: &'a Vec<PreTokenDefine>,
    pub newToken: &'a FilePreTokPos<PreToken>,
    pub expandArg: bool,
}
