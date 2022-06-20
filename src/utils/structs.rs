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
		format!("{}:{}:{}", self.content(), r, c)
	}
}