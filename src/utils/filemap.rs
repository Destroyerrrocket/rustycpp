//! Map of paths to files
#![allow(clippy::verbose_file_reads, clippy::cast_possible_truncation)]

use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs::File, fs::OpenOptions, io::Read, sync::Arc};

use crate::utils::structs::CompileFile;

use super::parameters::Parameters;

#[derive(Debug)]
enum Either {
    CompileFile(Arc<CompileFile>),
    NotReadFile(Option<File>),
}

#[derive(Debug)]
/// A map of all the files that are being used. This is used to avoid opening the same file twice.
pub struct FileMap {
    /// Parameters of the compilation
    params: Arc<Parameters>,
    /// Files opened
    files: Vec<Either>,
    /// Resolved paths
    resolvedPaths: HashMap<String, u64>,
    /// Reverse-resolved paths
    reverseResolved: HashMap<u64, String>,
}

impl<'a> FileMap {
    /// New file map.
    pub fn new(params: Arc<Parameters>) -> Self {
        let mut me = Self {
            params,
            files: vec![],
            resolvedPaths: HashMap::new(),
            reverseResolved: HashMap::new(),
        };
        me.files.push(Either::CompileFile(Arc::new(CompileFile::new(
            "<unknown>".to_string(),
            "You are trying to read an invalid file".to_owned(),
        ))));
        me.resolvedPaths.insert("<unknown>".to_owned(), 0);
        me
    }

    fn internalReadFile(&mut self, path: u64, mut file: &File) -> Arc<CompileFile> {
        let mut filecontents: String = String::new();
        let pathStr = self.reverseResolved.get(&path).unwrap();
        if let Err(err) = file.read_to_string(&mut filecontents) {
            panic!("Error reading {pathStr}. Error: {err}");
        }

        let res = Arc::new(CompileFile::new(pathStr.clone(), filecontents));
        self.files
            .insert(path as usize, Either::CompileFile(res.clone()));
        return res;
    }

    /// Get an already opened file. On error, crash.
    pub fn getOpenedFile(&mut self, path: u64) -> Arc<CompileFile> {
        match self.files.get_mut(path as usize) {
            Some(Either::CompileFile(v)) => return v.clone(),
            Some(Either::NotReadFile(file)) => {
                let fileRef = file.take().unwrap();
                return self.internalReadFile(path, &fileRef);
            }
            _ => panic!("File not found in visited files: {path}"),
        }
    }

    /// Get file. If not present, open it. On error, crash.
    pub fn getAddFile(&'a mut self, path: &str) -> u64 {
        return self.getPath(path).unwrap();
    }

    /// Get file. If not present, open it. On error, crash.
    pub fn getAddFileRef(&'a mut self, path: &str) -> Arc<CompileFile> {
        let index = &self.getPath(path).unwrap();
        if let Some(file) = self.files.get_mut(*index as usize) {
            match file {
                Either::CompileFile(file) => {
                    return file.clone();
                }
                Either::NotReadFile(file) => {
                    let fileRef = file.take().unwrap();
                    return self.internalReadFile(*index, &fileRef);
                }
            }
        } else {
            panic!("File not found in visited files, but somehow we have an index that matches this path? Internal bug, pls report. Path: {path}");
        }
    }

    /// Can it access the file? Does not need to be previously opened.
    pub fn hasFileAccess(&mut self, path: &str) -> bool {
        let absolutePath = self.getPath(path);
        return absolutePath.is_ok();
    }

    fn hasFileAccessImpl(&mut self, absolutePath: &str) -> Result<u64, String> {
        if let Some(pos) = self.resolvedPaths.get(absolutePath) {
            return Ok(*pos);
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
            let pos = self.files.len() as u64;
            self.files.push(Either::NotReadFile(Some(file)));
            return Ok(pos);
        }
    }

    /// Get paths of current files opened.
    pub fn getCurrPaths(&self) -> Vec<u64> {
        let mut paths = Vec::new();
        for path in 1..self.files.len() {
            paths.push(path as u64);
        }
        return paths;
    }

    /// Add a fake test file. Intened for testing.
    pub fn addTestFile(&mut self, path: String, content: String) {
        self.resolvedPaths
            .insert(path.clone(), self.files.len() as u64);
        self.reverseResolved
            .insert(self.files.len() as u64, path.clone());
        self.files
            .push(Either::CompileFile(Arc::new(CompileFile::new(
                path, content,
            ))));
    }

    fn findBestPath(params: &Arc<Parameters>, pathStr: &str) -> Result<String, String> {
        let res: Result<PathBuf, String> = (|| {
            let path = Path::new(&pathStr).to_path_buf();
            if path.is_absolute() && path.exists() {
                return Ok(path);
            }
            for dir in &params.includeDirs {
                let resultingPath = Path::new(dir).join(&path);
                if resultingPath.exists() {
                    return Ok(resultingPath);
                }
            }
            for dir in &params.includeSystemDirs {
                let resultingPath = Path::new(dir).join(&path);
                if resultingPath.exists() {
                    return Ok(resultingPath);
                }
            }
            return Err(format!("Could not find file: {pathStr}"));
        })();
        res.map(|path| path.canonicalize().unwrap().to_str().unwrap().to_string())
    }

    /// Resolve a path. On error, return error.
    fn getPath(&mut self, pathStr: &str) -> Result<u64, String> {
        if let Some(v) = self.resolvedPaths.get(pathStr) {
            return Ok(*v);
        } else {
            let canonical = Self::findBestPath(&self.params, pathStr)?;
            if let Some(v) = self.resolvedPaths.get(&canonical) {
                let v = *v;
                self.resolvedPaths.insert(pathStr.to_string(), v);
                return Ok(v);
            } else {
                let pos = self.hasFileAccessImpl(&canonical)?;
                self.reverseResolved.insert(pos, canonical.clone());
                if canonical != pathStr {
                    self.resolvedPaths.insert(canonical, pos);
                }
                self.resolvedPaths.insert(pathStr.to_string(), pos);
                return Ok(pos);
            }
        }
    }
}
