use crate::{prelexer::PreprocessingToken};

use colored::Colorize;

#[derive(Debug)]
pub struct CompileFile {
	path: String,
	content: String,
	newlines: Vec<usize>,
}

impl CompileFile {
	pub fn new(path: String, content: String) -> CompileFile {
		CompileFile {
			path: path,
			content: content.replace("\r\n", "\n"),
			newlines: content.char_indices().filter(|(_, char)| match char {'\n' => true, _ => false}).map(|(idx, _)| idx).collect(),
		}
	}

	pub fn path(&self) -> &String {
		return &self.path;
	}
	pub fn content(&self) -> &String {
		return &self.content;
	}

	pub fn getRowColumn(&self, diff: usize) -> (usize, usize) {
		if self.newlines.is_empty() {return (1, diff+1);}
		let part = self.newlines.as_slice().partition_point(|&x| x < diff);
		if part == self.newlines.len() {
			return (part+1, diff-self.newlines.last().unwrap());
		}
		return (part+1, diff-self.newlines.get(part-1).unwrap_or(&0));
	}

	pub fn getLocStr(&self, diff:usize) -> String {
		let (r,c) = self.getRowColumn(diff);
		format!("{}:{}:{}", self.path(), r, c)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompileMsgKind {
	Notice,
	Warning,
	Error,
	FatalError,
}

impl ToString for CompileMsgKind {
    fn to_string(&self) -> String {
        match self {
            CompileMsgKind::Notice => "Notice".bright_blue().to_string(),
            CompileMsgKind::Warning => "Warning".bright_yellow().to_string(),
            CompileMsgKind::Error => "Error".bright_red().to_string(),
            CompileMsgKind::FatalError => "Error".bright_red().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompileMsg<'a> {
	kind: CompileMsgKind,
	msg: String,
	file: &'a CompileFile,
	at: usize,
	atEnd: Option<usize>,
}

impl<'a> CompileMsg<'a> {
	pub fn errorLocStr(&self) -> String {
		return self.file.getLocStr(self.at);
	}

	pub fn severity(&self) -> CompileMsgKind {
		return self.kind;
	}
}

impl<'a> ToString for CompileMsg<'a> {
	fn to_string(&self) -> String {
		return format!("{}:{}\nAt: {}", self.kind.to_string(), self.msg, self.errorLocStr());
	}
}

pub struct CompileError {}

impl<'a> CompileError {
	pub fn from_preTo<T: ToString>(msg: T, file: &'a CompileFile, preToken: &PreprocessingToken) -> CompileMsg<'a> {
		CompileMsg { msg: msg.to_string(), file: file, at: preToken.originalDiff, atEnd: Some(preToken.originalDiffEnd), kind: CompileMsgKind::Error }
	}

	pub fn from_at(msg: String, file: &CompileFile, at: usize, atEnd: Option<usize>) -> CompileMsg {
		CompileMsg { msg: msg, file: file, at: at, atEnd: atEnd, kind: CompileMsgKind::Error }
	}
}

pub struct CompileWarning {}
impl<'a> CompileWarning {
	pub fn from_preTo<T: ToString>(msg: T, file: &'a CompileFile, preToken: &PreprocessingToken) -> CompileMsg<'a> {
		CompileMsg { msg: msg.to_string(), file: file, at: preToken.originalDiff, atEnd: Some(preToken.originalDiffEnd), kind: CompileMsgKind::Warning }
	}

	pub fn from_at(msg: String, file: &CompileFile, at: usize, atEnd: Option<usize>) -> CompileMsg {
		CompileMsg { msg: msg, file: file, at: at, atEnd: atEnd, kind: CompileMsgKind::Warning }
	}
}