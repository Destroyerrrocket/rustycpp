//! The Preprocessor is responsible for evaluating the step 4 of the C++
//! translation.

use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use crate::{
    grammars::defineast::DefineAst,
    utils::{
        filemap::FileMap,
        parameters::Parameters,
        structs::{CompileError, CompileMsg, CompileWarning, FilePreTokPos},
    },
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
    /// Parameters of the compilation
    parameters: Arc<Parameters>,
    /// The multilexer is the object that will generate the pretokens tokens
    multilexer: MultiLexer,
    /// The generated preprocessing tokens to be returned. This is a stash, as
    /// some tokens may generate more than one token.
    generated: VecDeque<FilePreTokPos<PreToken>>,
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
}

impl Preprocessor {
    /// Creates a new preprocessor from the given parameters, filemap and path
    pub fn new(data: (Arc<Parameters>, Arc<Mutex<FileMap>>, &str)) -> Self {
        Self {
            parameters: data.0,
            multilexer: MultiLexer::new((data.1, data.2)),
            generated: VecDeque::new(),
            errors: VecDeque::new(),
            scope: vec![],
            definitions: HashMap::new(),
            disabledMacros: HashMultiSet::new(),
            atStartLine: true,
        }
        .initCustomMacros()
    }

    /// Creates a new preprocessor from the given parameters, filemap and path
    fn undefineMacro(&mut self, preToken: FilePreTokPos<PreToken>) {
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
    fn consumeMacroDef(&mut self, _PreToken: FilePreTokPos<PreToken>) -> Option<String> {
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
    fn reachNl(&mut self) {
        loop {
            let inIdent = self.multilexer.next();
            match inIdent {
                None => {
                    return;
                }
                Some(ident) => {
                    if ident.tokPos.tok == PreToken::Newline {
                        return;
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

    /// Encountered a preprocessor directive. Evaluate it accordingly, alering
    /// the state of the preprocessor.
    fn preprocessorDirective(&mut self, _PreToken: FilePreTokPos<PreToken>) {
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
    fn consume(&mut self, newToken: FilePreTokPos<PreToken>) {
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
                                                .collect::<VecDeque<FilePreTokPos<PreToken>>>(),
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
    type Item = Result<FilePreTokPos<PreToken>, CompileMsg>;
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
