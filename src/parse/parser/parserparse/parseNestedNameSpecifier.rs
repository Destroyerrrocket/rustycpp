use crate::{
    ast::NestedNameSpecifier::{self, AstNestedNameSpecifier},
    fileTokPosMatchArm,
    lex::token::Token,
    parse::{bufferedLexer::StateBufferedLexer, parser::parserparse::ParseMatched},
    sema::scope::ScopeKind,
    utils::structs::{CompileError, CompileMsgImpl, FileTokPos, SourceRange, TokPos},
};

use super::super::Parser;

impl Parser {
    /**
     * Parses optionally a nested-name-specifier:
     * nested-name-specifier:
     *  :: [ROOT]
     *  type-name :: [ROOT]
     *  namespace-name :: [ROOT]
     *  decltype-specifier :: [ROOT] [TODO: UNIMPLEMENTED]
     *  nested-name-specifier identifier :: [CHILD]
     *  nested-name-specifier template [opt] simple-template-id :: [CHILD] [TODO: UNIMPLEMENTED]
     */
    pub fn optParseNestedNameSpecifier(
        &mut self,
        lexpos: &mut StateBufferedLexer,
    ) -> (&'static [AstNestedNameSpecifier], ParseMatched) {
        let startingTok = self.lexer().get(lexpos);
        let mut resVec;
        match startingTok {
            Some(fileTokPosMatchArm!(Token::DoubleColon)) => {
                resVec = vec![AstNestedNameSpecifier::new_scoped(
                    NestedNameSpecifier::Kind::Global,
                    SourceRange::newSingleTok(startingTok.unwrap()),
                    self.rootScope.clone(),
                )];
                self.lexer().next(lexpos);
            }
            Some(fileTokPosMatchArm!(Token::Identifier(ident))) => {
                if !self.lexer().ifEqOffset(lexpos, Token::DoubleColon, 1) {
                    return (&[], ParseMatched::NotMatched);
                }

                // Figure out what this name refers to
                let res = self.unqualifiedNameLookupWithCond(*ident, |child| {
                    match child {
                        // namespaces, classes, structs, unions, enums, etc have scope
                        crate::sema::scope::Child::Decl(_) => false,
                        crate::sema::scope::Child::Scope(scope) => scope
                            .borrow()
                            .flags
                            .intersects(ScopeKind::ENUM | ScopeKind::CLASS | ScopeKind::NAMESPACE),
                    }
                });

                if res.is_empty() {
                    self.errors.push(CompileError::fromPreTo(
                        "This identifier could not be resolved to a type or namespace.",
                        startingTok.unwrap(),
                    ));
                    // Our recovery strategy will be to use the global namespace. With a bit of luck, this will allow us to continue parsing.

                    return (
                        self.alloc()
                            .alloc_slice_clone(&[AstNestedNameSpecifier::new_scoped(
                                NestedNameSpecifier::Kind::Global,
                                SourceRange::newSingleTok(startingTok.unwrap()),
                                self.rootScope.clone(),
                            )]),
                        ParseMatched::Matched,
                    );
                }
                if res.len() > 1 {
                    self.errors.push(CompileError::fromPreTo(
                        "Somehow we resolved to multiple names for this identifier during the parsing of a nested name specifier. This is a bug. Please report it.",
                        startingTok.unwrap(),
                    ));
                    return (
                        self.alloc()
                            .alloc_slice_clone(&[AstNestedNameSpecifier::new_scoped(
                                NestedNameSpecifier::Kind::Global,
                                SourceRange::newSingleTok(startingTok.unwrap()),
                                self.rootScope.clone(),
                            )]),
                        ParseMatched::Matched,
                    );
                }

                if res[0]
                    .getScope()
                    .unwrap()
                    .borrow()
                    .flags
                    .contains(ScopeKind::NAMESPACE)
                {
                    resVec = vec![AstNestedNameSpecifier::new_scoped(
                        NestedNameSpecifier::Kind::Namespace(*ident),
                        SourceRange::newSingleTok(startingTok.unwrap()),
                        res[0].getScope().unwrap(),
                    )];
                } else {
                    resVec = vec![AstNestedNameSpecifier::new_scoped(
                        NestedNameSpecifier::Kind::Type(*ident),
                        SourceRange::newSingleTok(startingTok.unwrap()),
                        res[0].getScope().unwrap(),
                    )];
                }
            }
            _ => {
                return (&[], ParseMatched::NotMatched);
            }
        }
        while self
            .parseChildNestedNameSpecifier(lexpos, &mut resVec)
            .matched()
        {}

        (
            self.actOnNestedNameSpecifier(&resVec),
            ParseMatched::Matched,
        )
    }

    /**
     * Parses the childs of a nested-name-specifier:
     * nested-name-specifier:
     *  nested-name-specifier identifier :: [CHILD]
     *  nested-name-specifier template [opt] simple-template-id :: [CHILD] [TODO: UNIMPLEMENTED]
     */
    pub fn parseChildNestedNameSpecifier(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        result: &mut Vec<AstNestedNameSpecifier>,
    ) -> ParseMatched {
        // When we support the template id, this won't work anymore...
        if !self.lexer().ifEqOffset(lexpos, Token::DoubleColon, 1) {
            return ParseMatched::NotMatched;
        }
        // Don't reorder these lines please!
        let Some(identifier) = self.lexer().getConsumeTokenIfIdentifier(lexpos) else {
            return ParseMatched::NotMatched;
        };
        // Consume the double colon
        self.lexer().next(lexpos);

        let Token::Identifier(identifierStr) = identifier.tokPos.tok else {
            unreachable!()
        };
        result.push(AstNestedNameSpecifier::new(
            NestedNameSpecifier::Kind::Identifier(identifierStr),
            SourceRange::newSingleTok(identifier),
        ));
        ParseMatched::Matched
    }
}
