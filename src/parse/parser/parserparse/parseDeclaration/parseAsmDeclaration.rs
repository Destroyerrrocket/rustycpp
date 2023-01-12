use crate::{
    ast::{Attribute::AstAttribute, Decl::AstDecl},
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, CompileWarning, SourceRange},
};

use super::super::super::Parser;

impl Parser {
    /**
     * asm-declaration:
     *    attribute-specifier-seq [opt] asm ( string-literal ) ;
     */
    pub fn parseAsmDeclaration(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        attr: &Vec<&'static AstAttribute>,
    ) -> Vec<&'static AstDecl> {
        let startlexpos = *lexpos;
        let startedAsm = self.lexer().consumeTokenIfEq(lexpos, Token::Asm);
        assert!(startedAsm);
        if self.lexer().getIfEq(lexpos, Token::LParen).is_none() {
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
            self.errors.push(CompileError::fromPreTo(
                "Expected '(' after 'asm' keyword.",
                posErr,
            ));
            return vec![];
        }
        let parenPos = *lexpos;
        let Some(mut scoped) = self.parseAlmostBalancedPattern(lexpos) else {
            let posErr = self.lexer().getWithOffsetSaturating(&parenPos, 0);
            self.errors.push(CompileError::fromPreTo(
                "Expected a closing parentheses for this '(' while evaluating the 'asm' declaration.",
                posErr
            ));
            return vec![];
        };

        let Some(content) = self.lexer().getConsumeToken(&mut scoped) else {
            let posErr = self.lexer().getWithOffsetSaturating(&parenPos, 0);
            self.errors.push(CompileError::fromPreTo(
                "Expected a string literal inside the 'asm' declaration.", posErr
            ));
            return vec![];
        };

        let Token::StringLiteral(_, content) = content.tokPos.tok else {
            self.errors.push(CompileError::fromPreTo(
                "Expected a string literal for the 'asm' declaration.",
                content,
            ));
            return vec![];
        };

        if let Some(unused) = self.lexer().getConsumeToken(&mut scoped) {
            self.errors.push(CompileWarning::fromPreTo(
                "Unused content after the string literal for the 'asm' declaration.",
                unused,
            ));
        }

        if self.lexer().getIfEq(lexpos, Token::Semicolon).is_none() {
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
            self.errors.push(CompileError::fromPreTo(
                "Expected a ';' after the 'asm' declaration.",
                posErr,
            ));
        } else {
            self.lexer().next(lexpos);
        }
        let posAsm = SourceRange::newDoubleTok(
            self.lexer().getWithOffsetSaturating(&startlexpos, 0),
            self.lexer().getWithOffsetSaturating(lexpos, -1),
        );
        return self.actOnAsmDecl(attr, posAsm, content);
    }
}