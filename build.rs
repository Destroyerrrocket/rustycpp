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
    clippy::multiple_crate_versions,
    non_snake_case,
    clippy::missing_panics_doc
)]

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("hello.rs");

    fs::write(
        dest_path,
        codegen::GenASTNodes::generateFile(&codegen::AST::getAST()),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=codegen/mod.rs");
    println!("cargo:rerun-if-changed=codegen/ClassRepresentation.rs");
    println!("cargo:rerun-if-changed=codegen/AST.rs");
    println!("cargo:rerun-if-changed=codegen/ASTNodes.rs");
}
