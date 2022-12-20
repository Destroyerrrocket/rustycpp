//! An aggregation of lexers that can be used to represent the preprocessor
//! state of file inclussions.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::preprocessor::prelexer::PreLexer;
use crate::utils::filemap::FileMap;
use crate::utils::structs::FileTokPos;

use super::pretoken::PreToken;

#[derive(Debug)]
#[doc(hidden)]
struct FileLexer {
    pub compFile: String,
    pub lexer: PreLexer,
}

#[derive(Debug)]
/// An aggregation of lexers that can be used to represent the preprocessor
/// state of file inclussions.
pub struct MultiLexer {
    /// The current files opened by the hole compiler
    fileMapping: Arc<Mutex<FileMap>>,
    /// Files in the order they were opened
    files: Vec<FileLexer>,
    /// Pushed tokens to return back. This is specially useful when reevaluating an expanded macro
    pushedTokens: VecDeque<FileTokPos<PreToken>>,
}

impl MultiLexer {
    /// Creates a new empty multilexer
    pub fn new_def(files: Arc<Mutex<FileMap>>) -> Self {
        Self {
            fileMapping: files,
            files: vec![],
            pushedTokens: VecDeque::new(),
        }
    }

    /// Creates a new multilexer with the starting file
    pub fn new((files, file): (Arc<Mutex<FileMap>>, u64)) -> Self {
        let currFile = files.lock().unwrap().getOpenedFile(file);
        let lexer = PreLexer::new(currFile.content().clone());

        Self {
            fileMapping: files,
            files: vec![FileLexer {
                compFile: currFile.path().clone(),
                lexer,
            }],
            pushedTokens: VecDeque::new(),
        }
    }

    /// Push tokens to be returned back
    pub fn pushTokensDec(&mut self, toks: VecDeque<FileTokPos<PreToken>>) {
        for i in toks.into_iter().rev() {
            self.pushedTokens.push_front(i);
        }
    }

    /// Push tokens to be returned back
    pub fn pushTokensVec(&mut self, toks: Vec<FileTokPos<PreToken>>) {
        for i in toks.into_iter().rev() {
            self.pushedTokens.push_front(i);
        }
    }

    /// Push token to be returned back
    pub fn pushToken(&mut self, tok: FileTokPos<PreToken>) {
        self.pushedTokens.push_back(tok);
    }

    /// Push a new file. Please be careful when you're doing this, as the pushed
    /// tokens will still be returned first!
    pub fn pushFile(&mut self, path: String) {
        self.files.push(FileLexer {
            compFile: path.clone(),
            lexer: PreLexer::new(
                self.fileMapping
                    .lock()
                    .unwrap()
                    .getAddFileRef(path.as_str())
                    .content()
                    .to_string(),
            ),
        });
    }

    /// Push a new file. Please be careful when you're doing this, as the pushed
    /// tokens will still be returned first!
    pub fn expectHeader(&mut self) {
        if let Some(lex) = self.files.last_mut() {
            lex.lexer.expectHeader();
        }
    }

    /// Current mapping of files.
    pub fn fileMapping(&self) -> Arc<Mutex<FileMap>> {
        self.fileMapping.clone()
    }

    /// Can this multilexer access the file? It does not need to be previously
    /// oppened.
    pub fn hasFileAccess(&self, file: &str) -> bool {
        self.fileMapping.lock().unwrap().hasFileAccess(file)
    }
}

impl Iterator for MultiLexer {
    type Item = FileTokPos<PreToken>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.pushedTokens.pop_front() {
            return Some(t);
        }
        loop {
            if let Some(lexer) = self.files.last_mut() {
                match lexer.lexer.next() {
                    None => {}
                    Some(tok) => {
                        return Some(FileTokPos::new(
                            self.fileMapping
                                .lock()
                                .expect("Thread panic")
                                .getAddFile(&lexer.compFile),
                            tok,
                        ));
                    }
                }
            } else {
                return None;
            }
            // If we reach here, the single-lexer is empty. We pop it and hope the next one provides more content.
            self.files.pop();
        }
    }
}
