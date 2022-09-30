//! All the grammars used by the compiler
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
lalrpop_mod!(pub define, "/grammars/define.rs");

pub mod defineast;
pub mod generated;
pub mod macrointconstantexpressionast;
use lalrpop_util::lalrpop_mod;
