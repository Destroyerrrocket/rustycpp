use std::collections::{VecDeque, HashMap};

use crate::{utils::{pretoken::*, structs::*}, prelexer::{PreLexer, PreprocessingToken}, grammars::{defineast::{*}}};

#[derive(Debug)]
struct FileLexer<'a> {
	pub compFile: &'a CompileFile,
	pub lexer: PreLexer<'a>,
}

#[derive(Debug, PartialEq)]
enum ScopeStatus {
	SuccessBlock,
	FailureBlock,
	AlreadySucceededBlock,
}

mod defineparse;


#[derive(Debug)]
pub struct Preprocessor<'a> {
	files: Vec<FileLexer<'a>>,
	generated: VecDeque<PreprocessingToken>,
	errors: VecDeque<CompileMsg<'a>>,
	scope: Vec<ScopeStatus>,
	definitions: HashMap<String, DefineAst>,
	atStartLine: bool,
}

impl<'a> Preprocessor<'a> {
	pub fn new(file: &CompileFile)->Preprocessor {
		Preprocessor{
			files: vec!(FileLexer{
				compFile: file,
				lexer: PreLexer::new(file.content()),
				}),
			generated: VecDeque::new(),
			errors: VecDeque::new(),
			scope: vec!(),
			definitions: HashMap::new(),
			atStartLine: true,
		}
	}

	fn currFile(&self) -> &FileLexer {self.files.last().unwrap()}
	fn tokenLocCurrStr(&self, preToken: &PreprocessingToken) -> String {self.currFile().compFile.getLocStr(preToken.originalDiff)}

	fn undefineMacro(&mut self, preToken: PreprocessingToken) -> () {
		let vecPrepro = Iterator::take_while(&mut self.files.last_mut().unwrap().lexer, |pre| pre.kind != PreToken::Newline).collect::<Vec<PreprocessingToken>>();
		let currFile = self.files.last().unwrap().compFile;
		match vecPrepro.into_iter().find(|e| !e.kind.isWhitespace()) {
			None => {self.errors.push_back(CompileError::from_preTo("Expected an identifier to undefine", &currFile, &preToken));}
			Some(e) => {match e.kind {
				PreToken::Ident(id) => {
					if let None = self.definitions.remove(&id) {
						self.errors.push_back(CompileError::from_preTo(format!("Macro {} is not defined when reached", id), &currFile, &preToken));
					}
				}
				_ => {self.errors.push_back(CompileError::from_preTo(format!("Expected an identifier, found: {}", e.kind.to_str()), &currFile, &preToken));}
			}}
		}
		println!("Macros:");
		for (_,defi) in self.definitions.iter() {
			println!("{:?}", defi);
		}
		return;
	}

	fn includeFile(&mut self, file: Option<String>) -> () {
		todo!("Implement including");
	}

	fn consumeMacroInclude(&mut self, PreToken: PreprocessingToken) -> Option<String> {
		todo!("Implement header extraction");
	}

	fn consumeMacroDef(&mut self, PreToken: PreprocessingToken) -> Option<String> {
		let currFile = self.files.last_mut().unwrap();
		let identStr;
		loop {
			let inIdent = currFile.lexer.next();
			match inIdent {
				None => {return None;}
				Some(ident) => {match ident.kind {
					PreToken::Ident(str) => {identStr = str; break;}
					PreToken::Whitespace(_) => {continue;}
					PreToken::Newline => {return None;}
					_ => {self.reachNl(); return None;}
				}}
			}
		};
		self.reachNl();
		return Some(identStr);
	}

	fn reachNl(&mut self) -> () {
		let currFile = self.files.last_mut().unwrap();
		loop {
			let inIdent = currFile.lexer.next();
			match inIdent {
				None => {return;}
				Some(ident) => {match ident.kind {
					PreToken::Newline => {return;}
					_ => {}
				}}
			}
		};
	}
	fn consumeMacroExpr(&mut self, PreToken: PreprocessingToken) -> () {
		todo!();
	}

	fn evalIfScope(&self, tree: ()) -> bool {
		todo!();
	}

	fn evalIfDef(&self, def: Option<String>) -> bool {
		if let Some(macroName) = def {
			return self.definitions.contains_key(&macroName);
		}
		return false;
	}

	fn preprocessorDirective(&'a mut self, PreToken: PreprocessingToken) -> () {
		let currFile = self.files.last_mut().unwrap();
		let operation;
		let enabledBlock = match self.scope.last() {
			Some(ScopeStatus::SuccessBlock) |
			None => true,
			_ => false,
		};
		loop {
			match currFile.lexer.next() {
				None => {
					return;
				}
				Some(tok) => {
					match tok.kind {
						PreToken::Newline => {
							return;
						}
						PreToken::Whitespace(_) => {}
						_ => {
							operation = tok;
							break;
						}
					}
				}
			}
		}
		if enabledBlock {
			match operation.kind.to_str() {
				"include" => {currFile.lexer.expectHeader(); let t = self.consumeMacroInclude(operation); self.includeFile(t);}
				"define" => {self.defineMacro(operation);}
				"undef" => {self.undefineMacro(operation);}
				"if" => {let t = self.consumeMacroExpr(operation); let t2 = if self.evalIfScope(t) == true {ScopeStatus::SuccessBlock} else {ScopeStatus::FailureBlock}; self.scope.push(t2);}
				"ifdef" => {let t = {self.consumeMacroDef(operation)};let t2 = if self.evalIfDef(t) == true {ScopeStatus::SuccessBlock} else {ScopeStatus::FailureBlock}; self.scope.push(t2);}
				"ifndef" => {let t = self.consumeMacroDef(operation); let t2 = if self.evalIfDef(t) == false {ScopeStatus::SuccessBlock} else {ScopeStatus::FailureBlock}; self.scope.push(t2);}
				"elif" |
				"else" => {
					if let Some(scope) = self.scope.last_mut() {
						*scope = ScopeStatus::AlreadySucceededBlock;
						self.reachNl(); // TODO: Check empty in else
					} else {
						self.errors.push_back(CompileError::from_preTo("Missmatched preprocessor conditional block", &currFile.compFile, &operation));
					}
				}
				"pragma" => {
					self.errors.push_back(CompileError::from_preTo("LMAO, you really expected me to implement this now XD. No worries, we'll get there :D", &currFile.compFile, &operation));
					self.reachNl();
				}
				"endif" => {
					if self.scope.is_empty() {
						self.errors.push_back(CompileError::from_preTo("Missmatched preprocessor conditional block", &currFile.compFile, &operation));
					} else {self.scope.pop();}
					self.reachNl(); // TODO: Check empty
				}
				_ => {
					self.errors.push_back(CompileError::from_preTo("I do not know this preprocessing expression yet! I'm learning though :)", &currFile.compFile, &operation));
					self.reachNl();
				}
			}
		} else if &ScopeStatus::FailureBlock == self.scope.last().unwrap() {
			match operation.kind.to_str() {
				"if" | "ifdef" | "ifndef" => {
					self.scope.push(ScopeStatus::AlreadySucceededBlock);
				}
				"elif" => {
					let macroExpr = self.consumeMacroExpr(operation);
					if self.evalIfScope(macroExpr) == true {
						let scope = self.scope.last_mut().unwrap();
						*scope = ScopeStatus::SuccessBlock
					}
				}
				"else" => {
					let scope = self.scope.last_mut().unwrap();
					*scope = ScopeStatus::SuccessBlock;
					self.reachNl(); // TODO: Check empty
				}
				"endif" => {
					self.reachNl(); // TODO: Check empty
					self.scope.pop();
				}
				_ => {self.reachNl();}
			}
		} else if &ScopeStatus::AlreadySucceededBlock == self.scope.last().unwrap() {
			match operation.kind.to_str() {
				"if" | "ifdef" | "ifndef" => {
					self.reachNl();
					self.scope.push(ScopeStatus::AlreadySucceededBlock);
				}
				"endif" => {
					self.reachNl(); // TODO: Check empty
					self.scope.pop();
				}
				_ => {self.reachNl();}
			}
		}
	}

	fn consume(&'a mut self, newToken: PreprocessingToken) -> () {
		loop {
			match self.scope.last() {
				Some(ScopeStatus::SuccessBlock) |
				None => {
					if self.atStartLine {
						match newToken.kind {
							PreToken::Whitespace(_) |
							PreToken::Newline => {self.generated.push_back(newToken); break;}
							PreToken::PreprocessingOperator(PreprocessingOperator::Hash) => {
								self.preprocessorDirective(newToken); break;
							}
							_ => {
								self.atStartLine = false;
								continue;
							}
						}
					} else {
						match newToken.kind {
							PreToken::Newline => {
								self.atStartLine = true;
								self.generated.push_back(newToken);
								break;
							}
							PreToken::Ident(ref id) => {
								if let Some(preproMacro) = self.definitions.get(id) {
									// do stuff?
									todo!();
									break;
								}
								self.generated.push_back(newToken); break;
							}
							_ => {
								self.generated.push_back(newToken); break;
							}
						}
					}
				}
				_ => {
					if self.atStartLine {
						match newToken.kind {
							PreToken::Whitespace(_) |
							PreToken::Newline => {break;}
							PreToken::PreprocessingOperator(PreprocessingOperator::Hash) => {
								self.preprocessorDirective(newToken); break;
							}
							_ => {
								self.atStartLine = false;
								break;
							}
						}
					} else {
						match newToken.kind {
							PreToken::Newline => {
								self.atStartLine = true;
								break;
							}
							_ => {break;}
						}
					}
				}
			}
		}
	}
}
impl<'a> Iterator for Preprocessor<'a> {
	type Item = Result<PreprocessingToken, CompileMsg<'a>>;
	fn next(&mut self) -> Option<Self::Item> {
		let this = self as *mut Self;
		unsafe {
		loop {
			if let Some(err) = (*this).errors.pop_front() {
				return Some(Err(err));
			}
			match (*this).generated.pop_front() {
				Some(tok) => {return Some(Ok(tok));}
				None => {
					match (*this).files.last_mut() {
						None => {return None;}
						Some(file) => {
							match file.lexer.next() {
								None => {(*this).files.pop();}
								Some(token) => {
									(*this).consume(token);
								}
							}
						}
					}
				}
			}
		}
		}
	}
}