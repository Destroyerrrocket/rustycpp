//! A C++ compiler (on the works).
//!
//! Please see the readme at [github](https://github.com/Destroyerrrocket/rustycpp)
//! for more information.
//!
#![feature(
    option_result_contains,
    iter_collect_into,
    is_some_and,
    unwrap_infallible,
    new_uninit,
    map_try_insert,
    map_many_mut,
    get_mut_unchecked
)]
#![warn(
    missing_docs,
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
    non_snake_case,
    dead_code,
    clippy::needless_return,
    clippy::redundant_else,
    clippy::manual_assert,
    clippy::needless_pass_by_value,
    clippy::missing_const_for_fn // Bugged
)]
// These ones should be re-enabled, and possibly selectively disabled
#![allow(clippy::too_many_lines)]

mod compiler;
mod grammars;
mod lex;
mod module_tree;
mod parse;
mod preprocessor;
mod utils;

mod test;

use clap::Parser;
use compiler::Compiler;
use utils::compilerstate::CompilerState;
use utils::parameters::Parameters;
use utils::structs::CompileMsg;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[doc(hidden)]
struct Args {
    /// Filelist to compile
    #[clap(short, long)]
    files: String,

    /// Print the module depenedency tree of the provided set of files.
    #[clap(long, value_parser, default_value = "false")]
    printDependencyTree: bool,

    /// Preprocess files and print the result to stdout.
    #[clap(long, value_parser, default_value = "false")]
    preprocess: bool,

    /// Lexify files and print the result to stdout.
    #[clap(long, value_parser, default_value = "false")]
    lexify: bool,
}

/// Wrapper for main, to allow for the use of `?` in main
fn execCompiler(
    parameters: Parameters,
    args: Args,
) -> Result<(), (CompilerState, Vec<CompileMsg>)> {
    let mut compiler = Compiler::new(parameters);
    if args.printDependencyTree {
        compiler.print_dependency_tree()
    } else if args.preprocess {
        compiler.print_preprocessor()
    } else if args.lexify {
        compiler.print_lexer()
    } else {
        compiler.doTheThing()
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    if args.files.is_empty() {
        log::error!("File list not specified!");
        return;
    }

    let parameters = Parameters::new_file(&args.files).unwrap();
    if let Err((compilerState, errors)) = execCompiler(parameters, args) {
        for err in errors {
            err.print(&compilerState.compileFiles);
        }
    }
}
