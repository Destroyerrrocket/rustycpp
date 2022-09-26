use std::sync::{Arc, Mutex};

use crate::filemap::FileMap;
use crate::preprocessor::Preprocessor;
use crate::utils::structs::CompileMsgKind;

type TranslationUnit = String;
#[derive(Debug)]
pub struct Compiler {
    compileFiles: Arc<Mutex<FileMap>>,
    mainTranslationUnits: Vec<TranslationUnit>,
}

impl Compiler {
    pub fn new(compileFiles: FileMap) -> Self {
        let mainTranslationUnits = compileFiles.getCurrPaths();
        Self {
            compileFiles: Arc::new(Mutex::new(compileFiles)),
            mainTranslationUnits,
        }
    }

    pub fn print_preprocessor(&mut self) {
        for compFile in &self.mainTranslationUnits {
            log::info!("Applying preprocessor to: {}", &compFile);
            for prepro_token in Preprocessor::new((self.compileFiles.clone(), compFile)) {
                match prepro_token {
                    Ok(tok) => {
                        log::info!("{}", tok.tokPos.tok.to_str());
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
