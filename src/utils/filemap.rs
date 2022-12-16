//! Map of paths to files
use std::path::Path;
use std::{collections::HashMap, fs::File, fs::OpenOptions, io::Read, sync::Arc};

use crate::utils::structs::CompileFile;

use super::parameters::Parameters;

#[derive(Debug)]
/// A map of all the files that are being used. This is used to avoid opening the same file twice.
pub struct FileMap {
    /// Parameters of the compilation
    params: Arc<Parameters>,
    /// Files opened, but not read yet
    openedButNotRead: HashMap<String, File>,
    /// Files opened
    files: HashMap<String, Arc<CompileFile>>,
    /// Resolved paths
    resolvedPaths: HashMap<String, String>,
}

impl<'a> FileMap {
    /// New file map.
    pub fn new(params: Arc<Parameters>) -> Self {
        Self {
            params,
            openedButNotRead: HashMap::new(),
            files: HashMap::new(),
            resolvedPaths: HashMap::new(),
        }
    }

    /// Get an already opened file. On error, crash.
    pub fn getFile(&mut self, path: &str) -> Arc<CompileFile> {
        let path = self.getPath(path).unwrap();
        if let Some(v) = self.files.get(&path) {
            return v.clone();
        }
        panic!("File not found in visited files: {path}");
    }

    /// Get file. If not present, open it. On error, crash.
    pub fn getAddFile(&'a mut self, path: &str) -> Arc<CompileFile> {
        let path = &self.getPath(path).unwrap();
        if self.files.contains_key(path) {
        } else {
            if let Err(error) = self.hasFileAccessImpl(path) {
                panic!("Could not open {path}. Error: {error}");
            }
            let mut filecontents: String = String::new();
            if let Err(err) = self
                .openedButNotRead
                .get(path)
                .unwrap()
                .read_to_string(&mut filecontents)
            {
                panic!("Error reading {path}. Error: {err}");
            }
            self.files.insert(
                path.to_string(),
                Arc::new(CompileFile::new(path.to_string(), filecontents)),
            );
        }
        return self.files.get(path).unwrap().clone();
    }

    /// Can it access the file? Does not need to be previously opened.
    pub fn hasFileAccess(&mut self, path: &str) -> bool {
        let absolutePath = self.getPath(path);
        return absolutePath.is_ok() && self.hasFileAccessImpl(&absolutePath.unwrap()).is_ok();
    }

    /// Impl for `hasFileAccess`. Path is resolved here.
    fn hasFileAccessImpl(&mut self, absolutePath: &str) -> Result<(), String> {
        if self.files.contains_key(absolutePath) || self.openedButNotRead.contains_key(absolutePath)
        {
        } else {
            let filename = std::path::Path::new(absolutePath);
            if !filename.extension().map_or(false, |ext| {
                ext.eq_ignore_ascii_case("cpp")
                    || ext.eq_ignore_ascii_case("hpp")
                    || ext.eq_ignore_ascii_case("h")
            }) {
                log::error!("Unsuported file type: {}", absolutePath);
            }
            let file: File = match OpenOptions::new().read(true).open(absolutePath) {
                Ok(it) => it,
                Err(err) => {
                    return Err(err.to_string());
                }
            };
            self.openedButNotRead.insert(absolutePath.to_string(), file);
        }
        return Ok(());
    }

    /// Get paths of current files opened.
    pub fn getCurrPaths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        for path in self.files.keys() {
            paths.push(path.clone());
        }
        return paths;
    }

    /// Add a fake test file. Intened for testing.
    pub fn addTestFile(&mut self, path: String, content: String) {
        self.resolvedPaths.insert(path.clone(), path.clone());
        self.files
            .insert(path.clone(), Arc::new(CompileFile::new(path, content)));
    }

    /// Resolve a path. On error, return error.
    fn getPath(&mut self, pathStr: &str) -> Result<String, String> {
        match self.resolvedPaths.entry(pathStr.to_string()) {
            std::collections::hash_map::Entry::Occupied(e) => {
                return Ok(e.get().clone());
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                let path = Path::new(&pathStr);
                if path.is_absolute() && path.exists() {
                    return Ok(v.insert(pathStr.to_string()).clone());
                }
                for dir in &self.params.includeDirs {
                    let resultingPath = Path::new(dir).join(path);
                    if resultingPath.exists() {
                        return Ok(v
                            .insert(
                                resultingPath
                                    .canonicalize()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string(),
                            )
                            .clone());
                    }
                }
                for dir in &self.params.includeSystemDirs {
                    let resultingPath = Path::new(dir).join(path);
                    if resultingPath.exists() {
                        return Ok(v
                            .insert(
                                resultingPath
                                    .canonicalize()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string(),
                            )
                            .clone());
                    }
                }
                if !path.is_absolute() && path.exists() {
                    return Ok(v
                        .insert(path.canonicalize().unwrap().to_str().unwrap().to_string())
                        .clone());
                }
                return Err(format!("Could not find file {pathStr}"));
            }
        }
    }
}
