use std::{collections::HashMap, fs::File, io::Read, sync::Arc};

use crate::utils::structs::CompileFile;

#[derive(Debug)]
pub struct FileMap {
    files: HashMap<String, Arc<CompileFile>>,
}

impl<'a> FileMap {
    pub fn new() -> FileMap {
        FileMap {
            files: HashMap::new(),
        }
    }

    pub fn getFile(&self, path: &str) -> Arc<CompileFile> {
        if let Some(v) = self.files.get(path) {
            return v.clone();
        }
        panic!("File not found in visited files: {}", path);
    }

    pub fn getAddFile(&'a mut self, path: &str) -> Arc<CompileFile> {
        if !self.files.contains_key(path) {
            if !path.ends_with(".cpp") && !path.ends_with(".hpp") {
                log::error!("Unsuported file type: {}", path);
            }
            let mut file: File = match File::open(&path) {
                Ok(it) => it,
                Err(err) => {
                    panic!(
                        "Could not open {file}. Error: {error}",
                        file = path,
                        error = err
                    );
                }
            };
            let mut filecontents: String = String::new();
            if let Err(err) = file.read_to_string(&mut filecontents) {
                panic!(
                    "Error reading {file}. Error: {error}",
                    file = path,
                    error = err
                );
            }
            self.files.insert(
                path.to_string(),
                Arc::new(CompileFile::new(path.to_string(), filecontents)),
            );
        }
        return self.files.get(path).unwrap().clone();
    }

    pub fn getCurrPaths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        for path in self.files.keys() {
            paths.push(path.clone());
        }
        return paths;
    }

    #[allow(dead_code)]
    pub fn addTestFile(&mut self, path: String, content: String) {
        self.files
            .insert(path.to_string(), Arc::new(CompileFile::new(path, content)));
    }
}

impl Default for FileMap {
    fn default() -> Self {
        Self::new()
    }
}
