#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::verbose_file_reads,
    clippy::unneeded_field_pattern,
    clippy::unnecessary_self_imports,
    clippy::string_to_string,
    clippy::if_then_some_else_none,
    clippy::empty_structs_with_brackets,
    //clippy::missing_docs_in_private_items
)]
#![allow(
    dead_code,
    non_snake_case,
    clippy::missing_docs_in_private_items,
    clippy::cargo_common_metadata
)]

pub mod AST;
pub mod ClassRepresentation;
pub mod GenASTNodes;
