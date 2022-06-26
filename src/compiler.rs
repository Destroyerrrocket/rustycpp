use std::process::abort;

use crate::prelexer::{PreLexer};
use crate::preprocessor::{*};
use crate::utils::structs::{*};

#[derive(Debug)]
pub struct Compiler {
	compile_files : Vec<CompileFile>,
}

impl Compiler {
	pub fn new(compile_files:  Vec<CompileFile>) -> Compiler {
		Compiler{compile_files}
	}

	pub fn print_step123(&self) -> () {
		for comp_file in &self.compile_files {
			println!("Applying preprocessor lexer to: {}", &comp_file.path());
			for prepro_token in PreLexer::new(&comp_file.content()) {
				println!("{:?}", prepro_token);
			}
		}
	}

	pub fn print_preprocessor(&self) -> () {
		for comp_file in &self.compile_files {
			println!("Applying preprocessor to: {}", &comp_file.path());
			for prepro_token in Preprocessor::new(&comp_file) {
				match prepro_token {
					Ok(tok) => {println!("{}", tok.kind.to_str());}
					Err(err) => {eprintln!("{}", err.to_string()); if err.severity() == CompileMsgKind::FatalError {panic!("Force stop. Unrecoverable error");}}
				}
			}
		}
	}
}