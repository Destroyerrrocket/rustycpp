//! Test suite of the compiler
#![allow(missing_docs, clippy::missing_docs_in_private_items)]
#[cfg(test)]
pub mod TestIncluder;
#[cfg(test)]
pub mod TestLexer;
#[cfg(test)]
pub mod TestPreprocessorDefine;
#[cfg(test)]
pub mod TestPreprocessorIf;
#[cfg(test)]
pub mod TestProject;
#[cfg(test)]
pub mod TestSingleFile;
