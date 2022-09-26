use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{
    filemap::FileMap,
    prelexer::PreLexer,
    utils::{pretoken::PreToken, structs::FilePreTokPos},
};

#[derive(Debug)]
struct FileLexer {
    pub compFile: String,
    pub lexer: PreLexer,
}

#[derive(Debug)]
pub struct MultiLexer {
    fileMapping: Arc<Mutex<FileMap>>,
    files: Vec<FileLexer>,
    pushedTokens: VecDeque<FilePreTokPos<PreToken>>,
}

impl MultiLexer {
    pub fn new_def(files: Arc<Mutex<FileMap>>) -> Self {
        Self {
            fileMapping: files,
            files: vec![],
            pushedTokens: VecDeque::new(),
        }
    }

    pub fn new((files, file): (Arc<Mutex<FileMap>>, &str)) -> Self {
        let lexer = {
            let currFile = files.lock().unwrap().getFile(file);
            PreLexer::new(currFile.content().clone())
        };

        Self {
            fileMapping: files,
            files: vec![FileLexer {
                compFile: file.to_string(),
                lexer,
            }],
            pushedTokens: VecDeque::new(),
        }
    }

    pub fn pushTokensDec(&mut self, toks: VecDeque<FilePreTokPos<PreToken>>) {
        for i in toks.into_iter().rev() {
            self.pushedTokens.push_front(i);
        }
    }

    pub fn pushTokensVec(&mut self, toks: Vec<FilePreTokPos<PreToken>>) {
        for i in toks.into_iter().rev() {
            self.pushedTokens.push_front(i);
        }
    }

    pub fn pushToken(&mut self, tok: FilePreTokPos<PreToken>) {
        self.pushedTokens.push_back(tok);
    }

    pub fn pushFile(&mut self, path: String) {
        self.files.push(FileLexer {
            compFile: path.clone(),
            lexer: PreLexer::new(
                self.fileMapping
                    .lock()
                    .unwrap()
                    .getAddFile(path.as_str())
                    .content()
                    .to_string(),
            ),
        });
    }

    pub fn expectHeader(&mut self) {
        if let Some(lex) = self.files.last_mut() {
            lex.lexer.expectHeader();
        }
    }

    pub fn fileMapping(&self) -> Arc<Mutex<FileMap>> {
        self.fileMapping.clone()
    }

    pub fn hasFileAccess(&self, file: &str) -> bool {
        self.fileMapping.lock().unwrap().hasFileAccess(file)
    }
}

impl Iterator for MultiLexer {
    type Item = FilePreTokPos<PreToken>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.pushedTokens.pop_front() {
            return Some(t);
        }
        loop {
            if let Some(lexer) = self.files.last_mut() {
                match lexer.lexer.next() {
                    None => {}
                    Some(tok) => {
                        return Some(FilePreTokPos::new(
                            self.fileMapping
                                .lock()
                                .expect("Thread panic")
                                .getFile(&lexer.compFile),
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
