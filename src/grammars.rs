#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
lalrpop_mod!(pub define, "/grammars/define.rs");
pub mod defineast;
use lalrpop_util::lalrpop_mod;
