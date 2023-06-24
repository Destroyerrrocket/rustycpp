use crate::{
    Ast::Common::{AstAttribute, AstDecl},
    Lex::Token::Token,
    Parse::{BufferedLexer::StateBufferedLexer, Parser::ParserParse::ParseMatched},
    Utils::Structs::{CompileError, CompileMsgImpl, SourceRange},
};

use super::super::super::Parser;

impl Parser {
    /**
     * Either a namespace-definition, or a namespace-alias-definition:
     * using-directive:
     * attribute-specifier-seq [opt] using namespace nested-name-specifier [opt] namespace-name ;
     */
    pub fn parseUsingNamespaceDeclaration(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        attr: &[AstAttribute],
    ) -> Vec<AstDecl> {
        let startlexpos = &mut lexpos.clone();
        let Some(usingTok) = self.lexer().getConsumeTokenIfEq(lexpos, Token::Using) else {
            // We already expected a using keyword. Reaching this is a bug.
            let posErr = self.lexer().getWithOffsetSaturating(startlexpos, 0);
            self.errors.push(CompileError::fromPreTo(
                "Expected 'using namespace'. This is a bug. Report is please.",
                posErr,
            ));
            return vec![];
        };
        if !self.lexer().consumeTokenIfEq(lexpos, Token::Namespace) {
            // We already expected a using keyword. Reaching this is a bug.
            let posErr = self.lexer().getWithOffsetSaturating(startlexpos, 0);
            self.errors.push(CompileError::fromPreTo(
                "Expected 'using namespace'. This is a bug. Report is please.",
                posErr,
            ));
            return vec![];
        }

        let (nestedNamespace, nestedNamespaceMatched) = self.optParseNestedNameSpecifier(lexpos);

        let Some(nameTok) = self.lexer().getConsumeTokenIfIdentifier(lexpos) else {
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, 0);
            self.errors.push(CompileError::fromPreTo(
                "using namespace expects a namespace name after the namespace keyword",
                posErr,
            ));
            return vec![];
        };
        let Token::Identifier(name) = nameTok.tokPos.tok else {
            unreachable!();
        };

        let Some(semiTok) = self.lexer().getConsumeTokenIfEq(lexpos, Token::Semicolon) else {
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, 0);
            self.errors.push(CompileError::fromPreTo(
                "using namespace expects a semicolon at the end",
                posErr,
            ));
            return vec![];
        };

        let scope = match nestedNamespaceMatched {
            ParseMatched::Matched => {
                let Some(scope) = nestedNamespace.last().and_then(|nestedName| {
                    nestedName.scope.borrow().clone()
                }) else {
                    self.errors.push(CompileError::fromPreTo(
                        "We were unable to resolve this name. Something's wrong with the nested name specifier",
                        nameTok,
                    ));
                    return vec![];
                };
                Some(scope)
            }
            ParseMatched::NotMatched => None,
        };
        self.actOnUsingNamespaceDefinition(
            name,
            SourceRange::newDoubleTok(usingTok, semiTok),
            attr,
            nestedNamespace,
            scope.as_ref(),
        )
    }
}
