use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use lazy_regex::regex_is_match;

use crate::compiler::TranslationUnit;
use crate::preprocessor::prelexer::PreLexer;
use crate::preprocessor::pretoken::PreToken;
use crate::utils::filemap::FileMap;
use crate::utils::structs::{CompileError, CompileFile, CompileMsg, TokPos};

use super::structs::ModuleOperator;

fn parseModuleOp(
    lexer: &mut PreLexer,
    translationUnit: &Arc<CompileFile>,
) -> Result<Option<ModuleOperator>, CompileMsg> {
    let mut toks = lexer
        .take_while(|x| x.tok != PreToken::Newline)
        .collect::<VecDeque<_>>();
    while let Some(TokPos {
        tok: PreToken::Whitespace(_),
        ..
    }) = toks.front()
    {
        toks.pop_front();
    }

    if let Some(TokPos { tok, .. }) = toks.front() {
        match tok {
            PreToken::OperatorPunctuator(":" | ";") | PreToken::Ident(_) => {}
            _ => {
                return Ok(None);
            }
        }
    }

    let mut at = usize::MAX;
    let mut atEnd = usize::MIN;
    let mut name = String::new();

    for tok in toks {
        match tok.tok {
            PreToken::Keyword("private") => {
                name.push_str("private");
            }
            PreToken::Ident(str) => {
                name.push_str(&str);
            }
            PreToken::OperatorPunctuator(":") => {
                name.push(':');
            }
            PreToken::OperatorPunctuator(".") => {
                name.push('.');
            }
            PreToken::Whitespace(_) => {}
            _ => {
                break;
            }
        }
        at = at.min(tok.start);
        atEnd = atEnd.min(tok.end);
    }
    if !regex_is_match!(
        r"(:?[\w\d_]+\.)*[\w\d_]+(:?:(:?[\w\d_]+\.)*[\w\d_]+)?",
        &name
    ) && !name.is_empty()
        && name != ":private"
    {
        return Err(CompileError::from_at(
            format!("The module name \"{}\" is invalid!", name),
            translationUnit.clone(),
            at,
            Some(atEnd),
        ));
    }
    return Ok(Some(ModuleOperator::Module(name)));
}

fn parseImportOp(
    lexer: &mut PreLexer,
    translationUnit: &Arc<CompileFile>,
) -> Result<Option<ModuleOperator>, CompileMsg> {
    lexer.expectHeader();
    let mut toks = lexer
        .take_while(|x| x.tok != PreToken::Newline)
        .collect::<VecDeque<_>>();
    while let Some(TokPos {
        tok: PreToken::Whitespace(_),
        ..
    }) = toks.front()
    {
        toks.pop_front();
    }

    if let Some(TokPos { tok, .. }) = toks.front() {
        match tok {
            PreToken::HeaderName(_)
            | PreToken::OperatorPunctuator(":" | ";")
            | PreToken::Ident(_) => {}
            _ => {
                return Ok(None);
            }
        }
    }

    let mut at = usize::MAX;
    let mut atEnd = usize::MIN;

    let mut name = String::new();
    for tok in toks {
        match tok.tok {
            PreToken::HeaderName(header) => {
                return Ok(Some(ModuleOperator::ImportHeader(header)));
            }
            PreToken::Ident(str) => {
                name.push_str(&str);
            }
            PreToken::OperatorPunctuator(":") => {
                name.push(':');
            }
            PreToken::OperatorPunctuator(".") => {
                name.push('.');
            }
            PreToken::Whitespace(_) => {}
            _ => {
                break;
            }
        }
        at = at.min(tok.start);
        atEnd = atEnd.min(tok.end);
    }
    if !regex_is_match!(
        r"(:?(:?[\w\d_]+\.)*[\w\d_]+|:(:?[\w\d_]+\.)*[\w\d_]+)",
        &name
    ) {
        return Err(CompileError::from_at(
            format!("The import name \"{}\" is invalid!", name),
            translationUnit.clone(),
            at,
            Some(atEnd),
        ));
    }
    return Ok(Some(ModuleOperator::Import(name)));
}

fn parseExportOp(
    lexer: &mut PreLexer,
    translationUnit: &Arc<CompileFile>,
) -> Result<Option<ModuleOperator>, CompileMsg> {
    let tok = loop {
        let tok = lexer.next();
        if let Some(TokPos {
            tok: PreToken::Whitespace(_),
            ..
        }) = tok
        {
            continue;
        }
        break tok;
    };
    if let Some(TokPos { tok, .. }) = tok {
        return match tok {
            PreToken::Ident(id) if id == "import" => parseImportOp(lexer, translationUnit),
            PreToken::Ident(id) if id == "module" => {
                parseModuleOp(lexer, translationUnit).map(|x| {
                    x.map(|op| {
                        if let ModuleOperator::Module(module) = op {
                            ModuleOperator::ExportModule(module)
                        } else {
                            op
                        }
                    })
                })
            }
            _ => Ok(None),
        };
    }
    Ok(None)
}

/// Extract the module, export, import operations only rellevant for dependency scanning of a single file
fn parseModuleMacroOp(
    translationUnit: Arc<CompileFile>,
) -> Result<Vec<ModuleOperator>, Vec<CompileMsg>> {
    let mut err = vec![];
    let mut res = vec![];
    {
        let mut lexer = PreLexer::new(translationUnit.content().clone());
        let mut atStartLine = true;
        while let Some(TokPos { tok, .. }) = lexer.next() {
            if tok.isWhitespace() {
                continue;
            }

            if tok == PreToken::Newline {
                atStartLine = true;
                continue;
            }
            if atStartLine {
                atStartLine = false;
                match tok {
                    PreToken::Ident(str) if str == "module" => {
                        match parseModuleOp(&mut lexer, &translationUnit) {
                            Ok(None) => continue,
                            Ok(Some(v)) => {
                                atStartLine = true;
                                res.push(v);
                            }
                            Err(newErr) => err.push(newErr),
                        };
                    }
                    PreToken::Keyword("export") => {
                        match parseExportOp(&mut lexer, &translationUnit) {
                            Ok(None) => continue,
                            Ok(Some(v)) => {
                                atStartLine = true;
                                res.push(v);
                            }
                            Err(newErr) => err.push(newErr),
                        };
                    }
                    PreToken::Ident(str) if str == "import" => {
                        match parseImportOp(&mut lexer, &translationUnit) {
                            Ok(None) => continue,
                            Ok(Some(v)) => {
                                atStartLine = true;
                                res.push(v);
                            }
                            Err(newErr) => err.push(newErr),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    if err.is_empty() {
        Ok(res)
    } else {
        Err(err)
    }
}

/// Extract the module, export, import operations only rellevant for dependency scanning
pub fn parseModuleMacroOps(
    translationUnits: &Vec<TranslationUnit>,
    fileMap: &mut Arc<Mutex<FileMap>>,
) -> Result<Vec<(TranslationUnit, Vec<ModuleOperator>)>, Vec<CompileMsg>> {
    let mut err = vec![];
    let mut res = vec![];
    for translationUnit in translationUnits {
        let file = fileMap.lock().unwrap().getAddFile(translationUnit);
        match parseModuleMacroOp(file) {
            Ok(node) => res.push((translationUnit.clone(), node)),
            Err(mut err2) => err.append(&mut err2),
        }
    }

    if err.is_empty() {
        Ok(res)
    } else {
        Err(err)
    }
}
