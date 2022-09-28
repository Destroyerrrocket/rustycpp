#![feature(
    option_result_contains,
    iter_collect_into,
    is_some_with,
    unwrap_infallible,
    new_uninit,
    arbitrary_enum_discriminant
)]
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
    clippy::empty_structs_with_brackets
)]
#![allow(
    non_snake_case,
    dead_code,
    clippy::needless_return,
    clippy::redundant_else,
    clippy::manual_assert,
    clippy::needless_pass_by_value
)]
// These ones should be re-enabled, and possibly selectively disabled
#![allow(clippy::too_many_lines)]

mod compiler;
mod grammars;
mod preprocessor;
mod test;
mod utils;

use clap::Parser;
use compiler::Compiler;
use std::fs;
use utils::filemap::FileMap;

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
    let filecontents: String = match fs::read_to_string(&args.files) {
        Ok(it) => it,
        Err(err) => {
            log::error!(
                "Could not open & read {file}. Error: {error}",
                file = args.files,
                error = err
            );
            return;
        }
    };

    let mut compileFiles = FileMap::new();
    for line in filecontents.lines() {
        let filename = std::path::Path::new(line);
        if !filename
            .extension()
            .map_or(false, |ext| !ext.eq_ignore_ascii_case(".cpp"))
        {
            log::error!("Unsuported file type: {file}", file = line);
            return;
        }
        compileFiles.getAddFile(line);
    }

    let mut compiler = Compiler::new(compileFiles);
    compiler.print_preprocessor();
}
