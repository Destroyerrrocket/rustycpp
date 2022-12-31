use crate::utils::structs::FileTokPos;
use crate::utils::structs::TokPos;
use crate::{
    ast::Tu::AstTu,
    fileTokPosMatchArm,
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl},
};

use super::super::Parser;

impl Parser {
    /**
    * Entry point for parsing a translation unit.
    * translation-unit:
       declaration-seq[opt]
       global-module-fragment [opt] module-declaration declaration-seq [opt] private-module-fragment [opt]
    */
    pub fn parseTu(&mut self) -> AstTu {
        let mut totalDeclarations = Vec::new();
        let mut lexpos = self.lexerStart;
        while !self.lexer.reachedEnd(&mut lexpos) {
            let declarations = self.parseTopLevelDecl(&mut lexpos);
            totalDeclarations.extend(declarations);
        }
        return AstTu;
    }

    /**
     * Parse a top-level declaration. We have to detect the pattern for modules.
     *
     * translation-unit:
     *   declaration-seq [opt]
     *   global-module-fragment [opt] module-declaration declaration-seq [opt] private-module-fragment [opt]
     *
     * global-module-fragment:
     *   module-keyword ; declaration-seq [opt]
     *
     * private-module-fragment:
     *   module-keyword : private ; declaration-seq [opt]
     *
     * module-declaration:
     *   export-keyword [opt] module-keyword module-name module-partition [opt] attribute-specifier-seq [opt];
     */
    fn parseTopLevelDecl(&mut self, lexpos: &mut StateBufferedLexer) -> Vec<()> {
        let tok1 = self.lexer.get(lexpos).unwrap();
        match tok1 {
            fileTokPosMatchArm!(Token::Module) => {
                self.parseModuleFragmentIntro(lexpos);
                return Vec::new();
            }
            fileTokPosMatchArm!(Token::Export) => {
                let tok2 = self.lexer.getWithOffset(lexpos, 1);
                match tok2 {
                    Some(fileTokPosMatchArm!(Token::Module)) => {
                        self.parseModuleFragmentIntro(lexpos);
                        return Vec::new();
                    }
                    None => {
                        self.errors.push(CompileError::fromPreTo(
                            "Expected \"module\" or declaration after export keyword.",
                            &tok1,
                        ));
                        self.lexer.moveForward(lexpos, 1); // Skip malformed line. Puts us at the end of file.
                        return Vec::new();
                    }
                    _ => {}
                }
            }
            _ => {
                self.errors.push(CompileError::fromPreTo(
                    "No way to parse this start of the top level declaration",
                    &tok1,
                ));
                self.lexer.moveForward(lexpos, 1); // Skip malformed line. Puts us at the end of file.
            }
        }
        return Vec::new();
    }

    /**
     * Parse the introduction to a module fragment. At this point we know we have a module keyword comming up!
     *
     * global-module-fragment:
     *   module-keyword ;
     *
     * private-module-fragment:
     *   module-keyword : private ;
     *
     * module-declaration:
     *   export-keyword [opt] module-keyword module-name module-partition [opt] attribute-specifier-seq [opt];
     */
    fn parseModuleFragmentIntro(&mut self, lexpos: &mut StateBufferedLexer) {
        let mut startlexpos = *lexpos;
        // This line will bite us in the future... I just don't want to make a
        // new token delimiter that does this properly.
        let mut isExport = false;
        if let fileTokPosMatchArm!(Token::Export) = self.lexer.get(lexpos).unwrap() {
            self.lexer.consumeToken(lexpos);
            isExport = true;
        }
        let moduleKwd = self.lexer.getConsumeToken(lexpos);
        if let Some(fileTokPosMatchArm!(Token::Module)) = moduleKwd {
        } else {
            self.errors.push(CompileError::fromPreTo(
                "Expected \"module\" keyword arround here. This should not have happened, as we already checked for this. Report this bug please!",
                &self.lexer.getWithOffsetSaturating(lexpos, 0),
            ));
            return;
        }
        // Parsed export [opt] module
        // Parsing module-name [opt]
        let moduleName = self.optParseModuleName(lexpos);
        if moduleName.ends_with('.') {
            self.errors.push(CompileError::fromPreTo(
                "Module name cannot end with a dot.",
                &self.lexer.getWithOffsetSaturating(lexpos, -1),
            ));
            return;
        }
        // Parsing module-partition [opt]
        let modulePartition = self.optParseModulePartition(lexpos);
        if modulePartition.as_ref().is_some_and(|s| s.ends_with('.')) {
            self.errors.push(CompileError::fromPreTo(
                "Module partition cannot end with a dot.",
                &self.lexer.getWithOffsetSaturating(lexpos, -1),
            ));
            return;
        }
        if modulePartition.as_ref().is_some_and(String::is_empty) {
            self.errors.push(CompileError::fromPreTo(
                "Module partition cannot be just a colon.",
                &self.lexer.getWithOffsetSaturating(lexpos, -1),
            ));
            return;
        }
        self.optIgnoreAttributes(lexpos);

        // Everything parsed, now we need to check for the semicolon.
        if let Some(fileTokPosMatchArm!(Token::Semicolon)) = self.lexer.get(lexpos) {
            let (st, et) = (
                self.lexer.get(&mut startlexpos).unwrap(),
                self.lexer.get(lexpos).unwrap(),
            );
            if st.file != et.file || {
                let file = self
                    .compilerState
                    .compileFiles
                    .lock()
                    .unwrap()
                    .getOpenedFile(st.file);
                file.getRowColumn(st.tokPos.start).0 != file.getRowColumn(et.tokPos.start).0
            } {
                self.errors.push(CompileError::fromPreTo(
                    "Module declaration must be on a single line.",
                    &self.lexer.getWithOffsetSaturating(lexpos, 0),
                ));
            }
            self.lexer.consumeToken(lexpos);
        } else {
            self.errors.push(CompileError::fromPreTo(
                "Expected ';' at the end of module declaration.",
                &self.lexer.getWithOffsetSaturating(lexpos, -1),
            ));
        }

        self.actOnModuleDecl(isExport, moduleName, modulePartition);
        return;
    }

    /// Parse an optional module-name.
    /// module-name:
    ///   [identifier .? ]*
    /// Notice that there can be an extra dot at the end. This must be checked at call site.
    fn optParseModuleName(&mut self, lexpos: &mut StateBufferedLexer) -> String {
        let mut moduleName = String::new();
        loop {
            macro_rules! pushName {
                ($stringy:expr) => {
                    self.lexer.consumeToken(lexpos);
                    moduleName.push_str($stringy);
                    if let Some(fileTokPosMatchArm!(Token::Dot)) = self.lexer.get(lexpos) {
                        self.lexer.consumeToken(lexpos);
                        moduleName.push('.');
                        continue;
                    }
                };
            }
            match self.lexer.get(lexpos) {
                Some(fileTokPosMatchArm!(Token::Private)) => {
                    pushName!("private");
                }
                Some(fileTokPosMatchArm!(Token::Identifier(name))) => {
                    pushName!(name.as_ref());
                }
                _ => (),
            }
            break;
        }
        return moduleName;
    }

    /// Parse an optional module-partition.
    /// module-partition:
    ///   : module-name
    /// Notice that there can be an extra dot at the end. This must be checked at call site.
    /// Notice that this can return an empty string. This must be checked at call site.
    fn optParseModulePartition(&mut self, lexpos: &mut StateBufferedLexer) -> Option<String> {
        if let Some(fileTokPosMatchArm!(Token::Colon)) = self.lexer.get(lexpos) {
            self.lexer.consumeToken(lexpos);
            return Some(self.optParseModuleName(lexpos));
        }
        return None;
    }
}
