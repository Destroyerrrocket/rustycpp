use std::{collections::{HashMap}};

use crate::{grammars::{define, defineast::{*}}, utils::{pretoken::{PreToken, PreprocessingOperator}, structs::*, funcs::all_unique_elements, lalrpoplexerwrapper::LalrPopLexerWrapper}, prelexer::PreprocessingToken};

use super::Preprocessor;

impl<'a> Preprocessor<'a> {
	fn parseReplList(currFile: &'a CompileFile, parse: &DefineAst, tokens: Vec<PreToken>, idx: usize) -> Result<Vec<PreTokenDefine>, CompileMsg<'a>> {
		let variadicStr = if let IsVariadic::True(stri) = &parse.variadic {
			stri.as_str()
		} else {""};
		let mut vaOptExpectParen: Vec<i32> = vec![];
		let mut toksPre: Vec<PreTokenDefinePreParse> = tokens.into_iter().map(|tok| {
			match tok {
				PreToken::Ident(s) => {
					if parse.param.is_some_and(|param| param.contains(&s)) {PreTokenDefinePreParse::Arg(s)}
					else if s.as_str() == variadicStr {PreTokenDefinePreParse::VariadicArg}
					else if s.as_str() == "__VA_ARGS__" {PreTokenDefinePreParse::VariadicArg}
					else if s.as_str() == "__VA_OPT__" {
						vaOptExpectParen.push(-1); PreTokenDefinePreParse::VariadicOpt
					}
					else {PreTokenDefinePreParse::Normal(PreToken::Ident(s))}
				}
				PreToken::OperatorPunctuator("(") => {
					if let Some(pars) = vaOptExpectParen.last_mut() {
						*pars += 1;
						if *pars == 0 {
							return PreTokenDefinePreParse::VariadicOptParenL;
						}
					}
					return PreTokenDefinePreParse::Normal(tok);
				}

				PreToken::OperatorPunctuator(")") => {
					let shouldMut = if let Some(pars) = vaOptExpectParen.last_mut() {
						*pars -= 1;
						*pars == -1
					} else {false};
					if shouldMut {
						vaOptExpectParen.pop();
						return PreTokenDefinePreParse::VariadicOptParenR;
					} else {
						return PreTokenDefinePreParse::Normal(tok);
					}
				}

				PreToken::PreprocessingOperator(op) => {
					if op == PreprocessingOperator::Hash {PreTokenDefinePreParse::Hash} else {PreTokenDefinePreParse::HashHash}
				}
				_ => {PreTokenDefinePreParse::Normal(tok)}
			}
		}).collect();


		toksPre.reverse();
		let mut inHashHash = false;
		toksPre.retain(|x| match x {
			PreTokenDefinePreParse::Normal(PreToken::Whitespace(_)) => {!inHashHash}
			PreTokenDefinePreParse::HashHash => {inHashHash = true; true}
			_ => {inHashHash = false; true}
		});
		toksPre.reverse();
		println!("DEBUG: {:?}", toksPre);
		let lexer = LalrPopLexerWrapper::new(toksPre.as_slice());
		let res = define::DefineStmtParser::new().parse(lexer);
		return res.map_err(|err| CompileError::from_at(format!("Parse error: {:?}", err), currFile, idx, None));
	}

	fn getAstMacro(currFile: &'a CompileFile, initialToken: &PreprocessingToken, tokens: Vec<PreprocessingToken>) -> Result<DefineAst, CompileMsg<'a>> {
		let mut res = DefineAst{ id: "".to_string(), param: None, variadic: IsVariadic::False, replacement: vec![] };
		let mut ntok = tokens.into_iter().skip_while(|tok| tok.kind.isWhitespace());
		res.id = if let Some(tokId) = ntok.next() {
			if let PreToken::Ident(idStr) = &tokId.kind {
				idStr.to_string()
			} else {return Err(CompileError::from_preTo("Expected identifier, instead found: ".to_string() + tokId.kind.to_str(), currFile, &tokId));}
		} else {return Err(CompileError::from_preTo("Expected identifier in macro definition", currFile, &initialToken));};
		let mut rlt;
		if let Some(tokLParen) = ntok.next() {
			match &tokLParen.kind {
				PreToken::Whitespace(_) => // We have a replacement macro
				{
					rlt = ntok.skip_while(|tok| tok.kind.isWhitespace()).collect::<Vec<PreprocessingToken>>();
				}
				PreToken::OperatorPunctuator("(") => { // We have a function macro
					let mut paren = ntok.by_ref().take_while(|tok| !matches!(tok.kind, PreToken::OperatorPunctuator(")"))).filter(|tok| !tok.kind.isWhitespace());
					res.param = Some(vec![]);
					loop {
						let paramData = paren.by_ref().take_while(|x| !matches!(x.kind, PreToken::OperatorPunctuator(","))).collect::<Vec<PreprocessingToken>>();
						let identParamTokens = paramData.iter().map(|x| &x.kind).collect::<Vec<&PreToken>>();
						match identParamTokens.as_slice() {
							[] => {break;}
							[PreToken::OperatorPunctuator("...")] => {res.variadic = IsVariadic::True("".to_string()); break;}
							[PreToken::Ident(id), PreToken::OperatorPunctuator("...")] |
							[PreToken::OperatorPunctuator("..."), PreToken::Ident(id)] => {res.variadic = IsVariadic::True(id.to_string()); break;}
							[PreToken::Ident(id)] => {res.param.as_mut().unwrap().push(id.to_string());}
							_ => {return Err(CompileError::from_preTo(format!("Non-valid parameter to function-like macro: {:?}", identParamTokens), currFile, paramData.first().unwrap()));}
						}
					}
					if let Some(prepro) = paren.next() {return Err(CompileError::from_preTo("Unparsable extra token in macro parameter", currFile, &prepro));}
					if !all_unique_elements(res.param.as_ref().unwrap()) {return Err(CompileError::from_preTo("Repeated identifiers in parameters", currFile, &tokLParen));}

					rlt = ntok.skip_while(|tok| tok.kind.isWhitespace()).collect::<Vec<PreprocessingToken>>();
				}
				e => { // We have a replacement macro, but the first token is not whitespace. This is technically an extension
					rlt = vec![tokLParen];
					ntok.collect_into(&mut rlt);
				}
			}

			let idx = rlt.first().map(|x| x.originalDiff).unwrap_or(0);
			let rlttok = rlt.into_iter().map(|x| x.kind).collect::<Vec<PreToken>>();
			let mut rl = Self::parseReplList(&currFile, &res, rlttok, idx)?;
			while rl.last().is_some_and(|tok| matches!(tok, PreTokenDefine::Normal(PreToken::Whitespace(_)))) {rl.pop();}
			if res.variadic == IsVariadic::False {
				if rl.iter().any(|x| matches!(x, PreTokenDefine::VariadicArg))
					{return Err(CompileError::from_preTo("Non-variadic macro can't use __VA_ARGS__", currFile, &initialToken));}
				if rl.iter().any(|x| matches!(x, PreTokenDefine::VariadicOpt(_)))
					{return Err(CompileError::from_preTo("Non-variadic macro can't use __VA_OPT__", currFile, &initialToken));}
			}

			res.replacement = rl;
		}
		return Ok(res);
	}

	fn defineMacroImpl(definitions: &mut HashMap<String, DefineAst>, currFile: &'a CompileFile, vecPrepro: Vec<PreprocessingToken>, preToken: &PreprocessingToken) -> Result<(), CompileMsg<'a>> {
		let def = {match Self::getAstMacro(currFile, &preToken, vecPrepro) {
			Err(err) => {return Err(err);}
			Ok(def) => {def}
		}};

		match definitions.get_mut(&def.id) {
			Some(other) => {
					*other = def;
					return Err(CompileWarning::from_preTo("Redefining macro", currFile, &preToken));
			}
			None => {definitions.insert(def.id.to_string(), def);}
		}
		return Ok(());
	}

	pub fn defineMacro(&'a mut self, preToken: PreprocessingToken) -> () {
		let fileData = self.files.last_mut().unwrap();
		let vecPrepro = Iterator::take_while(&mut fileData.lexer, |pre| pre.kind != PreToken::Newline).collect::<Vec<PreprocessingToken>>();
		let currFile = fileData.compFile;

		{
			let res = Self::defineMacroImpl(&mut self.definitions, currFile, vecPrepro, &preToken);
			match res {
				Err(err) => {self.errors.push_back(err);}
				Ok(_) => {}
			};
		}
		println!("Macros:");
		for (_,defi) in self.definitions.iter() {
			println!("{:?}", defi);
		}
	}
}