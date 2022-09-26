#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
lalrpop_mod!(pub define, "/grammars/define.rs");
lalrpop_mod!(pub macrointconstantexpression, "/grammars/macrointconstantexpression.rs");

pub mod defineast;
pub mod macrointconstantexpressionast;
use lalrpop_util::lalrpop_mod;
