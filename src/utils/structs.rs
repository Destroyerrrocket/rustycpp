//! A varitety of structs used throughout the compiler.
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use colored::Colorize;

use super::filemap::FileMap;

#[derive(Debug, Default, Eq)]
/// A file to be compiled
pub struct CompileFile {
    /// Path to the file
    path: String,
    /// Contents of the file
    content: Arc<String>,
    /// Offsets to the newlines of the file
    newlines: Vec<usize>,
}

impl PartialEq for CompileFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl CompileFile {
    /// A file to be compiled
    pub fn new(path: String, content: String) -> Self {
        let contentFile = Arc::new(content.replace("\r\n", "\n"));
        Self {
            path,
            content: contentFile.clone(),
            newlines: contentFile
                .char_indices()
                .filter(|(_, char)| matches!(char, '\n'))
                .map(|(idx, _)| idx)
                .collect(),
        }
    }

    /// Get the path to the file
    pub const fn path(&self) -> &String {
        &self.path
    }

    /// Get the content of the file
    pub fn content(&self) -> &String {
        &self.content
    }

    /// Get the row and column of a position
    pub fn getRowColumn(&self, diff: usize) -> (usize, usize) {
        if self.newlines.is_empty() {
            return (1, diff + 1);
        }
        let part = self.newlines.as_slice().partition_point(|&x| x < diff);
        if part == self.newlines.len() {
            return (part + 1, diff - self.newlines.last().unwrap());
        } else if part == 0 {
            return (1, diff + 1);
        }
        return (part + 1, diff - self.newlines.get(part - 1).unwrap_or(&0));
    }

    /// Get the location of a position as a string
    pub fn getLocStr(&self, diff: Option<usize>) -> String {
        diff.map_or_else(
            || self.path().clone(),
            |diff| {
                let (r, c) = self.getRowColumn(diff);
                format!("{}:{r}:{c}", self.path())
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Kind of compile message
#[doc(hidden)]
pub enum CompileMsgKind {
    Notice,
    Warning,
    Error,
    FatalError,
}

impl ToString for CompileMsgKind {
    fn to_string(&self) -> String {
        match self {
            Self::Notice => "Notice".bright_blue().to_string(),
            Self::Warning => "Warning".bright_yellow().to_string(),
            Self::Error => "Error".bright_red().to_string(),
            Self::FatalError => "Fatal error".bright_red().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
/// A compile message
#[doc(hidden)]
pub struct CompileMsg {
    kind: CompileMsgKind,
    msg: String,
    file: u64,
    at: Option<usize>,
    atEnd: Option<usize>,
}

impl CompileMsg {
    /// Location of the message
    pub fn errorLocStr(&self, fileMap: &Arc<Mutex<FileMap>>) -> String {
        let file = fileMap.lock().unwrap().getOpenedFile(self.file);
        file.getLocStr(self.at)
    }

    /// Severity of the message
    pub const fn severity(&self) -> CompileMsgKind {
        self.kind
    }

    /// Print the message
    pub fn print(&self, fileMap: &Arc<Mutex<FileMap>>) {
        match self.kind {
            CompileMsgKind::Notice => log::info!("{}", self.to_string(fileMap)),
            CompileMsgKind::Warning => log::warn!("{}", self.to_string(fileMap)),
            CompileMsgKind::Error => log::error!("{}", self.to_string(fileMap)),
            CompileMsgKind::FatalError => log::error!("{}", self.to_string(fileMap)),
        }
    }

    pub fn to_string(&self, fileMap: &Arc<Mutex<FileMap>>) -> String {
        if self.file == 0 {
            format!("{}:\n{}\n", self.kind.to_string(), self.msg)
        } else {
            format!(
                "{} at: {}\n{}\n",
                self.kind.to_string(),
                self.errorLocStr(fileMap),
                self.msg
            )
        }
    }
}

#[doc(hidden)]
pub struct CompileError;

#[doc(hidden)]
impl CompileError {
    pub fn unlocated<T: ToString>(msg: T) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file: 0,
            at: None,
            atEnd: None,
            kind: CompileMsgKind::Error,
        }
    }

    pub fn on_file<T: ToString>(msg: T, file: u64) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file,
            at: None,
            atEnd: None,
            kind: CompileMsgKind::Error,
        }
    }

    pub fn from_preTo<T: ToString, Tok: Clone + Debug>(
        msg: T,
        preToken: &FileTokPos<Tok>,
    ) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file: preToken.file.clone(),
            at: Some(preToken.tokPos.start),
            atEnd: Some(preToken.tokPos.end),
            kind: CompileMsgKind::Error,
        }
    }

    pub fn from_at<T: ToString>(msg: T, file: u64, at: usize, atEnd: Option<usize>) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file,
            at: Some(at),
            atEnd,
            kind: CompileMsgKind::Error,
        }
    }
}

#[doc(hidden)]
pub struct CompileWarning;
#[doc(hidden)]
impl CompileWarning {
    pub fn unlocated<T: ToString>(msg: T) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file: 0,
            at: None,
            atEnd: None,
            kind: CompileMsgKind::Warning,
        }
    }

    pub fn on_file<T: ToString>(msg: T, file: u64) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file,
            at: None,
            atEnd: None,
            kind: CompileMsgKind::Warning,
        }
    }

    pub fn from_preTo<T: ToString, U: Clone + Debug>(
        msg: T,
        preToken: &FileTokPos<U>,
    ) -> CompileMsg {
        CompileMsg {
            msg: msg.to_string(),
            file: preToken.file.clone(),
            at: Some(preToken.tokPos.start),
            atEnd: Some(preToken.tokPos.end),
            kind: CompileMsgKind::Warning,
        }
    }

    pub fn from_at(msg: String, file: u64, at: usize, atEnd: usize) -> CompileMsg {
        CompileMsg {
            msg,
            file,
            at: Some(at),
            atEnd: Some(atEnd),
            kind: CompileMsgKind::Warning,
        }
    }
}

#[derive(Debug, Clone)]
/// A token and its possition in a file
pub struct TokPos<T: Clone + Debug> {
    /// Start of the token in the file
    pub start: usize,
    /// End of the token in the file
    pub end: usize,
    /// The token
    pub tok: T,
}
impl<T: Clone + Debug> TokPos<T> {
    /// Token to string. Use the debug impl
    pub fn tokStringDebug(&self) -> String {
        format!("{:?}", self.tok)
    }
}

#[derive(Debug, Clone)]
/// A token position and its file
pub struct FileTokPos<T: Clone + Debug> {
    /// file of the token
    pub file: u64,
    /// token + position
    pub tokPos: TokPos<T>,
}

impl<T: Clone + Debug> FileTokPos<T> {
    /// New token
    pub fn new(file: u64, tok: TokPos<T>) -> Self {
        Self { file, tokPos: tok }
    }

    /// New meta token. It is not located anywhere
    pub fn new_meta(tok: T) -> Self {
        Self {
            file: 0,
            tokPos: TokPos {
                start: 0,
                end: 0,
                tok,
            },
        }
    }

    /// New meta token. It copies its location from another token, even if it is
    /// not located anywhere. Allows for better diagnostics
    pub fn new_meta_c<U: Clone + Debug>(tok: T, other: &FileTokPos<U>) -> Self {
        Self {
            file: other.file.clone(),
            tokPos: TokPos {
                start: other.tokPos.start,
                end: other.tokPos.end,
                tok,
            },
        }
    }

    /// Token to string. Use the debug impl
    pub fn tokStringDebug(&self) -> String {
        self.tokPos.tokStringDebug()
    }
}

/// generate boilerplate for the left side of a match statement, where the
/// matched element is a [`FileTokPos`]. Most of the time, the file, start
/// and end of a token are not relevant at all.
#[macro_export]
macro_rules! fileTokPosMatchArm {
    ( $x:pat ) => {
        FileTokPos {
            tokPos: TokPos { tok: $x, .. },
            ..
        }
    };
}

/// generate boilerplate for a matches! macro, where the matched element is a
/// [`FileTokPos`]. Most of the time, the file, start and end of a token are
/// not relevant at all.
#[macro_export]
macro_rules! fileTokPosMatches {
    ( $file:expr, $x:pat ) => {
        matches!(
            $file,
            FileTokPos {
                tokPos: TokPos { tok: $x, .. },
                ..
            }
        )
    };
}
