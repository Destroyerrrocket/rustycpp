//! Generated antlr grammars
#![allow(
    dead_code,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction
)]

/// Include generated grammar
macro_rules! include_generated {
    ($name:ident) => {
        include!(concat!(
            env!("OUT_DIR"),
            "/generated/",
            stringify!($name),
            ".mod"
        ));
    };
}

include_generated!(macrointconstantexpressionast);
