//! The Preprocessor is responsible for evaluating the step 4 of the C++
//! translation.

use crate::{
    fileTokPosMatchArm,
    utils::{compilerstate::CompilerState, structs::TokPos},
};
use std::collections::{HashMap, VecDeque};

use crate::{
    fileTokPosMatches,
    grammars::defineast::DefineAst,
    utils::structs::{CompileError, CompileMsg, CompileWarning, FileTokPos},
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
    tu: String,
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
}

impl Preprocessor {
    /// Creates a new preprocessor from the given parameters, filemap and path
    pub fn new(data: (CompilerState, &str)) -> Self {
        Self {
            tu: data.1.to_string(),
            compilerState: data.0.clone(),
            multilexer: MultiLexer::new((data.0.compileFiles, data.1)),
            generated: VecDeque::new(),
            errors: VecDeque::new(),
            scope: vec![],
            definitions: HashMap::new(),
            disabledMacros: HashMultiSet::new(),
            atStartLine: true,
            alreadyEmittedEnd: false,
        }
        .initCustomMacros()
    }

    /// Creates a new preprocessor from the given parameters, filemap and path
    fn undefineMacro(&mut self, preToken: FileTokPos<PreToken>) {
        let vecPrepro = Iterator::take_while(&mut self.multilexer, |pre| {
            pre.tokPos.tok != PreToken::Newline
        });

        match vecPrepro.into_iter().find(|e| !e.tokPos.tok.isWhitespace()) {
            None => {
                self.errors.push_back(CompileError::from_preTo(
                    "Expected an identifier to undefine",
                    &preToken,
                ));
            }
            Some(e) => match e.tokPos.tok {
                PreToken::Ident(id) => {
                    if self.definitions.remove(&id).is_none() {
                        self.errors.push_back(CompileError::from_preTo(
                            format!("Macro {} is not defined when reached", id),
                            &preToken,
                        ));
                    }
                }
                _ => {
                    self.errors.push_back(CompileError::from_preTo(
                        format!("Expected an identifier, found: {}", e.tokPos.tok.to_str()),
                        &preToken,
                    ));
                }
            },
        }
        log::trace!("Macros:");
        for defi in self.definitions.values() {
            log::trace!("{:?}", defi);
        }
        return;
    }

    /// Consume a #ifdef or #ifndef and return the macro name if it exists
    fn consumeMacroDef(&mut self, _PreToken: FileTokPos<PreToken>) -> Option<String> {
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
        return Some(identStr);
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
        return false;
    }

    /// If applicable, generate a module token
    fn moduleDirective(&mut self, module: FileTokPos<PreToken>) -> VecDeque<FileTokPos<PreToken>> {
        let mut toks = self.reachNl();

        let tokVal = toks
            .iter()
            .find(|x| !fileTokPosMatches!(x, PreToken::Whitespace(_)));

        if let Some(moduleTok) = tokVal {
            if *module.file.path() == self.tu
                && fileTokPosMatches!(
                    module,
                    PreToken::OperatorPunctuator(":" | ";") | PreToken::Ident(_)
                )
            {
                toks.push_front(FileTokPos::new_meta_c(PreToken::Module, moduleTok));
                return toks;
            }
        }

        self.atStartLine = false;
        toks.push_front(module);
        self.multilexer.pushTokensDec(toks);

        return VecDeque::new();
    }

    /// If applicable, generate a import token
    fn importDirective(&mut self, import: FileTokPos<PreToken>) -> VecDeque<FileTokPos<PreToken>> {
        let mut toks = self.reachNl();
        let tokVal = toks
            .iter()
            .find(|x| !fileTokPosMatches!(x, PreToken::Whitespace(_)));

        if let Some(moduleTok) = tokVal {
            if *import.file.path() == self.tu {
                if fileTokPosMatches!(
                    moduleTok,
                    PreToken::OperatorPunctuator(":" | ";") | PreToken::Ident(_)
                ) {
                    toks.push_front(FileTokPos::new_meta_c(PreToken::Import, &import));
                    return toks;
                } else if let PreToken::HeaderName(ref header) = moduleTok.tokPos.tok {
                    let fileHeader = self
                        .compilerState
                        .compileFiles
                        .lock()
                        .unwrap()
                        .getFile(&header[1..header.len() - 1])
                        .path()
                        .clone();
                    let otherDefinitions = self
                        .compilerState
                        .compileUnits
                        .lock()
                        .unwrap()
                        .get(&fileHeader)
                        .unwrap()
                        .macroDefintionsAtTheEndOfTheFile
                        .clone();
                    self.definitions.extend(otherDefinitions);

                    // Remove the header token
                    let mut startingWhitespace = VecDeque::new();
                    while let Some(x) = toks.pop_front() {
                        if x.tokPos.tok != PreToken::Newline {
                            break;
                        }
                        startingWhitespace.push_back(x);
                    }
                    // Insert whitespace back, and a new special token for the later stages
                    toks.extend(startingWhitespace);
                    toks.push_front(FileTokPos::new_meta_c(
                        PreToken::ImportableHeaderName(fileHeader),
                        &import,
                    ));
                    toks.push_front(FileTokPos::new_meta_c(PreToken::Import, &import));
                    return toks;
                }
            }
        }

        self.atStartLine = false;
        toks.push_front(import);
        self.multilexer.pushTokensDec(toks);

        return VecDeque::new();
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
        return VecDeque::new();
    }

    /// Encountered a preprocessor directive. Evaluate it accordingly, alering
    /// the state of the preprocessor.
    fn preprocessorDirective(&mut self, _PreToken: FileTokPos<PreToken>) {
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
                            if let Err(err) = self.includeFile(&operation, path) {
                                self.errors.push_back(err);
                            }
                        }
                        Err(err) => {
                            self.errors.push_back(err);
                        }
                    }
                }
                "define" => {
                    self.defineMacro(operation);
                }
                "undef" => {
                    self.undefineMacro(operation);
                }
                "if" => {
                    let sequenceToEval = self.consumeMacroExpr();
                    match sequenceToEval {
                        Err(err) => {
                            self.errors.push_back(err);
                        }
                        Ok(sequenceToEval) => match Self::evalIfScope(sequenceToEval, &operation) {
                            Ok(true) => {
                                self.scope.push(ScopeStatus::Success);
                            }
                            Ok(false) => {
                                self.scope.push(ScopeStatus::Failure);
                            }
                            Err(err) => {
                                self.errors.extend(err);
                            }
                        },
                    }
                }
                "ifdef" => {
                    let t = self.consumeMacroDef(operation);
                    self.scope.push(if self.evalIfDef(t) {
                        ScopeStatus::Success
                    } else {
                        ScopeStatus::Failure
                    });
                }
                "ifndef" => {
                    let t = self.consumeMacroDef(operation);
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
                        self.errors.push_back(CompileError::from_preTo(
                            "Missmatched preprocessor conditional block",
                            &operation,
                        ));
                    }
                }
                "pragma" => {
                    self.errors.push_back(CompileError::from_preTo("LMAO, you really expected me to implement this now XD. No worries, we'll get there :D", &operation));
                    self.reachNl();
                }
                "endif" => {
                    if self.scope.is_empty() {
                        self.errors.push_back(CompileError::from_preTo(
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
                        .push_back(CompileError::from_preTo(msg, &operation));
                }
                "warning" => {
                    let mut msg = String::new();
                    for t in Iterator::take_while(&mut self.multilexer, |pre| {
                        pre.tokPos.tok != PreToken::Newline
                    }) {
                        msg.push_str(t.tokPos.tok.to_str());
                    }
                    self.errors
                        .push_back(CompileWarning::from_preTo(msg, &operation));
                }
                _ => {
                    self.errors.push_back(CompileError::from_preTo(
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
                            match Self::evalIfScope(sequenceToEval, &operation) {
                                Ok(true) => {
                                    let scope = self.scope.last_mut().unwrap();
                                    *scope = ScopeStatus::Success;
                                }
                                Ok(false) => {}
                                Err(err) => {
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
                                self.preprocessorDirective(newToken);
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
                                continue;
                            }
                        }
                    } else {
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
                }
                _ => {
                    if self.atStartLine {
                        match newToken.tokPos.tok {
                            PreToken::Whitespace(_) | PreToken::Newline => {
                                break;
                            }
                            PreToken::PreprocessingOperator(PreprocessingOperator::Hash) => {
                                self.preprocessorDirective(newToken);
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
                    } else {
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
                                .lock()
                                .unwrap()
                                .get_mut(&self.tu)
                                .unwrap()
                                .macroDefintionsAtTheEndOfTheFile
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
