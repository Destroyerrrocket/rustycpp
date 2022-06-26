// Unsoported. Haven't removed it as it may be useful someday
#[derive(Debug)]
pub enum LalrPopLexerWrapperState {
	Normal,
	SkipWhitespace,
	SkipWsNl,
}

#[derive(Debug)]
pub struct LalrPopLexerWrapper<'slice, T: Clone> {
	tokens: &'slice [T],
	state: LalrPopLexerWrapperState,
	idx: usize,
}

impl<'slice, T: Clone> LalrPopLexerWrapper<'slice, T> {
	pub fn new(tokens: &'slice [T])->LalrPopLexerWrapper<T> {
		LalrPopLexerWrapper{tokens: tokens, state: LalrPopLexerWrapperState::Normal, idx: 0 }
	}
}

impl<'slice, T: Clone> Iterator for LalrPopLexerWrapper<'slice, T> {
	type Item = Result<(usize, T, usize), ()>;
	fn next(&mut self) -> Option<Self::Item> {
		loop {
		if self.tokens.is_empty() {return None;}
		else {
			let tok = &self.tokens[0];
			self.tokens = &self.tokens[1..];
			self.idx += 1;
			/*
			if tok.kind.isWhitespace() && matches!(self.state, LalrPopLexerWrapperState::SkipWhitespace | LalrPopLexerWrapperState::SkipWsNl) {
				continue;
			}
			if matches!(tok.kind, PreToken::Newline) && matches!(self.state,LalrPopLexerWrapperState::SkipWsNl) {
				continue;
			}
			*/
			return Some(Ok((self.idx, tok.clone(), self.idx+1)));
		}
		};
	}
}