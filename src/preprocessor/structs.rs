//! Structs used only in the preprocessor

use std::collections::HashMap;

use multiset::HashMultiSet;

use crate::grammars::defineast::{DefineAst, PreTokenDefine};
use crate::utils::structs::FileTokPos;

use super::multilexer::MultiLexer;
use super::pretoken::PreToken;

#[derive(Debug, Clone)]
/// When a macro is expanded, this struct is passed to the expand functions so
/// they have all the necessary context.
pub struct ExpandData<'a> {
    /// Current definitions of the preprocessor
    pub definitions: &'a HashMap<String, DefineAst>,
    /// Disabled macros at this point
    pub disabledMacros: &'a HashMultiSet<String>,
    /// The lexer. Not guaranteed to be the same as the one in the preprocessor. Can't be modified.
    pub lexer: &'a MultiLexer,
    /// Conent of the named args, if macro was function-like
    pub namedArgs: &'a HashMap<String, Vec<FileTokPos<PreToken>>>,
    /// Conent of the variadic args, if macro was function-like
    pub variadic: &'a Vec<Vec<FileTokPos<PreToken>>>,
    /// Macro name
    pub astId: &'a String,
    /// The replacement list of the macro.
    pub replacement: &'a Vec<PreTokenDefine>,
    /// The new token that generated this instantiation
    pub newToken: &'a FileTokPos<PreToken>,
    /// Should the macro expand its arguments? Can be used for operator #, so it
    /// can disable expansion of arguments. That way, when this is encountered:
    /// #define A a
    /// #define B(b) #b
    /// B(A)
    /// The result is "A", not "a".
    pub expandArg: bool,
}
