use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

use crate::{
    filemap::FileMap,
    grammars::defineast::*,
    utils::{pretoken::*, structs::*},
};

use self::multilexer::MultiLexer;

#[derive(Debug, PartialEq)]
enum ScopeStatus {
    SuccessBlock,
    FailureBlock,
    AlreadySucceededBlock,
}

mod defineparse;
mod macroexpand;
mod multilexer;

#[derive(Debug)]
pub struct Preprocessor {
    multilexer: multilexer::MultiLexer,
    generated: VecDeque<FilePreTokPos<PreToken>>,
    errors: VecDeque<CompileMsg>,
    scope: Vec<ScopeStatus>,
    definitions: HashMap<String, DefineAst>,
    disabledMacros: HashSet<String>,
    atStartLine: bool,
}

impl Preprocessor {
    pub fn new(data: (Arc<Mutex<FileMap>>, &str)) -> Preprocessor {
        Preprocessor {
            multilexer: MultiLexer::new(data),
            generated: VecDeque::new(),
            errors: VecDeque::new(),
            scope: vec![],
            definitions: HashMap::new(),
            disabledMacros: HashSet::new(),
            atStartLine: true,
        }
    }

    fn undefineMacro(&mut self, preToken: FilePreTokPos<PreToken>) -> () {
        let vecPrepro = Iterator::take_while(&mut self.multilexer, |pre| {
            pre.tokPos.tok != PreToken::Newline
        })
        .collect::<Vec<FilePreTokPos<PreToken>>>();
        match vecPrepro.into_iter().find(|e| !e.tokPos.tok.isWhitespace()) {
            None => {
                self.errors.push_back(CompileError::from_preTo(
                    "Expected an identifier to undefine",
                    &preToken,
                ));
            }
            Some(e) => match e.tokPos.tok {
                PreToken::Ident(id) => {
                    if let None = self.definitions.remove(&id) {
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
        println!("Macros:");
        for (_, defi) in self.definitions.iter() {
            println!("{:?}", defi);
        }
        return;
    }

    fn includeFile(&mut self, file: Option<String>) -> () {
        todo!("Implement including");
    }

    fn consumeMacroInclude(&mut self, PreToken: FilePreTokPos<PreToken>) -> Option<String> {
        todo!("Implement header extraction");
    }

    fn consumeMacroDef(&mut self, PreToken: FilePreTokPos<PreToken>) -> Option<String> {
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

    fn reachNl(&mut self) -> () {
        loop {
            let inIdent = self.multilexer.next();
            match inIdent {
                None => {
                    return;
                }
                Some(ident) => match ident.tokPos.tok {
                    PreToken::Newline => {
                        return;
                    }
                    _ => {}
                },
            }
        }
    }
    fn consumeMacroExpr(&mut self, PreToken: FilePreTokPos<PreToken>) -> () {
        todo!();
    }

    fn evalIfScope(&self, tree: ()) -> bool {
        todo!();
    }

    fn evalIfDef(&self, def: Option<String>) -> bool {
        if let Some(macroName) = def {
            return self.definitions.contains_key(&macroName);
        }
        return false;
    }

    fn preprocessorDirective(&mut self, PreToken: FilePreTokPos<PreToken>) -> () {
        let operation;
        let enabledBlock = match self.scope.last() {
            Some(ScopeStatus::SuccessBlock) | None => true,
            _ => false,
        };
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
                    let t = self.consumeMacroInclude(operation);
                    self.includeFile(t);
                }
                "define" => {
                    self.defineMacro(operation);
                }
                "undef" => {
                    self.undefineMacro(operation);
                }
                "if" => {
                    let t = self.consumeMacroExpr(operation);
                    let t2 = if self.evalIfScope(t) == true {
                        ScopeStatus::SuccessBlock
                    } else {
                        ScopeStatus::FailureBlock
                    };
                    self.scope.push(t2);
                }
                "ifdef" => {
                    let t = { self.consumeMacroDef(operation) };
                    let t2 = if self.evalIfDef(t) == true {
                        ScopeStatus::SuccessBlock
                    } else {
                        ScopeStatus::FailureBlock
                    };
                    self.scope.push(t2);
                }
                "ifndef" => {
                    let t = self.consumeMacroDef(operation);
                    let t2 = if self.evalIfDef(t) == false {
                        ScopeStatus::SuccessBlock
                    } else {
                        ScopeStatus::FailureBlock
                    };
                    self.scope.push(t2);
                }
                "elif" | "else" => {
                    if let Some(scope) = self.scope.last_mut() {
                        *scope = ScopeStatus::AlreadySucceededBlock;
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
                _ => {
                    self.errors.push_back(CompileError::from_preTo(
                        "I do not know this preprocessing expression yet! I'm learning though :)",
                        &operation,
                    ));
                    self.reachNl();
                }
            }
        } else if &ScopeStatus::FailureBlock == self.scope.last().unwrap() {
            match operation.tokPos.tok.to_str() {
                "if" | "ifdef" | "ifndef" => {
                    self.scope.push(ScopeStatus::AlreadySucceededBlock);
                }
                "elif" => {
                    let macroExpr = self.consumeMacroExpr(operation);
                    if self.evalIfScope(macroExpr) == true {
                        let scope = self.scope.last_mut().unwrap();
                        *scope = ScopeStatus::SuccessBlock
                    }
                }
                "else" => {
                    let scope = self.scope.last_mut().unwrap();
                    *scope = ScopeStatus::SuccessBlock;
                    self.reachNl(); // TODO: Check empty
                }
                "endif" => {
                    self.reachNl(); // TODO: Check empty
                    self.scope.pop();
                }
                _ => {
                    self.reachNl();
                }
            }
        } else if &ScopeStatus::AlreadySucceededBlock == self.scope.last().unwrap() {
            match operation.tokPos.tok.to_str() {
                "if" | "ifdef" | "ifndef" => {
                    self.reachNl();
                    self.scope.push(ScopeStatus::AlreadySucceededBlock);
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

    fn consume(&mut self, newToken: FilePreTokPos<PreToken>) -> () {
        loop {
            match self.scope.last() {
                Some(ScopeStatus::SuccessBlock) | None => {
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
                                let toks = Self::macroExpand(
                                    &self.definitions,
                                    &self.disabledMacros,
                                    &mut self.multilexer,
                                    newToken,
                                );
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
impl<'a> Iterator for Preprocessor {
    type Item = Result<FilePreTokPos<PreToken>, CompileMsg>;
    fn next(&mut self) -> Option<Self::Item> {
        let this = self as *mut Self;
        unsafe {
            loop {
                if let Some(err) = (*this).errors.pop_front() {
                    return Some(Err(err));
                }
                match (*this).generated.pop_front() {
                    Some(tok) => {
                        return Some(Ok(tok));
                    }
                    None => match self.multilexer.next() {
                        None => {
                            return None;
                        }
                        Some(token) => {
                            (*this).consume(token);
                        }
                    },
                }
            }
        }
    }
}
