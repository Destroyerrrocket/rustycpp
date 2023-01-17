//! Main compiler driver. It pushes the machinery to do its thing!
#![warn(missing_docs)]

use std::thread;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use std::{
    collections::{HashMap, VecDeque},
    sync::atomic::Ordering,
};
use std::{num::NonZeroUsize, sync::atomic::AtomicBool};

use threadpool::ThreadPool;

use crate::module_tree::generate::generateDependencyTree;
use crate::module_tree::structs::ModuleTree;
use crate::module_tree::{
    dependency_iterator::DependencyIterator, dependency_parser::parseModuleMacroOp,
};
use crate::parse::parser::Parser;
use crate::preprocessor::Preprocessor;
use crate::utils::compilerstate::CompilerState;
use crate::utils::filemap::FileMap;
use crate::utils::parameters::Parameters;
use crate::utils::statecompileunit::StateCompileUnit;
use crate::utils::structs::{CompileMsg, CompileMsgKind};
use crate::{ast::Tu::AstTu, utils::statecompileunit::StageCompileUnit};
use crate::{lex::lexer::Lexer, utils::moduleHeaderAtomicLexingList::ModuleHeaderAtomicLexingList};

/// Path to a translation unit
pub type TranslationUnit = u64;

/// Main driver of the compilation. It coordinates to compilation of the various
/// translation untis
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
        let mut translationUnits = HashSet::new();
        for file in &parameters.translationUnits {
            translationUnits.insert(compileFiles.getAddFile(file));
        }
        let mut moduleHeaderUnits = HashSet::new();
        for file in &parameters.moduleHeaderUnits {
            moduleHeaderUnits.insert(compileFiles.getAddFile(file));
        }

        let mut compileUnits = HashMap::new();
        for tu in translationUnits.iter().chain(moduleHeaderUnits.iter()) {
            compileUnits.insert(*tu, StateCompileUnit::new());
        }
        let threadNum = parameters.threadNum;
        Self {
            compilerState: CompilerState {
                parameters,
                compileFiles: Arc::new(Mutex::new(compileFiles)),
                compileUnits: Arc::new(compileUnits),
                translationUnitsFiles: Arc::new(translationUnits),
                moduleHeaderUnitsFiles: Arc::new(moduleHeaderUnits),
                foundErrors: Arc::new(AtomicBool::new(false)),
            },
            pool: ThreadPool::new(threadNum.unwrap_or_else(|| {
                thread::available_parallelism()
                    .unwrap_or(NonZeroUsize::new(1).unwrap())
                    .get()
            })),
        }
    }

    fn genDependencyTreeAndAggregateErrors(&mut self) -> Result<ModuleTree, Vec<CompileMsg>> {
        if self.compilerState.foundErrors.load(Ordering::Relaxed) {
            let mut err = Vec::new();
            for tu in self.compilerState.moduleHeaderUnitsFiles.iter() {
                err.extend(
                    self.compilerState
                        .compileUnits
                        .get(tu)
                        .unwrap()
                        .errors
                        .lock()
                        .unwrap()
                        .drain(..),
                );
            }
            for tu in self.compilerState.translationUnitsFiles.iter() {
                err.extend(
                    self.compilerState
                        .compileUnits
                        .get(tu)
                        .unwrap()
                        .errors
                        .lock()
                        .unwrap()
                        .drain(..),
                );
            }
            return Err(err);
        }
        generateDependencyTree(&self.compilerState)
    }

    // TODO: we are repeating the same function with a minnor difference (new_module_header vs new) in anoying different contexts... Can we mix them? I don't like repeating code...
    fn lexAllCompileModule(&mut self) -> Result<ModuleTree, std::vec::Vec<CompileMsg>> {
        let moduleHeaderAtomicLexingList = Arc::new(ModuleHeaderAtomicLexingList::new(
            self.pool
                .max_count()
                .min(self.compilerState.moduleHeaderUnitsFiles.len()),
        ));
        let mut executionFunction: Vec<Box<dyn Fn() + Send>> = vec![];
        for tu in self.compilerState.moduleHeaderUnitsFiles.iter().copied() {
            let compilerState = self.compilerState.clone();
            let moduleHeaderAtomicLexingList = moduleHeaderAtomicLexingList.clone();
            executionFunction.push(Box::new(move || {
                let compileUnit = compilerState.compileUnits.get(&tu).unwrap();
                let (toks, mut err, mut moduleDirectives) = {
                    compileUnit
                        .processingStage
                        .store(StageCompileUnit::Lexer, Ordering::Relaxed);

                    let preprocessor = Preprocessor::new_module_header(
                        (compilerState.clone(), tu),
                        moduleHeaderAtomicLexingList.clone(),
                    );
                    let mut lexer = Lexer::new(preprocessor);
                    let toks = (&mut lexer).collect::<Vec<_>>();
                    let moduleDirectivesPos = lexer.moduleDirectives();

                    // TODO: We are mixing parsing the module macros with lexing the file. This could be split.
                    let moduleDirectives = parseModuleMacroOp(tu, &toks, moduleDirectivesPos);
                    (toks, lexer.errors(), moduleDirectives)
                };
                if let Err(errModuleDirectives) = moduleDirectives.as_mut() {
                    err.append(errModuleDirectives);
                }

                if err.is_empty() {
                    *compileUnit.moduleOperations.lock().unwrap() = Some(moduleDirectives.unwrap());
                } else {
                    compileUnit.errors.lock().unwrap().extend(err);
                    compilerState.foundErrors.store(true, Ordering::Relaxed);
                }
                *compileUnit.tokens.lock().unwrap() = Some(toks);
                compileUnit
                    .finishedStage
                    .store(StageCompileUnit::Lexer, Ordering::Relaxed);
            }));
        }
        moduleHeaderAtomicLexingList.push(executionFunction);
        for _ in 0..self.pool.max_count() {
            let moduleHeaderAtomicLexingList = moduleHeaderAtomicLexingList.clone();
            self.pool.execute(move || {
                while let Some(exec) = moduleHeaderAtomicLexingList.pop() {
                    exec();
                }
            });
        }

        while self.pool.queued_count() != 0 {} // Just in case. header modules need to go first.

        for tu in self.compilerState.translationUnitsFiles.iter().copied() {
            let compilerState = self.compilerState.clone();
            self.pool.execute(move || {
                let compileUnit = compilerState.compileUnits.get(&tu).unwrap();
                let (toks, mut err, mut moduleDirectives) = {
                    compileUnit
                        .processingStage
                        .store(StageCompileUnit::Lexer, Ordering::Relaxed);

                    let preprocessor = Preprocessor::new((compilerState.clone(), tu));
                    let mut lexer = Lexer::new(preprocessor);

                    let toks = (&mut lexer).collect::<Vec<_>>();
                    let moduleDirectivesPos = lexer.moduleDirectives();

                    // TODO: We are mixing parsing the module macros with lexing the file. This could be split.
                    let moduleDirectives = parseModuleMacroOp(tu, &toks, moduleDirectivesPos);
                    (toks, lexer.errors(), moduleDirectives)
                };
                if let Err(errModuleDirectives) = moduleDirectives.as_mut() {
                    err.append(errModuleDirectives);
                }

                if err.is_empty() {
                    *compileUnit.moduleOperations.lock().unwrap() = Some(moduleDirectives.unwrap());
                } else {
                    compileUnit.errors.lock().unwrap().extend(err);
                    compilerState.foundErrors.store(true, Ordering::Relaxed);
                }
                *compileUnit.tokens.lock().unwrap() = Some(toks);
                compileUnit
                    .finishedStage
                    .store(StageCompileUnit::Lexer, Ordering::Relaxed);
            });
        }
        self.pool.join();
        assert!(self.pool.panic_count() == 0);
        self.genDependencyTreeAndAggregateErrors()
    }

    /// Executes the preprocessing stage
    pub fn print_dependency_tree(&mut self) -> Result<(), (CompilerState, Vec<CompileMsg>)> {
        let tree = self
            .lexAllCompileModule()
            .map_err(|err| (self.compilerState.clone(), err))?;
        println!("Resulting module tree: {:?}", tree.roots);
        let dependencyIterator = DependencyIterator::new(&tree, 0);
        let mut tuDone = VecDeque::new();
        loop {
            while dependencyIterator.wouldLockIfNext() {
                assert!(!tuDone.is_empty(), "Internal error: Somehow we don't have any TU that are done left, but the dependency iterator is still locked!");
                let tu = tuDone.pop_front().unwrap();
                dependencyIterator.markDone(tu, 0);
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
    pub fn print_preprocessor(&mut self) -> Result<(), (CompilerState, Vec<CompileMsg>)> {
        let tree = self
            .lexAllCompileModule()
            .map_err(|err| (self.compilerState.clone(), err))?;

        let dependencyIterator = Arc::new(DependencyIterator::new(&tree, 0));

        loop {
            let next = dependencyIterator.next();
            let dependencyIterator = dependencyIterator.clone();
            let compilerState = self.compilerState.clone();
            match next {
                Some(tu) => self.pool.execute(move || {
                    let mut output = format!("// file: {}\n", &tu);
                    for tok in Preprocessor::new((compilerState.clone(), tu)) {
                        match tok {
                            Ok(tok) => {
                                output.push_str(tok.tokPos.tok.to_str());
                            }
                            Err(err) => {
                                log::info!("{}", err.to_string(&compilerState.compileFiles));
                                assert!(
                                    err.severity() != CompileMsgKind::FatalError,
                                    "Force stop. Unrecoverable error"
                                );
                            }
                        }
                    }
                    print!("{output}");
                    dependencyIterator.markDone(tu, 1);
                }),
                None => break,
            }
        }
        self.pool.join();
        assert!(self.pool.panic_count() == 0);
        Ok(())
    }

    /// Executes the preprocessing stage and parses the tokens to its final token form
    pub fn print_lexer(&mut self) -> Result<(), (CompilerState, Vec<CompileMsg>)> {
        self.lexAllCompileModule()
            .map_err(|err| (self.compilerState.clone(), err))?;

        #[allow(clippy::significant_drop_in_scrutinee)]
        for (tu, compileUnit) in self.compilerState.compileUnits.iter() {
            let mut output = format!("// file: {}\n", &tu);
            for tok in compileUnit.tokens.lock().unwrap().as_ref().unwrap().iter() {
                output.push_str(&format!("{:?}\n", tok.tokPos.tok));
            }
            print!("{output}");
        }
        Ok(())
    }

    /// Parses the resulting tokens to an AST and prints it
    pub fn print_parsed_tree(&mut self) -> Result<(), (CompilerState, Vec<CompileMsg>)> {
        let tree = self
            .lexAllCompileModule()
            .map_err(|err| (self.compilerState.clone(), err))?;

        let dependencyIterator = Arc::new(DependencyIterator::new(&tree, 0));

        loop {
            let next = dependencyIterator.next();
            let dependencyIterator = dependencyIterator.clone();
            let compilerState = self.compilerState.clone();
            match next {
                Some(tu) => self.pool.execute(move || {
                    let compileUnit = compilerState.compileUnits.get(&tu).unwrap();
                    compileUnit
                        .processingStage
                        .store(StageCompileUnit::Parser, Ordering::Relaxed);
                    let mut output = format!("// file: {}\n", &tu);
                    let mut parser = Parser::new(
                        compileUnit.tokens.lock().unwrap().take().unwrap(),
                        tu,
                        compilerState.clone(),
                    );
                    let (ast, errors) = parser.parse();

                    output.push('\n');
                    for err in errors {
                        output.push_str(&err.to_string(&compilerState.compileFiles));
                        output.push('\n');
                    }
                    output.push_str(&Parser::printStringTree(&ast));
                    output.push('\n');

                    print!("{output}");
                    dependencyIterator.markDone(tu, 1);
                    compileUnit
                        .finishedStage
                        .store(StageCompileUnit::Parser, Ordering::Relaxed);
                }),
                None => break,
            }
        }
        self.pool.join();
        assert!(self.pool.panic_count() == 0);
        Ok(())
    }

    /// Parses the resulting tokens to an AST and returns it
    pub fn parsed_tree_test(
        &mut self,
        result: &mut (HashMap<String, AstTu>, Vec<CompileMsg>),
    ) -> Result<CompilerState, (CompilerState, Vec<CompileMsg>)> {
        let tree = match self.lexAllCompileModule() {
            Ok(tree) => tree,
            Err(err) => {
                result.1.extend(err);
                return Ok(self.compilerState.clone());
            }
        };

        let dependencyIterator = Arc::new(DependencyIterator::new(&tree, 0));
        let resultLoc = Arc::new(Mutex::new((HashMap::new(), vec![])));

        loop {
            let next = dependencyIterator.next();
            let dependencyIterator = dependencyIterator.clone();
            let compilerState = self.compilerState.clone();
            let result = resultLoc.clone();
            match next {
                Some(tu) => self.pool.execute(move || {
                    let compileUnit = compilerState.compileUnits.get(&tu).unwrap();
                    compileUnit
                        .processingStage
                        .store(StageCompileUnit::Parser, Ordering::Relaxed);
                    let mut parser = Parser::new(
                        compileUnit.tokens.lock().unwrap().take().unwrap(),
                        tu,
                        compilerState.clone(),
                    );
                    let (ast, errors) = parser.parse();
                    let mut res = result.lock().unwrap();
                    res.0.insert(
                        compilerState
                            .compileFiles
                            .lock()
                            .unwrap()
                            .getOpenedFile(tu)
                            .path()
                            .clone(),
                        ast,
                    );
                    res.1.extend(errors);
                    dependencyIterator.markDone(tu, 1);
                    compileUnit
                        .finishedStage
                        .store(StageCompileUnit::Parser, Ordering::Relaxed);
                }),
                None => break,
            }
        }
        self.pool.join();
        assert!(self.pool.panic_count() == 0);
        let res = resultLoc.lock().unwrap().clone();
        result.0 = res.0;
        result.1 = res.1;
        Ok(self.compilerState.clone())
    }

    /// Attempts to compile everything, until the last thing implemented.
    pub fn doTheThing(&mut self) -> Result<(), (CompilerState, Vec<CompileMsg>)> {
        self.print_parsed_tree()
    }
}
