//! The Preprocessor is responsible for evaluating the step 4 of the C++
//! translation.
#![allow(clippy::too_many_lines)]

use crate::{
    compiler::TranslationUnit,
    fileTokPosMatchArm,
    utils::{
        compilerstate::CompilerState, moduleHeaderAtomicLexingList::ModuleHeaderAtomicLexingList,
        statecompileunit::StageCompileUnit, structs::TokPos,
    },
};
use std::{
    collections::{HashMap, VecDeque},
    sync::{atomic::Ordering, Arc},
    time::Instant,
};

use crate::{
    fileTokPosMatches,
    grammars::defineast::DefineAst,
    utils::structs::{CompileError, CompileMsg, CompileMsgImpl, CompileWarning, FileTokPos},
};

use multiset::HashMultiSet;

use super::{
    multilexer::MultiLexer,
    pretoken::{PreToken, PreprocessingOperator},
};

mod custommacros;
mod defineparse;
mod includer;
mod macroexpand;
mod macroexpression;

#[derive(Debug, PartialEq)]
/// The current #if scope is in status...
enum ScopeStatus {
    /// Successful. We return all tokens
    Success,
    /// Failure. We return no tokens
    Failure,
    /// Already been successful. Can happen in:
    /// Example:
    /// #if 1
    /// #else
    /// <here>
    /// #endif
    /// We return no tokens, and won't return any in any future scope change.
    AlreadySucceeded,
}

#[derive(Debug)]
/// The preprocessor is an iterable object that generates tokens from the
/// original input file. It will report any preprocessing errors as it
/// encounters them, previous to returning the possibly incorrect tokens.
pub struct Preprocessor {
    /// Main file TU
    tu: u64,
    /// Parameters of the compilation
    compilerState: CompilerState,
    /// The multilexer is the object that will generate the pretokens tokens
    multilexer: MultiLexer,
    /// The generated preprocessing tokens to be returned. This is a stash, as
    /// some tokens may generate more than one token.
    generated: VecDeque<FileTokPos<PreToken>>,
    /// The generated errors be returned. This is a stash, as some tokens may
    /// generate more than one error.
    errors: VecDeque<CompileMsg>,
    /// The #if scope status. Keeps track of what to do in this scope.
    scope: Vec<ScopeStatus>,
    /// Current definitions in the preprocessor.
    definitions: HashMap<String, DefineAst>,
    /// Macros that are disabled in the preprocessor at this point in the evaluation.
    disabledMacros: HashMultiSet<String>,
    /// The preprocessor is at the start of a line. No tokens have been found
    /// yet in this one, except for whitespace.
    atStartLine: bool,
    /// The preprocessor is at the end of the file. None already sent to next stage.
    alreadyEmittedEnd: bool,
    /// If we are parsing a module header, this is the ditributor of tasks so we can parse another file.
    moduleHeaderAtomicLexingList: Option<Arc<ModuleHeaderAtomicLexingList>>,
}

impl Preprocessor {
    /// Creates a new preprocessor from the given parameters, filemap and path
    pub fn new(data: (CompilerState, TranslationUnit)) -> Self {
        Self {
            tu: data.1,
            compilerState: data.0.clone(),
            multilexer: MultiLexer::new((data.0.compileFiles, data.1)),
            generated: VecDeque::new(),
            errors: VecDeque::new(),
            scope: vec![],
            definitions: HashMap::new(),
            disabledMacros: HashMultiSet::new(),
            atStartLine: true,
            alreadyEmittedEnd: false,
            moduleHeaderAtomicLexingList: None,
        }
        .initCustomMacros()
    }

    pub fn new_module_header(
        data: (CompilerState, TranslationUnit),
        moduleHeaderAtomicLexingList: Arc<ModuleHeaderAtomicLexingList>,
    ) -> Self {
        Self {
            tu: data.1,
            compilerState: data.0.clone(),
            multilexer: MultiLexer::new((data.0.compileFiles, data.1)),
            generated: VecDeque::new(),
            errors: VecDeque::new(),
            scope: vec![],
            definitions: HashMap::new(),
            disabledMacros: HashMultiSet::new(),
            atStartLine: true,
            alreadyEmittedEnd: false,
            moduleHeaderAtomicLexingList: Some(moduleHeaderAtomicLexingList),
        }
        .initCustomMacros()
    }

    /// Creates a new preprocessor from the given parameters, filemap and path
    fn undefineMacro(&mut self, preToken: &FileTokPos<PreToken>) {
        let vecPrepro = Iterator::take_while(&mut self.multilexer, |pre| {
            pre.tokPos.tok != PreToken::Newline
        });

        match vecPrepro.into_iter().find(|e| !e.tokPos.tok.isWhitespace()) {
            None => {
                self.errors.push_back(CompileError::fromPreTo(
                    "Expected an identifier to undefine",
                    preToken,
                ));
            }
            Some(e) => match e.tokPos.tok {
                PreToken::Ident(id) => {
                    if self.definitions.remove(&id).is_none() {
                        self.errors.push_back(CompileError::fromPreTo(
                            format!("Macro {id} is not defined when reached"),
                            preToken,
                        ));
                    }
                }
                _ => {
                    self.errors.push_back(CompileError::fromPreTo(
                        format!("Expected an identifier, found: {}", e.tokPos.tok.to_str()),
                        preToken,
                    ));
                }
            },
        }
        log::trace!("Macros:");
        for defi in self.definitions.values() {
            log::trace!("{:?}", defi);
        }
    }

    /// Consume a #ifdef or #ifndef and return the macro name if it exists
    fn consumeMacroDef(&mut self, _PreToken: &FileTokPos<PreToken>) -> Option<String> {
        let identStr;
        loop {
            let inIdent = self.multilexer.next();
            match inIdent {
                None => {
                    return None;
                }
                Some(ident) => match ident.tokPos.tok {
                    PreToken::Ident(str) => {
                        identStr = str;
                        break;
                    }
                    PreToken::Whitespace(_) => {
                        continue;
                    }
                    PreToken::Newline => {
                        return None;
                    }
                    _ => {
                        self.reachNl();
                        return None;
                    }
                },
            }
        }
        self.reachNl();
        Some(identStr)
    }

    /// Reach the nl.
    fn reachNl(&mut self) -> VecDeque<FileTokPos<PreToken>> {
        let mut toks = VecDeque::new();
        loop {
            let inIdent = self.multilexer.next();
            match inIdent {
                None => {
                    return toks;
                }
                Some(ident) => {
                    toks.push_back(ident);
                    if toks.back().unwrap().tokPos.tok == PreToken::Newline {
                        return toks;
                    }
                }
            }
        }
    }

    /// Evaluate the macro in an #ifdef/#ifndef directive
    fn evalIfDef(&self, def: Option<String>) -> bool {
        if let Some(macroName) = def {
            return self.definitions.contains_key(&macroName);
        }
        false
    }

    /// If applicable, generate a module token
    fn moduleDirective(&mut self, module: FileTokPos<PreToken>) -> VecDeque<FileTokPos<PreToken>> {
        let mut toks = self.reachNl();
        let isDirective = toks
            .iter()
            .rev()
            .nth(1)
            .is_some_and(|t| fileTokPosMatches!(t, PreToken::OperatorPunctuator(";")));

        if module.file == self.tu && isDirective {
            let mut paramLexer = MultiLexer::new_def(self.multilexer.fileMapping());
            paramLexer.pushTokensDec(toks);
            let expandedTokens = Self::expandASequenceOfTokens(
                &self.compilerState,
                paramLexer,
                &self.definitions,
                &self.disabledMacros,
            );
            if let Err(err) = expandedTokens {
                self.errors.push_back(err);
                return VecDeque::new();
            }
            let mut expandedTokens = expandedTokens.unwrap();
            expandedTokens.push_front(FileTokPos::new_meta_c(PreToken::Module, &module));
            return expandedTokens;
        }

        self.atStartLine = false;
        toks.push_front(module);
        self.multilexer.pushTokensDec(toks);

        VecDeque::new()
    }

    /// We know there is a dependency loop, so we need to find it.
    /// Returns the paths of the headers in the loop.
    fn getDependencyLoop(&self) -> (Vec<String>, bool) {
        let mut loopVec = Vec::new();
        let startLoop = self.tu;
        let mut current = self.tu;
        let loopFound = loop {
            let filePath = self
                .compilerState
                .compileFiles
                .lock()
                .unwrap()
                .getOpenedFile(current)
                .path()
                .clone();
            loopVec.push(filePath);
            let nextTu = self
                .compilerState
                .compileUnits
                .get(&current)
                .unwrap()
                .blockedByImportHeader
                .load(Ordering::Relaxed);
            if nextTu == 0 {
                break false;
            } else if nextTu == startLoop {
                loopVec.push(loopVec.first().unwrap().clone());
                break true;
            }
            current = nextTu;
        };
        (loopVec, loopFound)
    }

    fn importHeaderDirectiveGetDefinitions(
        &mut self,
        tu: TranslationUnit,
        includePath: &str,
        import: &FileTokPos<PreToken>,
    ) -> Option<HashMap<String, DefineAst>> {
        if !self.compilerState.moduleHeaderUnitsFiles.contains(&tu) {
            self.errors.push_back(
            CompileError::fromPreTo(format!("You must define in your project configuration that you explicitly want to import the header file at path: {includePath}"), import)
        );
            return None;
        }

        /*
        Time to decide what to do with the header, depending on the stage.
        - If we are preprocessing a normal tu, we just need to block for the header to finish (it may still be running)
        and then just return the definitions.

        - If we are preprocessing a header, we need to check if the included header is being preprocessed by another thread.
        If it is, DON'T block, will try to then get another task of preprocessing a header.
          - If there is another task, we'll start that task, marking this header as blocked by import of this other header.
          - If there is no other task, we'll block for the header to finish and then return the definitions.
            - If the number of blocked preprocessors equals the number of threads, we need to bail out; To do so, we'll
            have to report an error explaining that there is a loop somewhere in the import graph.
        */
        let importableHeader = self.compilerState.compileUnits.get(&tu).unwrap();

        let normalTu = self.moduleHeaderAtomicLexingList.is_none(); // Normal TU don't have a list of headers to lex
        if normalTu {
            while importableHeader.finishedStage.load(Ordering::Relaxed) != StageCompileUnit::Lexer
            {
                // Wait for the header to finish. Hot loop, it shouldn't take that long.
                std::thread::yield_now();
            }
            return Some(
                importableHeader
                    .macroDefintionsAtTheEndOfTheFile
                    .lock()
                    .unwrap()
                    .clone(),
            );
        }
        // We are in a header file, we need to check if the header is being preprocessed by another thread.
        if importableHeader.finishedStage.load(Ordering::Relaxed) == StageCompileUnit::Lexer {
            // It is done! return directly
            return Some(
                importableHeader
                    .macroDefintionsAtTheEndOfTheFile
                    .lock()
                    .unwrap()
                    .clone(),
            );
        }
        // It is not done, we need to see if there is another task available.
        if let Some(task) = self.moduleHeaderAtomicLexingList.as_ref().unwrap().pop() {
            // There is another task, we'll start that task, marking this header as blocked by import of this other header.
            self.compilerState
                .compileUnits
                .get(&self.tu)
                .unwrap()
                .blockedByImportHeader
                .store(tu, Ordering::Relaxed);
            task();
            self.compilerState
                .compileUnits
                .get(&self.tu)
                .unwrap()
                .blockedByImportHeader
                .store(0, Ordering::Relaxed);
        } else {
            if self
                .moduleHeaderAtomicLexingList
                .as_ref()
                .unwrap()
                .markThreadLocked()
            {
                self.compilerState
                    .compileUnits
                    .get(&self.tu)
                    .unwrap()
                    .blockedByImportHeader
                    .store(tu, Ordering::Relaxed);
                // We are the last thread, we need to bail out; To do so, we'll have to report an error explaining that there is a loop somewhere in the import graph.
                self.errors.push_back(CompileError::fromPreTo(
                    format!(
                        "There is a loop in the import graph of the module header files: {}",
                        self.getDependencyLoop().0.join(" -> ")
                    ),
                    import,
                ));
                self.compilerState
                    .compileUnits
                    .get(&self.tu)
                    .unwrap()
                    .blockedByImportHeader
                    .store(0, Ordering::Relaxed);
                return None;
            }
            self.compilerState
                .compileUnits
                .get(&self.tu)
                .unwrap()
                .blockedByImportHeader
                .store(tu, Ordering::Relaxed);
            // There is no other task, we'll block for the header to finish and then return the definitions.
            let mut start = Instant::now();
            while importableHeader.finishedStage.load(Ordering::Relaxed) != StageCompileUnit::Lexer
            {
                // Wait for the header to finish. Hot loop, it shouldn't take that long.
                std::thread::yield_now();
                if start.elapsed().as_millis() > 5 {
                    let looping = self.getDependencyLoop();
                    // This can happen if we have a very small loop that allows other threads to get eahead...
                    if looping.1 {
                        self.errors.push_back(CompileError::fromPreTo(
                            format!(
                            "There is a loop in the import graph of the module header files: {}",
                            looping.0.join(" -> ")
                        ),
                            import,
                        ));

                        break;
                    }
                    start = Instant::now();
                }
            }
            self.compilerState
                .compileUnits
                .get(&self.tu)
                .unwrap()
                .blockedByImportHeader
                .store(0, Ordering::Relaxed);

            self.moduleHeaderAtomicLexingList
                .as_ref()
                .unwrap()
                .markThreadUnlocked();

            if importableHeader.finishedStage.load(Ordering::Relaxed) != StageCompileUnit::Lexer {
                return None;
            }
        }
        return Some(
            importableHeader
                .macroDefintionsAtTheEndOfTheFile
                .lock()
                .unwrap()
                .clone(),
        );
    }

    /// If applicable, generate a import token
    fn importDirective(&mut self, import: FileTokPos<PreToken>) -> VecDeque<FileTokPos<PreToken>> {
        let mut toks = self.reachNl();
        let isDirective = toks
            .iter()
            .rev()
            .nth(1)
            .is_some_and(|t| fileTokPosMatches!(t, PreToken::OperatorPunctuator(";")));
        if self.moduleHeaderAtomicLexingList.is_none() /*module headers can't have explicit import directives*/ && import.file == self.tu && isDirective
        {
            let mut paramLexer = MultiLexer::new_def(self.multilexer.fileMapping());
            paramLexer.pushTokensDec(toks);
            let expandedTokens = Self::expandASequenceOfTokens(
                &self.compilerState,
                paramLexer,
                &self.definitions,
                &self.disabledMacros,
            );
            if let Err(err) = expandedTokens {
                self.errors.push_back(err);
                return VecDeque::new();
            }
            let mut expandedTokens = expandedTokens.unwrap();

            if let Some(includePath) = Self::checkForInclude(&expandedTokens) {
                let tu = {
                    let mut compileFiles = self.compilerState.compileFiles.lock().unwrap();
                    compileFiles.getAddFile(&includePath)
                };

                let otherDefinitions =
                    self.importHeaderDirectiveGetDefinitions(tu, &includePath, &import);
                if otherDefinitions.is_none() {
                    return VecDeque::new();
                }
                self.definitions.extend(otherDefinitions.unwrap());

                // Remove the header path
                let pathTok = expandedTokens.pop_front().unwrap();
                // Insert a new special token for the later stages
                expandedTokens.push_front(FileTokPos::new_meta_c(
                    PreToken::ImportableHeaderName(tu),
                    &pathTok,
                ));
            }
            expandedTokens.push_front(FileTokPos::new_meta_c(PreToken::Import, &import));
            return expandedTokens;
        }

        self.atStartLine = false;
        toks.push_front(import);
        self.multilexer.pushTokensDec(toks);

        VecDeque::new()
    }

    /// If applicable, generate a module/import token
    fn exportDirective(&mut self, export: FileTokPos<PreToken>) -> VecDeque<FileTokPos<PreToken>> {
        let mut whitespaces = VecDeque::new();
        let tok = loop {
            let tok = self.multilexer.next();
            if let Some(fileTokPosMatchArm!(PreToken::Whitespace(_))) = tok {
                whitespaces.push_back(tok.unwrap());
                continue;
            }
            break tok;
        };
        if let Some(fileTokPosMatchArm!(ref tokie)) = tok {
            match tokie {
                PreToken::Ident(ref id) if id == "import" => {
                    let mut toks = self.importDirective(tok.unwrap());
                    if !toks.is_empty() {
                        for w in whitespaces {
                            toks.push_front(w);
                        }
                        toks.push_front(export);
                        return toks;
                    }
                }
                PreToken::Ident(ref id) if id == "module" => {
                    let mut toks = self.moduleDirective(tok.unwrap());
                    if !toks.is_empty() {
                        for w in whitespaces {
                            toks.push_front(w);
                        }
                        toks.push_front(export);
                        return toks;
                    }
                }
                _ => {
                    self.atStartLine = false;
                    whitespaces.push_front(export);
                    whitespaces.push_back(tok.unwrap());
                    self.multilexer.pushTokensDec(whitespaces);
                }
            };
        } else {
            self.atStartLine = false;
            whitespaces.push_front(export);
            self.multilexer.pushTokensDec(whitespaces);
        }

        self.atStartLine = false;
        VecDeque::new()
    }

    /// Encountered a preprocessor directive. Evaluate it accordingly, alering
    /// the state of the preprocessor.
    fn preprocessorDirective(&mut self, _PreToken: &FileTokPos<PreToken>) {
        let operation;
        let enabledBlock = matches!(self.scope.last(), Some(ScopeStatus::Success) | None);
        loop {
            match self.multilexer.next() {
                None => {
                    return;
                }
                Some(tok) => match tok.tokPos.tok {
                    PreToken::Newline => {
                        return;
                    }
                    PreToken::Whitespace(_) => {}
                    _ => {
                        operation = tok;
                        break;
                    }
                },
            }
        }
        if enabledBlock {
            match operation.tokPos.tok.to_str() {
                "include" => {
                    self.multilexer.expectHeader();
                    match self.consumeMacroInclude(&operation) {
                        Ok(path) => {
                            let tuModuleHeader = {
                                let mut compileFiles =
                                    self.compilerState.compileFiles.lock().unwrap();
                                compileFiles
                                    .getPath(&path)
                                    .ok()
                                    .and_then(|path| {
                                        self.compilerState.moduleHeaderUnitsFiles.get(&path)
                                    })
                                    .copied()
                            };
                            if let Some(tu) = tuModuleHeader {
                                let otherDefinitions =
                                    self.importHeaderDirectiveGetDefinitions(tu, &path, &operation);
                                if let Some(otherDefinitions) = otherDefinitions {
                                    self.definitions.extend(otherDefinitions);
                                }
                                self.generated.push_back(FileTokPos::new_meta_c(
                                    PreToken::Import,
                                    &operation,
                                ));
                                self.generated.push_back(FileTokPos::new_meta_c(
                                    PreToken::ImportableHeaderName(tu),
                                    &operation,
                                ));
                            } else if let Err(err) = self.includeFile(&operation, &path) {
                                self.errors.push_back(err);
                            }
                        }
                        Err(err) => {
                            self.errors.push_back(err);
                        }
                    }
                }
                "define" => {
                    self.defineMacro(&operation);
                }
                "undef" => {
                    self.undefineMacro(&operation);
                }
                "if" => {
                    let sequenceToEval = self.consumeMacroExpr();
                    match sequenceToEval {
                        Err(err) => {
                            self.errors.push_back(err);
                        }
                        Ok(sequenceToEval) => {
                            match Self::evalIfScope(&sequenceToEval, &operation) {
                                Ok((b, err)) => {
                                    self.errors.extend(err);
                                    if b {
                                        self.scope.push(ScopeStatus::Success);
                                    } else {
                                        self.scope.push(ScopeStatus::Failure);
                                    }
                                }
                                Err(err) => {
                                    self.errors.extend(err);
                                }
                            }
                        }
                    }
                }
                "ifdef" => {
                    let t = self.consumeMacroDef(&operation);
                    self.scope.push(if self.evalIfDef(t) {
                        ScopeStatus::Success
                    } else {
                        ScopeStatus::Failure
                    });
                }
                "ifndef" => {
                    let t = self.consumeMacroDef(&operation);
                    let t2 = if self.evalIfDef(t) {
                        ScopeStatus::Failure
                    } else {
                        ScopeStatus::Success
                    };
                    self.scope.push(t2);
                }
                "elif" | "else" => {
                    if let Some(scope) = self.scope.last_mut() {
                        *scope = ScopeStatus::AlreadySucceeded;
                        self.reachNl(); // TODO: Check empty in else
                    } else {
                        self.errors.push_back(CompileError::fromPreTo(
                            "Missmatched preprocessor conditional block",
                            &operation,
                        ));
                    }
                }
                "pragma" => {
                    self.errors.push_back(CompileError::fromPreTo("LMAO, you really expected me to implement this now XD. No worries, we'll get there :D", &operation));
                    self.reachNl();
                }
                "endif" => {
                    if self.scope.is_empty() {
                        self.errors.push_back(CompileError::fromPreTo(
                            "Missmatched preprocessor conditional block",
                            &operation,
                        ));
                    } else {
                        self.scope.pop();
                    }
                    self.reachNl(); // TODO: Check empty
                }
                "error" => {
                    let mut msg = String::new();
                    for t in Iterator::take_while(&mut self.multilexer, |pre| {
                        pre.tokPos.tok != PreToken::Newline
                    }) {
                        msg.push_str(t.tokPos.tok.to_str());
                    }
                    self.errors
                        .push_back(CompileError::fromPreTo(msg, &operation));
                }
                "warning" => {
                    let mut msg = String::new();
                    for t in Iterator::take_while(&mut self.multilexer, |pre| {
                        pre.tokPos.tok != PreToken::Newline
                    }) {
                        msg.push_str(t.tokPos.tok.to_str());
                    }
                    self.errors
                        .push_back(CompileWarning::fromPreTo(msg, &operation));
                }
                _ => {
                    self.errors.push_back(CompileError::fromPreTo(
                        "I do not know this preprocessing expression yet! I'm learning though :)",
                        &operation,
                    ));
                    self.reachNl();
                }
            }
        } else if &ScopeStatus::Failure == self.scope.last().unwrap() {
            match operation.tokPos.tok.to_str() {
                "if" | "ifdef" | "ifndef" => {
                    self.scope.push(ScopeStatus::AlreadySucceeded);
                }
                "elif" => {
                    let sequenceToEval = self.consumeMacroExpr();
                    match sequenceToEval {
                        Err(err) => {
                            self.errors.push_back(err);
                        }
                        Ok(sequenceToEval) => {
                            match Self::evalIfScope(&sequenceToEval, &operation) {
                                Ok((true, err)) => {
                                    let scope = self.scope.last_mut().unwrap();
                                    *scope = ScopeStatus::Success;
                                    self.errors.extend(err);
                                }
                                Ok((false, err)) | Err(err) => {
                                    self.errors.extend(err);
                                }
                            };
                        }
                    }
                }
                "else" => {
                    let scope = self.scope.last_mut().unwrap();
                    *scope = ScopeStatus::Success;
                    self.reachNl(); // TODO: Check it is empty
                }
                "endif" => {
                    self.reachNl(); // TODO: Check it is empty
                    self.scope.pop();
                }
                _ => {
                    self.reachNl();
                }
            }
        } else if &ScopeStatus::AlreadySucceeded == self.scope.last().unwrap() {
            match operation.tokPos.tok.to_str() {
                "if" | "ifdef" | "ifndef" => {
                    self.reachNl();
                    self.scope.push(ScopeStatus::AlreadySucceeded);
                }
                "endif" => {
                    self.reachNl(); // TODO: Check empty
                    self.scope.pop();
                }
                _ => {
                    self.reachNl();
                }
            }
        }
    }

    /// Consumes a new token generated, and depending on the state of the
    /// preprocessor, does something with it. Might consume more tokens from the
    /// lexer.
    fn consume(&mut self, newToken: FileTokPos<PreToken>) {
        loop {
            match self.scope.last() {
                Some(ScopeStatus::Success) | None => {
                    if self.atStartLine {
                        match newToken.tokPos.tok {
                            PreToken::Whitespace(_) | PreToken::Newline => {
                                self.generated.push_back(newToken);
                                break;
                            }
                            PreToken::PreprocessingOperator(PreprocessingOperator::Hash) => {
                                self.preprocessorDirective(&newToken);
                                break;
                            }

                            // Module directives
                            PreToken::Ident(ref import) if import == "import" => {
                                if self.definitions.contains_key("module") {
                                    self.atStartLine = false;
                                    continue;
                                }
                                let tokies = self.importDirective(newToken);
                                self.generated.extend(tokies);
                                break;
                            }
                            PreToken::Ident(ref module) if module == "module" => {
                                if self.definitions.contains_key("module") {
                                    self.atStartLine = false;
                                    continue;
                                }
                                let tokies = self.moduleDirective(newToken);
                                self.generated.extend(tokies);
                                break;
                            }
                            PreToken::Keyword("export") => {
                                if self.definitions.contains_key("export") {
                                    self.atStartLine = false;
                                    continue;
                                }
                                let tokies = self.exportDirective(newToken);
                                self.generated.extend(tokies);
                                break;
                            }

                            _ => {
                                self.atStartLine = false;
                                continue;
                            }
                        }
                    }
                    match newToken.tokPos.tok {
                        PreToken::EnableMacro(macroName) => {
                            self.disabledMacros.remove(&macroName);
                            break;
                        }
                        PreToken::DisableMacro(macroName) => {
                            self.disabledMacros.insert(macroName);
                            break;
                        }
                        PreToken::Newline => {
                            self.atStartLine = true;
                            self.generated.push_back(newToken);
                            break;
                        }
                        PreToken::Ident(_) => {
                            let toks = self.macroExpand(newToken);
                            match toks {
                                Ok(toks) => {
                                    self.generated.append(
                                        &mut toks
                                            .into_iter()
                                            .collect::<VecDeque<FileTokPos<PreToken>>>(),
                                    );
                                }
                                Err(err) => {
                                    self.errors.push_back(err);
                                }
                            };
                            break;
                        }
                        _ => {
                            self.generated.push_back(newToken);
                            break;
                        }
                    }
                }
                _ => {
                    if self.atStartLine {
                        match newToken.tokPos.tok {
                            PreToken::Whitespace(_) | PreToken::Newline => {
                                break;
                            }
                            PreToken::PreprocessingOperator(PreprocessingOperator::Hash) => {
                                self.preprocessorDirective(&newToken);
                                break;
                            }

                            // Module directives
                            PreToken::Ident(ref import) if import == "import" => {
                                let tokies = self.importDirective(newToken);
                                self.generated.extend(tokies);
                                break;
                            }
                            PreToken::Ident(ref module) if module == "module" => {
                                let tokies = self.moduleDirective(newToken);
                                self.generated.extend(tokies);
                                break;
                            }
                            PreToken::Keyword("export") => {
                                let tokies = self.exportDirective(newToken);
                                self.generated.extend(tokies);
                                break;
                            }

                            _ => {
                                self.atStartLine = false;
                                break;
                            }
                        }
                    }
                    match newToken.tokPos.tok {
                        PreToken::Newline => {
                            self.atStartLine = true;
                            break;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
        }
    }
}
impl Iterator for Preprocessor {
    type Item = Result<FileTokPos<PreToken>, CompileMsg>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(err) = self.errors.pop_front() {
                return Some(Err(err));
            }
            match self.generated.pop_front() {
                Some(tok) => {
                    return Some(Ok(tok));
                }
                None => match self.multilexer.next() {
                    None => {
                        if !self.alreadyEmittedEnd {
                            self.compilerState
                                .compileUnits
                                .get(&self.tu)
                                .unwrap()
                                .macroDefintionsAtTheEndOfTheFile
                                .lock()
                                .unwrap()
                                .extend(self.definitions.clone());
                            self.alreadyEmittedEnd = true;
                        }
                        return None;
                    }
                    Some(token) => {
                        self.consume(token);
                    }
                },
            }
        }
    }
}
