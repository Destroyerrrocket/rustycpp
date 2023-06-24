//! Component responsible of running the preprocessor on an input file.
//!
//! The preporcessing step of a compilation is composed of a first step of
//! tokenization done by the [prelexer], followed by the preprocessor, mostly
//! driven by the [driver], and it's submodules.

pub mod Driver;
pub mod Multilexer;
pub mod Prelexer;
pub mod Pretoken;
pub mod Structs;

pub use Driver::*;
