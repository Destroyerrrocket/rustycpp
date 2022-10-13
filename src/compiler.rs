//! Main compiler driver. It pushes the machinery to do its thing!
#![warn(missing_docs)]

use std::collections::{HashMap, VecDeque};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::thread;

use threadpool::ThreadPool;

use crate::lex::lexer::Lexer;
use crate::module_tree::dependency_iterator::DependencyIterator;
use crate::module_tree::generate::generateDependencyTree;
use crate::module_tree::structs::ModuleTree;
use crate::parse::parser::Parser;
use crate::preprocessor::Preprocessor;
use crate::utils::compilerstate::CompilerState;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::statecompileunit::StateCompileUnit;
use crate::utils::structs::{CompileMsg, CompileMsgKind};

/// Path to a translation unit
pub type TranslationUnit = String;

/// Main driver of the compilation. It coordinates to compilation of the various
/// translation untis
#[derive(Debug)]
pub struct Compiler {
    /// State of the compiler
    compilerState: CompilerState,
    /// Threadpool
    pool: ThreadPool,
}

impl Compiler {
    /// Creates a new compiler with the given parameters
    pub fn new(parameters: Parameters) -> Self {
        let parameters = Arc::new(parameters);
        let mut compileFiles = FileMap::new(parameters.clone());
        for file in &parameters.translationUnits {
            compileFiles.getAddFile(file);
        }
        Self {
            compilerState: CompilerState {
                parameters,
                compileFiles: Arc::new(Mutex::new(compileFiles)),
                compileUnits: Arc::new(Mutex::new(HashMap::new())),
            },
            pool: ThreadPool::new(
                thread::available_parallelism()
                    .unwrap_or(unsafe { NonZeroUsize::new_unchecked(1) })
                    .get(),
            ),
        }
    }

    /// Parses the dependencies of the main TU, and sets up the state of the compiler accordingly
    pub fn prepareDependencyTreeAndSetupInitialState(
        &mut self,
    ) -> Result<ModuleTree, Vec<CompileMsg>> {
        let mainCompileFiles = self
            .compilerState
            .compileFiles
            .lock()
            .unwrap()
            .getCurrPaths();
        let tree = generateDependencyTree(&mainCompileFiles, &mut self.compilerState.compileFiles)?;
        let mut compileUnits = self.compilerState.compileUnits.lock().unwrap();
        for tu in &tree.roots {
            compileUnits.insert(tu.1.module.1.clone(), StateCompileUnit::new());
        }
        for tu in &tree.childModules {
            compileUnits.insert(tu.1.module.1.clone(), StateCompileUnit::new());
        }
        return Ok(tree);
    }

    /// Executes the preprocessing stage
    pub fn print_dependency_tree(&mut self) -> Result<(), Vec<CompileMsg>> {
        let tree = self.prepareDependencyTreeAndSetupInitialState()?;
        println!("Resulting module tree: {:?}", tree.roots);
        let dependencyIterator = DependencyIterator::new(&tree, 0);
        let mut tuDone = VecDeque::new();
        loop {
            while dependencyIterator.wouldLockIfNext() {
                if tuDone.is_empty() {
                    panic!("Internal error: Somehow we don't have any TU that are done left, but the dependency iterator is still locked!");
                }
                let tu = tuDone.pop_front().unwrap();
                dependencyIterator.markDone(&tu, 0);
                println!("=== Once {tu} completes ===");
            }
            let next = dependencyIterator.next();
            match next {
                Some(tu) => {
                    println!("{tu}");
                    tuDone.push_back(tu);
                }
                None => break,
            }
        }
        Ok(())
    }

    /// Executes the preprocessing stage
    pub fn print_preprocessor(&mut self) -> Result<(), Vec<CompileMsg>> {
        let tree = self.prepareDependencyTreeAndSetupInitialState()?;

        let dependencyIterator = Arc::new(DependencyIterator::new(&tree, 0));

        loop {
            let next = dependencyIterator.next();
            let dependencyIterator = dependencyIterator.clone();
            let compilerState = self.compilerState.clone();
            match next {
                Some(tu) => self.pool.execute(move || {
                    let mut output = format!("// file: {}\n", &tu);
                    for tok in Preprocessor::new((compilerState, &tu)) {
                        match tok {
                            Ok(tok) => {
                                output.push_str(tok.tokPos.tok.to_str());
                            }
                            Err(err) => {
                                log::info!("{}", err.to_string());
                                if err.severity() == CompileMsgKind::FatalError {
                                    panic!("Force stop. Unrecoverable error");
                                }
                            }
                        }
                    }
                    print!("{output}");
                    dependencyIterator.markDone(&tu, 1);
                }),
                None => break,
            }
        }
        self.pool.join();
        Ok(())
    }

    /// Executes the preprocessing stage and parses the tokens to its final token form
    pub fn print_lexer(&mut self) -> Result<(), Vec<CompileMsg>> {
        let tree = self.prepareDependencyTreeAndSetupInitialState()?;

        let dependencyIterator = Arc::new(DependencyIterator::new(&tree, 0));

        loop {
            let next = dependencyIterator.next();
            let dependencyIterator = dependencyIterator.clone();
            let compilerState = self.compilerState.clone();
            match next {
                Some(tu) => self.pool.execute(move || {
                    let mut output = format!("// file: {}\n", &tu);
                    let preprocessor = Preprocessor::new((compilerState, &tu));
                    let mut lexer = Lexer::new(preprocessor);
                    for tok in &mut lexer {
                        output.push_str(&format!("{:?}\n", tok.tokPos.tok));
                    }
                    let errors = lexer.errors();
                    if !errors.is_empty() {
                        output.push('\n');
                        for err in errors {
                            output.push_str(&err.to_string());
                            output.push('\n');
                        }
                    }
                    print!("{output}");
                    dependencyIterator.markDone(&tu, 1);
                }),
                None => break,
            }
        }
        self.pool.join();
        Ok(())
    }

    /// Parses the resulting tokens to an AST and prints it
    pub fn print_parsed_tree(&mut self) -> Result<(), Vec<CompileMsg>> {
        let tree = self.prepareDependencyTreeAndSetupInitialState()?;

        let dependencyIterator = Arc::new(DependencyIterator::new(&tree, 0));

        loop {
            let next = dependencyIterator.next();
            let dependencyIterator = dependencyIterator.clone();
            let compilerState = self.compilerState.clone();
            match next {
                Some(tu) => self.pool.execute(move || {
                    let mut output = format!("// file: {}\n", &tu);
                    let preprocessor = Preprocessor::new((compilerState.clone(), &tu));
                    let lexer = Lexer::new(preprocessor);
                    let mut parser = Parser::new(lexer, tu.clone(), compilerState);
                    match parser.parseStringTree() {
                        Ok(ast) => {
                            output.push_str(&ast);
                            output.push('\n');
                        }
                        Err(errors) => {
                            output.push('\n');
                            for err in errors {
                                output.push_str(&err.to_string());
                                output.push('\n');
                            }
                        }
                    }
                    print!("{output}");
                    dependencyIterator.markDone(&tu, 1);
                }),
                None => break,
            }
        }
        self.pool.join();
        Ok(())
    }

    /// Attempts to compile everything, until the last thing implemented.
    pub fn doTheThing(&mut self) -> Result<(), Vec<CompileMsg>> {
        self.print_parsed_tree()
    }
}
