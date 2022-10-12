extern crate lalrpop;

use std::env;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    lalrpop::Configuration::new()
        .process_dir(
            (*current_dir().unwrap().as_path())
                .join("src")
                .to_str()
                .unwrap(),
        )
        .unwrap();

    let grammars = vec!["macrointconstantexpressionast", "mainCpp"];
    let additional_args = vec![&["-visitor"][..], &["-visitor"][..]];
    let antlr_path = "./antlr4-4.8-2-SNAPSHOT-complete.jar";

    for (grammar, arg) in grammars.into_iter().zip(additional_args) {
        //ignoring error because we do not need to run anything when deploying to crates.io
        gen_for_parser_grammar(grammar, antlr_path, arg).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=antlr4-4.8-2-SNAPSHOT-complete.jar");
}

fn gen_for_parser_grammar(
    grammar_file_name: &str,
    antlr_path: &str,
    additional_arg: &[&str],
) -> Result<(), Box<Error>> {
    let input = env::current_dir().unwrap().join("src/grammars");
    let file_name = grammar_file_name.to_owned() + ".g4";

    let generated_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("generated");

    Command::new("java")
        .current_dir(&input)
        .arg("-jar")
        .arg(antlr_path)
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg(&generated_path)
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;

    let filepath = &generated_path
        .join((grammar_file_name.to_owned() + "parser.rs").to_lowercase())
        .canonicalize()
        .unwrap();

    let s = fs::read_to_string(filepath).unwrap().replace(
        &(grammar_file_name.to_owned() + "ParserContext"),
        &(grammar_file_name.to_owned() + "Context"),
    );
    fs::remove_file(filepath)?;
    fs::write(filepath, s)?;

    let mut f = File::create(
        generated_path
            .join(grammar_file_name.to_owned().to_lowercase())
            .with_extension("mod"),
    )
    .unwrap();

    // Workarround in order to import the generated parser
    write!(
        f,
        r##"#
[path = "{path}/{base}listener.rs"]
pub mod {base}listener;
#[path = "{path}/{base}parser.rs"]
pub mod {base}parser;
#[path = "{path}/{base}visitor.rs"]
pub mod {base}visitor;
pub mod {base} {{
    pub use super::{base}listener::*;
    pub use super::{base}parser::*;
    pub use super::{base}visitor::*;
}}
"##,
        path = generated_path.to_str().unwrap(),
        base = grammar_file_name.to_owned().to_lowercase(),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=src/grammars/{}", file_name);
    Ok(())
}
