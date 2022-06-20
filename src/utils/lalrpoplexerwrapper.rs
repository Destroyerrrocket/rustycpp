use std::collections::VecDeque;

use crate::prelexer::{PreprocessingToken};
use crate::utils::pretoken::PreToken;

#[derive(Debug)]
pub struct LalrPopLexerWrapper {
	tokens: VecDeque<PreprocessingToken>,
}

impl LalrPopLexerWrapper {
	pub fn new(tokens: VecDeque<PreprocessingToken>)->LalrPopLexerWrapper {
		LalrPopLexerWrapper{tokens: tokens}
	}
}

impl Iterator for LalrPopLexerWrapper {
	type Item = Result<(usize, PreToken, usize), String>;
	fn next(&mut self) -> Option<Self::Item> {
		if self.tokens.is_empty() {return None;}
		else {
			let tok = self.tokens.pop_front().unwrap();
			return Some(Ok((tok.originalDiff, tok.kind, tok.originalDiff+tok.originalLen)));
		}
	}
}