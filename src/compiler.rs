//! Main compiler driver. It pushes the machinery to do its thing!
#![warn(missing_docs)]

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::module_tree::dependency_iterator::DependencyIterator;
use crate::module_tree::generate::generateDependencyTree;
use crate::preprocessor::Preprocessor;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::structs::CompileMsgKind;

/// Path to a translation unit
pub type TranslationUnit = String;

/// Main driver of the compilation. It coordinates to compilation of the various
/// translation untis
#[derive(Debug)]
pub struct Compiler {
    /// The parameters of the compilation
    parameters: Arc<Parameters>,
    /// The files opened by the compiler
    compileFiles: Arc<Mutex<FileMap>>,
    /// The translation units to compile
    mainTranslationUnits: Vec<TranslationUnit>,
}

impl Compiler {
    /// Creates a new compiler with the given parameters
    pub fn new(parameters: Parameters) -> Self {
        let parameters = Arc::new(parameters);
        let mut compileFiles = FileMap::new(parameters.clone());
        for file in &parameters.translationUnits {
            compileFiles.getAddFile(file);
        }
        let mainTranslationUnits = compileFiles.getCurrPaths();
        Self {
            parameters,
            compileFiles: Arc::new(Mutex::new(compileFiles)),
            mainTranslationUnits,
        }
    }

    /// Executes the preprocessing stage
    pub fn print_preprocessor(&mut self) {
        for compFile in &self.mainTranslationUnits {
            log::info!("Applying preprocessor to: {}", &compFile);
            print!("// Entering {}", compFile);
            for prepro_token in
                Preprocessor::new((self.parameters.clone(), self.compileFiles.clone(), compFile))
            {
                match prepro_token {
                    Ok(tok) => {
                        print!("{}", tok.tokPos.tok.to_str());
                    }
                    Err(err) => {
                        log::info!("{}", err.to_string());
                        if err.severity() == CompileMsgKind::FatalError {
                            panic!("Force stop. Unrecoverable error");
                        }
                    }
                }
            }
        }
    }

    /// Executes the preprocessing stage
    pub fn print_dependency_tree(&mut self) {
        match generateDependencyTree(&self.mainTranslationUnits, &mut self.compileFiles) {
            Ok(result) => {
                println!("Resulting module tree: {:?}", result.roots);
                let dependencyIterator = DependencyIterator::new(&result, 0);
                let mut tuDone = VecDeque::new();
                loop {
                    while dependencyIterator.wouldLockIfNext() {
                        if tuDone.is_empty() {
                            panic!("Internal error: Somehow we don't have any TU that are done left, but the dependency iterator is still locked!");
                        }
                        let tu = tuDone.pop_front().unwrap();
                        dependencyIterator.markDone(&tu, 0);
                        println!("=== Once {} completes ===", tu);
                    }
                    let next = dependencyIterator.next();
                    match next {
                        Some(tu) => {
                            println!("{}", tu);
                            tuDone.push_back(tu);
                        }
                        None => break,
                    }
                }
            }
            Err(err) => {
                for err in err {
                    log::error!("{}", err.to_string());
                }
            }
        }
    }

    /// Attempts to compile everything, until the last thing implemented.
    pub fn doTheThing(&mut self) {
        self.print_preprocessor();
    }
}
