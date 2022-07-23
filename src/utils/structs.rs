use std::fmt::Debug;
use std::sync::Arc;

use colored::Colorize;

use super::pretoken::PreToken;

#[derive(Debug, Default)]
pub struct CompileFile {
    path: String,
    content: Arc<String>,
    newlines: Vec<usize>,
}

impl CompileFile {
    pub fn new(path: String, content: String) -> CompileFile {
        let contentFile = Arc::new(content.replace("\r\n", "\n"));
        CompileFile {
            path: path,
            content: contentFile.clone(),
            newlines: contentFile
                .char_indices()
                .filter(|(_, char)| match char {
                    '\n' => true,
                    _ => false,
                })
                .map(|(idx, _)| idx)
                .collect(),
        }
    }

    pub fn path(&self) -> &String {
        return &self.path;
    }
    pub fn content(&self) -> &String {
        return &self.content;
    }

    pub fn getRowColumn(&self, diff: usize) -> (usize, usize) {
        if self.newlines.is_empty() {
            return (1, diff + 1);
        }
        let part = self.newlines.as_slice().partition_point(|&x| x < diff);
        if part == self.newlines.len() {
            return (part + 1, diff - self.newlines.last().unwrap());
        }
        return (part + 1, diff - self.newlines.get(part - 1).unwrap_or(&0));
    }

    pub fn getLocStr(&self, diff: usize) -> String {
        let (r, c) = self.getRowColumn(diff);
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
pub struct CompileMsg {
    kind: CompileMsgKind,
    msg: String,
    file: Arc<CompileFile>,
    at: usize,
    atEnd: Option<usize>,
}

impl CompileMsg {
    pub fn errorLocStr(&self) -> String {
        return self.file.getLocStr(self.at);
    }

    pub fn severity(&self) -> CompileMsgKind {
        return self.kind;
    }
}

impl ToString for CompileMsg {
    fn to_string(&self) -> String {
        return format!(
            "{} at: {}\n{}\n",
            self.kind.to_string(),
            self.errorLocStr(),
            self.msg
        );
    }
}

pub struct CompileError {}

impl CompileError {
    pub fn from_preTo<T: ToString, Tok: Clone + Debug>(
        msg: T,
        preToken: &FilePreTokPos<Tok>,
    ) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file: preToken.file.clone(),
            at: preToken.tokPos.start,
            atEnd: Some(preToken.tokPos.end),
            kind: CompileMsgKind::Error,
        }
    }

    pub fn from_at<T: ToString>(
        msg: T,
        file: Arc<CompileFile>,
        at: usize,
        atEnd: Option<usize>,
    ) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file: file,
            at: at,
            atEnd: atEnd,
            kind: CompileMsgKind::Error,
        }
    }
}

pub struct CompileWarning {}
impl CompileWarning {
    pub fn from_preTo<T: ToString>(msg: T, preToken: &FilePreTokPos<PreToken>) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file: preToken.file.clone(),
            at: preToken.tokPos.start,
            atEnd: Some(preToken.tokPos.end),
            kind: CompileMsgKind::Warning,
        }
    }

    pub fn from_at(
        msg: String,
        file: Arc<CompileFile>,
        at: usize,
        atEnd: Option<usize>,
    ) -> CompileMsg {
        CompileMsg {
            msg: msg,
            file: file,
            at: at,
            atEnd: atEnd,
            kind: CompileMsgKind::Warning,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PreTokPos<T: Clone + Debug> {
    pub start: usize,
    pub tok: T,
    pub end: usize,
}
impl<T: Clone + Debug> PreTokPos<T> {
    pub fn tokString(&self) -> String {
        format!("{:?}", self.tok).to_string()
    }
}

#[derive(Debug, Clone)]
pub struct FilePreTokPos<T: Clone + Debug> {
    pub file: Arc<CompileFile>,
    pub tokPos: PreTokPos<T>,
}

impl<T: Clone + Debug> FilePreTokPos<T> {
    pub fn new(file: Arc<CompileFile>, tok: PreTokPos<T>) -> FilePreTokPos<T> {
        FilePreTokPos {
            file: file,
            tokPos: tok,
        }
    }

    pub fn new_meta(tok: T) -> FilePreTokPos<T> {
        FilePreTokPos {
            file: Arc::new(CompileFile::default()),
            tokPos: PreTokPos {
                start: 0,
                tok,
                end: 0,
            },
        }
    }

    pub fn new_meta_c<U: Clone + Debug>(tok: T, other: &FilePreTokPos<U>) -> FilePreTokPos<T> {
        FilePreTokPos {
            file: other.file.clone(),
            tokPos: PreTokPos {
                start: other.tokPos.start,
                tok,
                end: other.tokPos.end,
            },
        }
    }

    pub fn tokString(&self) -> String {
        self.tokPos.tokString()
    }
}

#[macro_export]
macro_rules! filePreTokPosMatchArm {
    ( $x:pat ) => {
        FilePreTokPos {
            file: _,
            tokPos: PreTokPos {
                start: _,
                tok: $x,
                end: _,
            },
        }
    };
}

#[macro_export]
macro_rules! filePreTokPosMatches {
    ( $file:expr, $x:pat ) => {
        matches!(
            $file,
            FilePreTokPos {
                file: _,
                tokPos: PreTokPos {
                    start: _,
                    tok: $x,
                    end: _,
                },
            }
        )
    };
}
