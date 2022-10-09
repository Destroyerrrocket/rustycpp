//! Finds the module related operations, translates them to custom tokens, and
//! generates the module dependency tree.
mod dependency_dfs;
mod dependency_interpreter;
pub mod dependency_iterator;
mod dependency_parser;
pub mod generate;
pub mod structs;
