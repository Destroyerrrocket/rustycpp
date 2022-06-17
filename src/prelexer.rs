pub mod prelexer {
	use lazy_regex::{regex, regex_find, regex_captures};
	use logos::Logos;

	#[derive(Clone, Copy, Debug)]
	pub struct LexerFilter {
		since_open_paren_has_include: u64,
		since_preprocessing_operator : u64,
		since_include : u64,
		since_has_include : u64,
		since_import : u64,
	}
	impl LexerFilter {
		fn incr(&mut self) {
			self.since_open_paren_has_include += 1;
			self.since_preprocessing_operator += 1;
			self.since_include += 1;
			self.since_has_include += 1;
			self.since_import += 1;
		}
	}
	// Control chars [ \n\x0B\t\x0C]
	#[derive(Clone, Copy, Debug, PartialEq, Logos)]
	#[logos(extras = LexerFilter)]
	pub enum PreprocessorToken {
		HeaderName,
		#[regex(r"[a-zA-Z_[^\x00-\x7F]][a-zA-Z0-9_[^\x00-\x7F]]*")]
		Ident,
		#[token(r"#")]
		#[token(r"##")]
		#[token(r"%:")]
		#[token(r"%:%:")]
		PreprocessingOperator,
		#[token(r"{")]
		#[token(r"}")]
		#[token(r"[")]
		#[token(r"]")]
		#[token(r"(")]
		#[token(r")")]
		#[token(r"<:")]
		#[token(r":>")]
		#[token(r"<%")]
		#[token(r"%>")]
		#[token(r";")]
		#[token(r":")]
		#[token(r"...")]
		#[token(r"?")]
		#[token(r"::")]
		#[token(r".")]
		#[token(r".*")]
		#[token(r"->")]
		#[token(r"->*")]
		#[token(r"~!")]
		#[token(r"+")]
		#[token(r"-")]
		#[token(r"*")]
		#[token(r"/")]
		#[token(r"%")]
		#[token(r"^")]
		#[token(r"&")]
		#[token(r"|")]
		#[token(r"=")]
		#[token(r"+=")]
		#[token(r"-=")]
		#[token(r"*=")]
		#[token(r"/=")]
		#[token(r"%=")]
		#[token(r"^=")]
		#[token(r"&=")]
		#[token(r"|=")]
		#[token(r"==")]
		#[token(r"!=")]
		#[token(r"<")]
		#[token(r">")]
		#[token(r"<=")]
		#[token(r">=")]
		#[token(r"<=>")]
		#[token(r"&&")]
		#[token(r"||")]
		#[token(r"<<")]
		#[token(r">>")]
		#[token(r"<<=")]
		#[token(r">>=")]
		#[token(r"++")]
		#[token(r"--")]
		#[token(r",")]
		#[token(r"and")]
		#[token(r"or")]
		#[token(r"xor")]
		#[token(r"not")]
		#[token(r"bitand")]
		#[token(r"bitor")]
		#[token(r"compl")]
		#[token(r"and_eq")]
		#[token(r"or_eq")]
		#[token(r"xor_eq")]
		#[token(r"not_eq")]
		OperatorPunctuator,
		#[token(r"alignas")]
		#[token(r"alignof")]
		#[token(r"asm")]
		#[token(r"auto")]
		#[token(r"bool")]
		#[token(r"break")]
		#[token(r"case")]
		#[token(r"catch")]
		#[token(r"char")]
		#[token(r"char8_t")]
		#[token(r"char16_t")]
		#[token(r"char32_t")]
		#[token(r"class")]
		#[token(r"concept")]
		#[token(r"const")]
		#[token(r"consteval")]
		#[token(r"constexpr")]
		#[token(r"constinit")]
		#[token(r"const_cast")]
		#[token(r"continue")]
		#[token(r"co_await")]
		#[token(r"co_return")]
		#[token(r"co_yield")]
		#[token(r"decltype")]
		#[token(r"default")]
		#[token(r"delete")]
		#[token(r"do")]
		#[token(r"double")]
		#[token(r"dynamic_cast")]
		#[token(r"else")]
		#[token(r"enum")]
		#[token(r"explicit")]
		#[token(r"export")]
		#[token(r"extern")]
		#[token(r"false")]
		#[token(r"float")]
		#[token(r"for")]
		#[token(r"friend")]
		#[token(r"goto")]
		#[token(r"if")]
		#[token(r"inline")]
		#[token(r"int")]
		#[token(r"long")]
		#[token(r"mutable")]
		#[token(r"namespace")]
		#[token(r"new")]
		#[token(r"noexcept")]
		#[token(r"nullptr")]
		#[token(r"operator")]
		#[token(r"private")]
		#[token(r"protected")]
		#[token(r"public")]
		#[token(r"register")]
		#[token(r"reinterpret_cast")]
		#[token(r"requires")]
		#[token(r"return")]
		#[token(r"short")]
		#[token(r"signed")]
		#[token(r"sizeof")]
		#[token(r"static")]
		#[token(r"static_assert")]
		#[token(r"static_cast")]
		#[token(r"struct")]
		#[token(r"switch")]
		#[token(r"template")]
		#[token(r"this")]
		#[token(r"thread_local")]
		#[token(r"throw")]
		#[token(r"true")]
		#[token(r"try")]
		#[token(r"typedef")]
		#[token(r"typeid")]
		#[token(r"typename")]
		#[token(r"union")]
		#[token(r"unsigned")]
		#[token(r"using")]
		#[token(r"virtual")]
		#[token(r"void")]
		#[token(r"volatile")]
		#[token(r"wchar_t")]
		#[token(r"while")]
		Keyword,
		#[token("\n")]
		Newline,
		#[regex(r"[\t \x0B\x0C]+")]
		Whitespace,
		#[regex(r"/\*[^\*/]*\*/")]
		Comment,
		/* Lmao, no repetition ranges ???*/
		// Normal strings
		#[regex(r#"(?:u8|u|U|L)?"(?:[\x20-\x7E&&[^"\\\n]]|\\[uU'"?\\abfnrtvx0-7])*""#)]
		StringLiteral,
		#[regex(r#"(?:u8|u|U|L)?R"[\x20-\x7E&&[^ \(\)\\\n\x0B\t\x0C]]*\("#)]
		RawStringLiteral,
		#[regex(r#"(?:u8|u|U|L)?'(?:[\x20-\x7E&&[^'\\\n]]|\\[uU'"?\\abfnrtvx0-7])*'"#)]
		CharLiteral,
		#[regex(r#"[\.]?[0-9](:?'?[0-9]|'[a-zA-Z_]|[eEpP][+-]|\\u[A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9]|\\U[A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9])*[\.]?"#)]
		PPNumber,
		#[error]
		Error
	}

	#[derive(Debug)]
	pub struct PreprocessingToken<'b> {
		kind: PreprocessorToken,
		originalString : &'b str,
		modifiedString : String,
		line: u32,
		column: u32,
		lineEnd: u32,
		columnEnd: u32,
	}

	#[derive(Debug)]
	pub struct PreLexer<'a> {
		extras: LexerFilter,
		currentNonSpliced: &'a str,
		current: String,
		line: u32,
		column: u32,
	}
	impl<'a> PreLexer<'a> {
		pub fn new(content: &String)->PreLexer {
			PreLexer{
				extras: LexerFilter{since_include: 3, since_has_include: 3, since_import: 3, since_preprocessing_operator: 0, since_open_paren_has_include: 0 },
				currentNonSpliced: content,
				current: content.clone(),
				line: 1,
				column: 0,
			}
		}

		fn spliceNewlinePosition(&self) -> Option<usize> {
			/* This is going to be the next nl found. IT DOES NOT DELIMIT TOKENS.
				if the next one is \n and the previous is a "\", it needs to be spliced.
				If we're unable to generate the token, or the token generated reaches
				the "\", then we splice and try again.
			*/
			let mut maybe_remove : Option<usize> = None;
			if Some("\n") == regex_find!(r"[\n]", &self.current) {
				let salt_pos: usize = self.current.chars().position(|x:char| x == '\n').unwrap();
				if salt_pos > 0 && self.current.chars().nth(salt_pos-1) == Some('\\') {
					maybe_remove = Some(salt_pos-1);
				}
			}
			return maybe_remove;
		}

		fn getNextTokenNonSpliced(&mut self) -> (Option<PreprocessorToken>, usize) {
			if self.extras.since_open_paren_has_include == 1 || self.extras.since_include == 1 {
				if let Some(res) = regex_find!(r"^<[^\n>]+>", &self.current) {
					return (Some(PreprocessorToken::HeaderName), res.len());
				}
				if let Some(res) = regex_find!(r#"^"[^\n"]+""#, &self.current) {
					return (Some(PreprocessorToken::HeaderName), res.len());
				}
			}
			let mut lex = PreprocessorToken::lexer_with_extras(&self.current, self.extras);
			if let Some(idx) = lex.next() {
				match idx {
					PreprocessorToken::PreprocessingOperator => {
						self.extras.since_preprocessing_operator = 0;
						self.extras.incr();
						return (Some(idx), lex.slice().len());
					}
					PreprocessorToken::OperatorPunctuator => {
						let text = lex.slice();
						match text {
							"(" => {if self.extras.since_has_include == 1 {self.extras.since_open_paren_has_include = 0;}}
							_ => {}
						}
						self.extras.incr();
						return (Some(idx), text.len());
					}
					PreprocessorToken::Ident => {
						let text = lex.slice();
						match text {
							"include" => {if self.extras.since_preprocessing_operator == 1 {self.extras.since_include = 0;}}
							"__has_include" => {self.extras.since_has_include = 0;}
							"import" => {self.extras.since_import = 0;}
							_ => {}
						}
						self.extras.incr();
						return (Some(idx), text.len());
					}
					PreprocessorToken::RawStringLiteral => {
						if let Some ((_,key)) = regex_captures!(r#"R"(.*)\("#, lex.slice()) {
							if let Some(position) = self.current.find((")".to_owned() + key + "\"").as_str()) {
								self.extras.incr();
								return (Some(idx), position+key.len()+2);
							}
						}
					}
					PreprocessorToken::Whitespace |
					PreprocessorToken::Comment |
					PreprocessorToken::Newline => {
						return (Some(idx), lex.slice().len());
					}
					PreprocessorToken::Error => {
						return (None, 0);
					}
					_ => {
						self.extras.incr();
						return (Some(idx), lex.slice().len());
					}
				}
			}
			return (None, 0);
		}

		fn applySplice(&mut self, splice_point: usize) -> () {
			self.current.remove(splice_point);
			self.current.remove(splice_point);
		}

		fn getNextTokenData(&mut self) -> (Option<PreprocessorToken>, usize, usize) {
			let (mut kind, mut idx, mut splices) = (None, 0, 0);
			loop {
				if self.current.is_empty() {break;}
				else {
					if let Some(_) = regex_find!(r#"^<::[^:>]"#, &self.current)
					{
						(kind, idx) = (Some(PreprocessorToken::OperatorPunctuator), 1);
						break;
					} else {
						let splice_point_slash_nl = self.spliceNewlinePosition();
						(kind, idx) = self.getNextTokenNonSpliced();
						if splice_point_slash_nl.contains(&idx) || (kind.is_none() && splice_point_slash_nl.is_some()) {
							self.applySplice(splice_point_slash_nl.unwrap());
							splices += 1;
							continue;
						} else if kind.is_some() {
							break;
						} else {
							eprintln!("Encountered unmachable preprocessing token at: {} {}", self.line, self.column);
							return (None, 0,0);
						}
					}
				}
			}
			return (kind, idx, splices);
		}

	}
	impl<'a> Iterator for PreLexer<'a> {
		type Item = PreprocessingToken<'a>;
		fn next(&mut self) -> Option<Self::Item> {
			let mut res: Option<Self::Item> = None;
			let (kind, idx, splices) = self.getNextTokenData();
			if let Some(kind) = kind {
				let (mut lineEnd, mut columnEnd) = (self.line, self.column);
				{
					let (mut idxCpy, mut splicesCpy) = (idx as i64, splices as i64);
					for charGud in self.currentNonSpliced.chars() {
						idxCpy-=1;
						columnEnd += 1;
						if charGud == '\n' {
							columnEnd = 1;
							lineEnd += 1;
							if splicesCpy > 0 {
								splicesCpy -= 1;
								idxCpy+=2;
							}
						}
						if splicesCpy == 0 && idxCpy == 0 {
							break;
						}
					}
				}
				let mut originalString = &self.currentNonSpliced[0..idx+splices*2];
				if originalString.ends_with("\\\n") {
					originalString = &self.currentNonSpliced[0..idx+splices*2-2];
				}
				res = Some(
					Self::Item {
						kind: kind,
						originalString: &originalString,
						modifiedString: self.current[0..idx].to_string(),
						line: self.line,
						column: self.column,
						lineEnd,
						columnEnd,
					}
				);
				self.currentNonSpliced = &self.currentNonSpliced[idx+splices*2..];
				self.current = self.current[idx..].to_string();
				(self.line, self.column) = (lineEnd, columnEnd);
			}
			return res;
		}
	}
}