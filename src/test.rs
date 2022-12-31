//! Test suite of the compiler
#![allow(missing_docs, clippy::missing_docs_in_private_items)]
#[cfg(test)]
pub mod testSingleFile;
#[cfg(test)]
pub mod testincluder;
#[cfg(test)]
pub mod testlexer;
#[cfg(test)]
pub mod testpreprocessordefine;
#[cfg(test)]
pub mod testpreprocessorif;
#[cfg(test)]
pub mod testprocmacro;
