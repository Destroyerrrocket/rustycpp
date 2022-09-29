extern crate lalrpop;

use std::env;
use std::fs;
use std::io::Error;
use std::process::Command;

fn main() {
    lalrpop::process_root().unwrap();

    let grammars = vec!["macrointconstantexpressionast"];
    let additional_args = vec![&["-visitor"][..]];
    let antlr_path = "./antlr4-4.8-2-SNAPSHOT-complete.jar";

    for (grammar, arg) in grammars.into_iter().zip(additional_args) {
        //ignoring error because we do not need to run anything when deploying to crates.io
        gen_for_grammar(grammar, antlr_path, arg).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=antlr4-4.8-2-SNAPSHOT-complete.jar");
}

fn gen_for_grammar(
    grammar_file_name: &str,
    antlr_path: &str,
    additional_arg: &[&str],
) -> Result<(), Box<Error>> {
    let input = env::current_dir().unwrap().join("src/grammars");
    let file_name = grammar_file_name.to_owned() + ".g4";

    Command::new("java")
        .current_dir(&input)
        .arg("-jar")
        .arg(antlr_path)
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg("generated")
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;

    let filepath = input
        .join(&("generated/".to_owned() + grammar_file_name + "parser.rs").to_lowercase())
        .canonicalize()
        .unwrap();

    let s = fs::read_to_string(&filepath).unwrap().replace(
        &(grammar_file_name.to_owned() + "ParserContext"),
        &(grammar_file_name.to_owned() + "Context"),
    );
    fs::remove_file(&filepath)?;
    fs::write(&filepath, s)?;

    println!("cargo:rerun-if-changed=src/grammars/{}", file_name);
    Ok(())
}
