//! All the grammars used by the compiler
#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items
)]
lalrpop_mod!(pub define, "/grammars/define.rs");

pub mod defineast;
pub mod generated;
pub mod macrointconstantexpressionast;
use lalrpop_util::lalrpop_mod;
pub mod mainCpp;
