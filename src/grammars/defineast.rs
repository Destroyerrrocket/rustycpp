//! Related classes used during the parsing of a #define expression

use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::Send;

use crate::preprocessor::pretoken::PreToken;
use crate::preprocessor::structs::ExpandData;
use crate::utils::structs::CompileMsg;

use crate::utils::structs::FileTokPos;

#[derive(Debug, Clone)]
/// Is the macro variadic? Supports named variadics.
pub enum IsVariadic {
    /// The macro is variadic, with a custom name if String is not empty
    True(String),
    /// The macro is not variadic
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
/// Before the macro definition is parsed, some tokens need to be distinguished
pub enum PreTokenDefinePreParse {
    /// Normal token
    Normal(PreToken),
    /// A token that is one of the arguments of the macro
    Arg(String),
    /// # operator
    Hash,
    /// ## operator
    HashHash,
    /// __VA_ARG__
    VariadicArg,
    /// __VA_OPT__
    VariadicOpt,
    /// Left paren of a previous __VA_OPT__
    VariadicOptParenL,
    /// Right paren of a previous __VA_OPT__
    VariadicOptParenR,
}

#[derive(Debug, Clone)]
/// Resulting AST of the macro definition. The definition will be a list of these tokens
pub enum PreTokenDefine {
    /// Normal token
    Normal(FileTokPos<PreToken>),
    /// Argument of the macro
    Arg(FileTokPos<String>),
    /// Variadic argument of the macro
    VariadicArg(FileTokPos<()>),
    /// A # operator. It contains the tokens it will stringify
    Hash(FileTokPos<()>, Vec<PreTokenDefine>),
    /// A ## operator. It contains the tokens it will concatenate
    HashHash(FileTokPos<()>, Vec<PreTokenDefine>, Vec<PreTokenDefine>),
    /// A __VA_OPT__ operator. It contains the tokens It will place if the variadic argument is not empty
    VariadicOpt(FileTokPos<()>, Vec<PreTokenDefine>),
}

#[doc(hidden)]
type DefineExpansionFunc =
    dyn Fn(&ExpandData) -> Result<VecDeque<FileTokPos<PreToken>>, CompileMsg>;
#[derive(Clone)]
/// A macro definition, with all the needed data
pub struct DefineAst {
    /// Name of the macro
    pub id: String,
    /// Parmeters of the macro, if any
    pub param: Option<Vec<String>>,
    /// Whether the macro is variadic
    pub variadic: IsVariadic,
    /// The parsed replacement list, used in macro subsitution
    pub replacement: Vec<PreTokenDefine>,
    /// Function used for expansion. Intended to aid in the implementation of custom macros
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
