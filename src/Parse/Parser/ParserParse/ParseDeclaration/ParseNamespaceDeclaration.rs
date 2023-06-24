use crate::{
    fileTokPosMatchArm,
    Ast::Common::{AstAttribute, AstDecl},
    Lex::Token::Token,
    Parse::BufferedLexer::StateBufferedLexer,
    Utils::Structs::{CompileError, CompileMsgImpl, FileTokPos, SourceRange, TokPos},
};

use super::super::super::Parser;

impl Parser {
    /**
     * Either a namespace-definition, or a namespace-alias-definition:
     *
     * namespace-definition:
     *   named-namespace-definition
     *   unnamed-namespace-definition
     *   nested-namespace-definition
     * named-namespace-definition:
     *   inline [opt] namespace attribute-specifier-seq [opt] identifier { namespace-body }
     * unnamed-namespace-definition:
     *    inline [opt] namespace attribute-specifier-seq [opt] { namespace-body }
     * nested-namespace-definition:
     *    namespace enclosing-namespace-specifier :: inline [opt] identifier { namespace-body }
     * enclosing-namespace-specifier:
     *    identifier (:: inline [opt] identifier)*
     *
     * or a block scope:
     * namespace-alias-definition:
     *   namespace identifier = qualified-namespace-specifier ;
     */
    pub fn parseNamespaceDeclaration(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        attr: &[AstAttribute],
    ) -> Vec<AstDecl> {
        self.actWrongAttributeLocation(attr);

        let isInline = self.lexer().consumeTokenIfEq(lexpos, Token::Inline);

        if !self.lexer().consumeTokenIfEq(lexpos, Token::Namespace) {
            // We already expected a namespace keyword. Reaching this is a bug.
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
            self.errors.push(CompileError::fromPreTo(
                "Expected 'namespace' keyword. This is a bug. Report is please.",
                posErr,
            ));
        }

        let attr: Vec<AstAttribute> = self.parseAttributes(lexpos);
        let name = self.lexer().getConsumeToken(lexpos);
        match name {
            Some(fileTokPosMatchArm!(Token::Identifier(nameStr))) => {
                self.errorAttributes(lexpos);
                let Some(tok) = self.lexer().get(lexpos) else {
                    let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
                    self.errors.push(CompileError::fromPreTo(
                        "Expected '{' (to introduce a namespace) or '=' (to make a namespace alias) after the namespace name.",
                        posErr,
                    ));
                    return vec![];
                };
                match tok.tokPos.tok {
                    Token::LBrace => {
                        // named-namespace-definition
                        let astNamespace = self.actOnStartNamedNamespaceDefinition(
                            isInline,
                            attr.as_slice(),
                            *nameStr,
                            SourceRange::newSingleTok(name.unwrap()),
                        );
                        let contents = self.parseNamespaceBody(lexpos);
                        if let Some(astNamespace) = astNamespace.first() {
                            self.actOnEndNamedNamespaceDefinition(astNamespace, &contents);
                        }
                        astNamespace
                    }
                    Token::Equal => todo!(),
                    _ => {
                        self.errors.push(CompileError::fromPreTo(
                            "Expected '{' (to introduce a namespace) or '=' (to make a namespace alias) after the namespace name. Instead, we found this.",
                            tok,
                        ));
                        vec![]
                    }
                }
            }
            _ => todo!(),
        }
    }

    /**
     * We have not parsed the '{' yet! (but we know it's comming)
     * namespace-body:
     *   declaration-seq [opt]
     */
    pub fn parseNamespaceBody(&mut self, lexpos: &mut StateBufferedLexer) -> Vec<AstDecl> {
        if !self.lexer().consumeTokenIfEq(lexpos, Token::LBrace) {
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
            self.errors.push(CompileError::fromPreTo(
                "Expected '{' to introduce a namespace body. This is a bug. Please report it.",
                posErr,
            ));
            return vec![];
        }

        let mut decls = vec![];
        loop {
            if let Some(fileTokPosMatchArm!(tok)) = self.lexer().get(lexpos) {
                if matches!(tok, Token::RBrace) {
                    self.lexer().consumeToken(lexpos);
                    break;
                }
                let attrs = self.parseAttributes(lexpos);
                let newDecls = self.parseDeclaration(lexpos, attrs.as_slice());
                decls.extend(newDecls);
            } else {
                let posErr = self.lexer().getWithOffsetSaturating(lexpos, -1);
                self.errors.push(CompileError::fromPreTo(
                    "Expected '}' to end the namespace body. Maybe insert one here?",
                    posErr,
                ));
                break;
            }
        }
        decls
    }
}
