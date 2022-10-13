//! Generated antlr grammars
#![allow(
    dead_code,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    unused_parens
)]

/// Include generated grammar
macro_rules! include_generated {
    ($name:ident) => {
        include!(concat!(
            env!("OUT_DIR"),
            "/generated/",
            stringify!($name),
            ".rs"
        ));
    };
}

include_generated!(macrointconstantexpressionast);
include_generated!(maincpp);
