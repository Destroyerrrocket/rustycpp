//! Main compiler driver. It pushes the machinery to do its thing!
#![warn(missing_docs)]

use std::sync::{Arc, Mutex};

use crate::preprocessor::Preprocessor;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::structs::CompileMsgKind;

/// Path to a translation unit
type TranslationUnit = String;

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
}
