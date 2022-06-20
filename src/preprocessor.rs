use std::{collections::{VecDeque, HashMap}};

use crate::{utils::{pretoken::{PreToken, PreprocessingOperator}, structs::CompileFile, lalrpoplexerwrapper::LalrPopLexerWrapper}, prelexer::{PreLexer, PreprocessingToken}, grammars::{define, defineast::{*}}};

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

#[derive(Debug)]
pub struct Preprocessor<'a> {
	files: Vec<FileLexer<'a>>,
	generated: VecDeque<PreprocessingToken>,
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
			scope: vec!(),
			definitions: HashMap::new(),
			atStartLine: true,
		}
	}

	fn defineMacro(&mut self) -> () {
		let vecPrepro = Iterator::take_while(&mut self.files.last_mut().unwrap().lexer, |pre| pre.kind != PreToken::Newline).collect::<VecDeque<PreprocessingToken>>();
		let lexer = LalrPopLexerWrapper::new(vecPrepro);

		let definition;
		match define::DefineStmtParser::new().parse(lexer) {
			Err(e) => {eprintln!("{:?}", e); panic!("TODO: Error more gracefully");}
			Ok(e) => {definition = e;}
		}
		if let DefineAst::Define(ref ident, ref params, ref variadic, ref replacement) = definition {
			// TODO: CHECK SEMANTICALLY
			if let Some(DefineAst::Define(identOther, paramsOther, variadicOther, replacementOther)) = self.definitions.get_mut(ident) {
				if  (params.is_none() && paramsOther.is_none()) || (
						params.as_ref().unwrap().len() == paramsOther.as_ref().unwrap().len() &&
						variadic == variadicOther
					) {
						*replacementOther = replacement.to_vec();
					} else {
						eprint!("Couldn't replace macro {}", ident);
					}
			} else {
				self.definitions.insert(ident.to_string(), definition);
			}
		}
		println!("Macros:");
		for (_,defi) in self.definitions.iter() {
			println!("{:?}", defi);
		}
	}

	fn undefineMacro(&mut self) -> () {
		let vecPrepro = Iterator::take_while(&mut self.files.last_mut().unwrap().lexer, |pre| pre.kind != PreToken::Newline).collect::<Vec<PreprocessingToken>>();
		match vecPrepro.into_iter().find(|e| if let PreToken::Whitespace(_) = e.kind {false} else if let PreToken::Comment(_) = e.kind {false} else {true}) {
			None => {eprintln!("Expected an identifier to undefine!");}
			Some(e) => {match e.kind {
				PreToken::Ident(id) => {
					if let None = self.definitions.remove(&id) {
						eprintln!("Macro {} is not defined at this point", id);
					}
				}
				_ => {eprintln!("Expected an identifier, found: {}", e.kind.to_str());}
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

	fn consumeMacroInclude(&mut self) -> Option<String> {
		todo!("Implement header extraction");
	}

	fn consumeMacroDef(&mut self) -> Option<String> {
		let currFile = self.files.last_mut().unwrap();
		let identStr;
		loop {
			let inIdent = currFile.lexer.next();
			match inIdent {
				None => {return None;}
				Some(ident) => {match ident.kind {
					PreToken::Ident(str) => {identStr = str; break;}
					PreToken::Whitespace(_) |
					PreToken::Comment(_) => {continue;}
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
	fn consumeMacroExpr(&mut self) -> () {
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

	fn preprocessorDirective(&mut self, PreToken: PreprocessingToken) -> () {
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
				"include" => {currFile.lexer.expectHeader(); let t = self.consumeMacroInclude(); self.includeFile(t);}
				"define" => {self.defineMacro();}
				"undef" => {self.undefineMacro();}
				"if" => {let t = self.consumeMacroExpr(); let t2 = if self.evalIfScope(t) == true {ScopeStatus::SuccessBlock} else {ScopeStatus::FailureBlock}; self.scope.push(t2);}
				"ifdef" => {let t = {self.consumeMacroDef()};let t2 = if self.evalIfDef(t) == true {ScopeStatus::SuccessBlock} else {ScopeStatus::FailureBlock}; self.scope.push(t2);}
				"ifndef" => {let t = self.consumeMacroDef(); let t2 = if self.evalIfDef(t) == false {ScopeStatus::SuccessBlock} else {ScopeStatus::FailureBlock}; self.scope.push(t2);}
				"elif" |
				"else" => {
					if let Some(scope) = self.scope.last_mut() {
						*scope = ScopeStatus::AlreadySucceededBlock;
						self.reachNl(); // TODO: Check empty in else
					} else {
						eprintln!("Missmatched preprocessor conditional block at: {}", currFile.compFile.getLocStr(operation.originalDiff));
					}
				}
				"pragma" => {
					eprintln!("LMAO, you really expected me to implement this now XD. No worries, we'll get there :D at: {}", currFile.compFile.getLocStr(operation.originalDiff));
					self.reachNl();
				}
				"endif" => {
					if self.scope.is_empty() {
						eprintln!("Missmatched preprocessor conditional block at: {}", currFile.compFile.getLocStr(operation.originalDiff));
					} else {self.scope.pop();}
					self.reachNl(); // TODO: Check empty
				}
				_ => {
					eprintln!("I do not know this preprocessing expression yet! I'm learning though :) at: {}", currFile.compFile.getLocStr(operation.originalDiff));
				}
			}
		} else if &ScopeStatus::FailureBlock == self.scope.last().unwrap() {
			match operation.kind.to_str() {
				"if" | "ifdef" | "ifndef" => {
					self.scope.push(ScopeStatus::AlreadySucceededBlock);
				}
				"elif" => {
					let macroExpr = self.consumeMacroExpr();
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
				_ => {}
			}
		}
	}

	fn consume(&mut self, newToken: PreprocessingToken) -> () {
		loop {
			match self.scope.last() {
				Some(ScopeStatus::SuccessBlock) |
				None => {
					if self.atStartLine {
						match newToken.kind {
							PreToken::Whitespace(_) |
							PreToken::Comment(_) |
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
							PreToken::Comment(_) |
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
	type Item = PreprocessingToken;
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.generated.pop_front() {
				Some(tok) => {return Some(tok);}
				None => {
					match self.files.last_mut() {
						None => {return None;}
						Some(file) => {
							match file.lexer.next() {
								None => {self.files.pop();}
								Some(token) => {
									self.consume(token);
								}
							}
						}
					}
				}
			}
		}
	}
}