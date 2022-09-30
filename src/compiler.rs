use std::sync::{Arc, Mutex};

use crate::preprocessor::Preprocessor;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::structs::CompileMsgKind;

type TranslationUnit = String;
#[derive(Debug)]
pub struct Compiler {
    parameters: Arc<Parameters>,
    compileFiles: Arc<Mutex<FileMap>>,
    mainTranslationUnits: Vec<TranslationUnit>,
}

impl Compiler {
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
