//! Parse the files to search for dependency instructions.
use lazy_regex::regex_is_match;

use crate::fileTokPosMatchArm;
use crate::utils::structs::{CompileError, CompileMsg, CompileMsgImpl, TokPos};
use crate::{compiler::TranslationUnit, lex::token::Token, utils::structs::FileTokPos};

use super::structs::ModuleOperator;

/// When encountering a module operator, validates it can be used and parses it.
fn parseModuleOp(
    translationUnit: TranslationUnit,
    tokens: &[FileTokPos<Token>],
    pos: usize,
) -> Result<Option<ModuleOperator>, CompileMsg> {
    let mut at = usize::MAX;
    let mut atEnd = usize::MIN;
    let mut name = String::new();
    for tok in tokens.iter().skip(pos) {
        match tok.tokPos.tok {
            Token::Private => {
                name.push_str("private");
            }
            Token::Identifier(string) => {
                name.push_str(string.as_ref());
            }
            Token::Colon => {
                name.push(':');
            }
            Token::Dot => {
                name.push('.');
            }
            _ => {
                break;
            }
        }
        at = at.min(tok.tokPos.start);
        atEnd = atEnd.max(tok.tokPos.end);
    }

    if !regex_is_match!(
        r"(:?[\w\d_]+\.)*[\w\d_]+(:?:(:?[\w\d_]+\.)*[\w\d_]+)?",
        &name
    ) && !name.is_empty()
        && name != ":private"
    {
        return Err(CompileError::fromAt(
            format!("The module name \"{name}\" is invalid!"),
            translationUnit,
            at,
            Some(atEnd),
        ));
    }
    Ok(Some(ModuleOperator::Module(name)))
}

/// When encountering an import operator, validates it can be used and parses it.
fn parseImportOp(
    translationUnit: TranslationUnit,
    tokens: &[FileTokPos<Token>],
    pos: usize,
) -> Result<Option<ModuleOperator>, CompileMsg> {
    let mut at = usize::MAX;
    let mut atEnd = usize::MIN;

    let mut name = String::new();
    for tok in tokens.iter().skip(pos) {
        match tok.tokPos.tok {
            Token::ImportableHeaderName(header) => {
                return Ok(Some(ModuleOperator::ImportHeader(header)));
            }
            Token::Identifier(string) => {
                name.push_str(string.as_ref());
            }
            Token::Colon => {
                name.push(':');
            }
            Token::Dot => {
                name.push('.');
            }
            _ => {
                break;
            }
        }
        at = at.min(tok.tokPos.start);
        atEnd = atEnd.max(tok.tokPos.end);
    }
    if !regex_is_match!(
        r"(:?(:?[\w\d_]+\.)*[\w\d_]+|:(:?[\w\d_]+\.)*[\w\d_]+)",
        &name
    ) {
        return Err(CompileError::fromAt(
            format!("The import name \"{name}\" is invalid!"),
            translationUnit,
            at,
            Some(atEnd),
        ));
    }
    Ok(Some(ModuleOperator::Import(name)))
}

/// When encountering an export operator, validates it can be used and parses it.
fn parseExportOp(
    translationUnit: TranslationUnit,
    tokens: &[FileTokPos<Token>],
    pos: usize,
) -> Result<Option<ModuleOperator>, CompileMsg> {
    if let Some(fileTokPosMatchArm!(tok)) = tokens.get(pos) {
        return match tok {
            Token::Import => parseImportOp(translationUnit, tokens, pos + 1),
            Token::Module => parseModuleOp(translationUnit, tokens, pos + 1).map(|x| {
                x.map(|op| {
                    if let ModuleOperator::Module(module) = op {
                        ModuleOperator::ExportModule(module)
                    } else {
                        op
                    }
                })
            }),
            _ => Ok(None),
        };
    }
    Ok(None)
}

/// Extract the module, export, import operations only rellevant for dependency scanning of a single file
pub fn parseModuleMacroOp(
    translationUnit: TranslationUnit,
    tokens: &[FileTokPos<Token>],
    positions: Vec<usize>,
) -> Result<Vec<ModuleOperator>, Vec<CompileMsg>> {
    let mut err = vec![];
    let mut res = vec![];

    for pos in positions {
        if pos > 0 && tokens[pos - 1].tokPos.tok == Token::Export {
            match parseExportOp(translationUnit, tokens, pos) {
                Ok(None) => continue,
                Ok(Some(v)) => {
                    res.push(v);
                }
                Err(newErr) => err.push(newErr),
            };
        } else {
            match tokens[pos].tokPos.tok {
                Token::Module => {
                    match parseModuleOp(translationUnit, tokens, pos + 1) {
                        Ok(None) => continue,
                        Ok(Some(v)) => {
                            res.push(v);
                        }
                        Err(newErr) => err.push(newErr),
                    };
                }
                Token::Import => {
                    match parseImportOp(translationUnit, tokens, pos + 1) {
                        Ok(None) => continue,
                        Ok(Some(v)) => {
                            res.push(v);
                        }
                        Err(newErr) => err.push(newErr),
                    };
                }
                _ => {
                    unreachable!()
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
