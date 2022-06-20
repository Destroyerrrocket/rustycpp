#![feature(option_result_contains)]
#![allow(non_snake_case)]
mod compiler;
mod prelexer;
mod preprocessor;
mod utils;
mod grammars;

use clap::Parser;
use std::{fs::File, io::Read};
use crate::utils::structs::CompileFile;
use compiler::Compiler;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Print step 2 of compilation
    #[clap(short, long, action)]
    print_step2: bool,
    /// Filelist to compile
    #[clap(short, long, value_parser, default_value="")]
    files: String,
}

fn main() {
    let args = Args::parse();
    if args.files.is_empty()
    {
        eprintln!("File list not specified!");
        return;
    }
    let mut file: File = match File::open(&args.files) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Could not open {file}. Error: {error}", file=args.files, error=err.to_string());
            return;
        }
    };
    let mut filecontents : String = String::new();
    if let Err(err) = file.read_to_string(&mut filecontents) {
        eprintln!("Error reading {file}. Error: {error}", file=args.files, error=err.to_string());
        return;
    }

    let mut compile_files = Vec::new();
    for line in filecontents.lines()
    {
        if !line.ends_with(".cpp") {
            eprint!("Unsuported file type: {file}", file=line);
        }
        let mut file: File = match File::open(&line) {
            Ok(it) => it,
            Err(err) => {
                eprintln!("Could not open {file}. Error: {error}", file=line, error=err.to_string());
                return;
            }
        };
        let mut filecontents : String = String::new();
        if let Err(err) = file.read_to_string(&mut filecontents) {
            eprintln!("Error reading {file}. Error: {error}", file=line, error=err.to_string());
            return;
        }
        compile_files.push(CompileFile::new(std::string::String::from(line), filecontents));
    }

    let compiler = Compiler::new(compile_files);
    compiler.print_preprocessor();
    println!("Hello, world!");
}
