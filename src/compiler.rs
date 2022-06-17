pub mod compiler {
    use crate::prelexer::prelexer::{PreLexer};

	#[derive(Debug)]
	pub struct CompileFile {
		path: String,
		content: String,
	}

	impl CompileFile {
		pub fn new(path: String, content: String) -> CompileFile {
			CompileFile {
				path,
				content,
			}
		}
	}

	#[derive(Debug)]
	pub struct Compiler {
		compile_files : Vec<CompileFile>,
	}

	impl Compiler {
		pub fn new(mut files:  Vec<CompileFile>) -> Compiler {
			for file in &mut files {
				file.content = file.content.replace("\r\n", "\n");
			}
			Compiler{compile_files: files}
		}

		pub fn print_step123(&self) -> () {
			for comp_file in &self.compile_files {
				println!("Applying preprocessor lexer to: {}", &comp_file.path);
				for prepro_token in PreLexer::new(&comp_file.content) {
					println!("{:?}", prepro_token);
				}
			}
		}
	}
}