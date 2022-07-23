#![feature(option_result_contains)]
#![feature(iter_collect_into)]
#![feature(is_some_with)]
#![feature(new_uninit)]
#![feature(unwrap_infallible)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]

mod compiler;
mod filemap;
mod grammars;
mod prelexer;
mod preprocessor;
mod test;
mod utils;

use crate::filemap::FileMap;
use clap::Parser;
use compiler::Compiler;
use std::{fs::File, io::Read};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Print step 2 of compilation
    #[clap(short, long, action)]
    print_step2: bool,
    /// Filelist to compile
    #[clap(short, long, value_parser, default_value = "")]
    files: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    if args.files.is_empty() {
        log::error!("File list not specified!");
        return;
    }
    let mut file: File = match File::open(&args.files) {
        Ok(it) => it,
        Err(err) => {
            log::error!(
                "Could not open {file}. Error: {error}",
                file = args.files,
                error = err
            );
            return;
        }
    };
    let mut filecontents: String = String::new();
    if let Err(err) = file.read_to_string(&mut filecontents) {
        log::error!(
            "Error reading {file}. Error: {error}",
            file = args.files,
            error = err
        );
        return;
    }

    let mut compileFiles = FileMap::new();
    for line in filecontents.lines() {
        if !line.ends_with(".cpp") {
            log::error!("Unsuported file type: {file}", file = line);
            return;
        }
        compileFiles.getAddFile(line);
    }

    let mut compiler = Compiler::new(compileFiles);
    compiler.print_preprocessor();
}
