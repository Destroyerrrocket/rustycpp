use crate::{
    ast::{Attribute::AstAttribute, Decl::AstDecl},
    fileTokPosMatchArm,
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, FileTokPos, SourceRange, TokPos},
};

use super::super::super::Parser;

impl Parser {
    /**
     * __rustycpp__ (stuff) custom operator
     */
    pub fn parseCustom__rustycpp__Decl(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        attr: &Vec<&'static AstAttribute>,
    ) -> Vec<&'static AstDecl> {
        let Some(rustyCpp) = self
            .lexer()
            .getConsumeTokenIfEq(lexpos, Token::__rustycpp__) else
        {
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
            self.errors.push(CompileError::fromPreTo(
                "Expected '__rustycpp__' keyword. This is a bug. Report is please.",
                posErr
            ));
            return vec![];
        };

        let Some(lParen) = self.lexer().getConsumeTokenIfEq(lexpos, Token::LParen) else {
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
            self.errors.push(CompileError::fromPreTo(
                "Expected '(' after '__rustycpp__' keyword.", posErr
            ));
            return vec![];
        };

        let result = self.parseContentsOf__rustycpp__Decl(lexpos, rustyCpp, attr);

        while let Some(fileTokPosMatchArm!(tok)) = self.lexer().get(lexpos) {
            if matches!(tok, Token::RParen) {
                break;
            }
            self.lexer().next(lexpos);
        }

        if !self.lexer().consumeTokenIfEq(lexpos, Token::RParen) {
            self.errors.push(CompileError::fromPreTo(
                "Expected ')' to match this '('.",
                lParen,
            ));
        };

        result
    }

    fn parseContentsOf__rustycpp__Decl(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        rustyCpp: &FileTokPos<Token>,
        attr: &Vec<&'static AstAttribute>,
    ) -> Vec<&'static AstDecl> {
        let Some(enumTok) = self.lexer().getConsumeTokenIfEq(lexpos, Token::Enum) else {
            self.errors.push(CompileError::fromPreTo(
                "Expected \"enum\" inside '__rustycpp__' keyword.",
                rustyCpp,
            ));
            return vec![];
        };

        let Some(nameTok) = self.lexer().getConsumeTokenIfIdentifier(lexpos) else {
            self.errors.push(CompileError::fromPreTo(
                "Expected enum name after 'enum'.",
                enumTok,
            ));
            return vec![];
        };

        let location = SourceRange::newDoubleTok(enumTok, nameTok);
        let fileTokPosMatchArm!(Token::Identifier(name)) = nameTok else {unreachable!()};

        return self.actOnRustyCppEnumDefinition(*name, location, attr);
    }
}
