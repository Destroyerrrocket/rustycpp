use crate::{
    ast::{Attribute::AstAttribute, Decl::AstDecl},
    fileTokPosMatchArm,
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{
        CompileError, CompileMsgImpl, CompileWarning, FileTokPos, SourceRange, TokPos,
    },
};

use super::super::Parser;

impl Parser {
    /**
     * declaration:
     *   block-declaration
     *   nodeclspec-function-declaration
     *   function-definition
     *   template-declaration
     *   deduction-guide
     *   explicit-instantiation
     *   explicit-specialization
     *   export-declaration
     *   linkage-specification
     *   namespace-definition
     *   empty-declaration
     *   attribute-declaration
     *   module-import-declaration
     *   module-declaration [NOT IMPLEMENTED HERE!]
     */
    #[allow(clippy::too_many_lines)]
    pub fn parseDeclaration(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        attr: &Vec<&'static AstAttribute>,
    ) -> Vec<&'static AstDecl> {
        let tok = self.lexer().get(lexpos);
        if tok.is_none() {
            let posErr = self.lexer().getWithOffsetSaturating(lexpos, 0);
            self.errors.push(CompileError::fromPreTo(
                "Unexpected end of file. Maybe you forgot a semicolon?",
                posErr,
            ));
            return vec![];
        }
        let tok = tok.unwrap();
        #[allow(unused_doc_comments)]
        match tok.tokPos.tok {
            Token::RParen => {
                self.errors
                    .push(CompileError::fromPreTo("Extra ')' found.", tok));
                self.lexer().next(lexpos);
                vec![]
            }
            Token::RBrace => {
                self.errors
                    .push(CompileError::fromPreTo("Extra '}' found.", tok));
                self.lexer().next(lexpos);
                vec![]
            }
            Token::RBracket => {
                self.errors
                    .push(CompileError::fromPreTo("Extra ']' found.", tok));
                self.lexer().next(lexpos);
                vec![]
            }
            Token::Alignas
            | Token::Break
            | Token::Case
            | Token::Catch
            | Token::Concept
            | Token::Const_cast
            | Token::Continue
            | Token::Co_await
            | Token::Co_return
            | Token::Co_yield
            | Token::Default
            | Token::Delete
            | Token::Do
            | Token::Dynamic_cast
            | Token::Else
            | Token::For
            | Token::Goto
            | Token::If
            | Token::New
            | Token::Operator
            | Token::Noexcept
            | Token::Private
            | Token::Protected
            | Token::Public
            | Token::Reinterpret_cast
            | Token::Requires
            | Token::Return
            | Token::Sizeof
            | Token::Static_cast
            | Token::Switch
            | Token::This
            | Token::Throw
            | Token::Try
            | Token::Typeid
            | Token::While => {
                let tokstr = tok.tokPos.tok.to_string();
                self.errors.push(CompileError::fromPreTo(
                    format!("Expected a declaration, but found the keyword '{tokstr}', which can't start one."),
                    tok,
                ));
                self.lexer().next(lexpos);
                vec![]
            }

            Token::Colon
            | Token::ThreeDots
            | Token::Question
            | Token::Dot
            | Token::DotStar
            | Token::Arrow
            | Token::ArrowStar
            | Token::Tilde
            | Token::Exclamation
            | Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Percent
            | Token::Caret
            | Token::Ampersand
            | Token::Pipe
            | Token::Equal
            | Token::PlusEqual
            | Token::MinusEqual
            | Token::StarEqual
            | Token::SlashEqual
            | Token::PercentEqual
            | Token::CaretEqual
            | Token::AmpersandEqual
            | Token::PipeEqual
            | Token::DoubleEqual
            | Token::ExclamationEqual
            | Token::Spaceship
            | Token::DoubleAmpersand
            | Token::DoublePipe
            | Token::DoubleLess
            | Token::DoubleLessEqual
            | Token::DoublePlus
            | Token::DoubleMinus
            | Token::Comma
            | Token::SingleGreater
            | Token::FirstGreater
            | Token::SecondGreater
            | Token::StrippedGreaterEqual
            | Token::LessEqual => {
                let tokstr = tok.tokPos.tok.to_string();
                self.errors.push(CompileError::fromPreTo(
                    format!("Expected a declaration, but found the punctuator '{tokstr}', which can't start one."),
                    tok,
                ));
                self.lexer().next(lexpos);
                vec![]
            }

            Token::IntegerLiteral(_, _)
            | Token::FloatingPointLiteral(_, _)
            | Token::CharacterLiteral(_, _)
            | Token::StringLiteral(_, _)
            | Token::BoolLiteral(_)
            | Token::PointerLiteral
            | Token::UdIntegerLiteral(_, _, _)
            | Token::UdFloatingPointLiteral(_, _, _)
            | Token::UdCharacterLiteral(_, _, _)
            | Token::UdStringLiteral(_, _, _) => {
                let tokstr = tok.tokPos.tok.to_string();
                self.errors.push(CompileError::fromPreTo(
                    format!("Expected a declaration, but found the literal '{tokstr}', which can't start one."),
                    tok,
                ));
                self.lexer().next(lexpos);
                vec![]
            }
            Token::Module => {
                self.errors.push(CompileError::fromPreTo(
                    "Expected a declaration, but found the keyword 'module', which can't start one.",
                    tok,
                ));
                self.lexer().next(lexpos);
                vec![]
            }
            /**
             * empty-declaration | attribute-declaration
             */
            Token::Semicolon => {
                self.lexer().next(lexpos);
                return self.actOnEmptyDecl(attr, SourceRange::newSingleTok(tok));
            }
            /**
             * asm-declaration:
             *     asm ( asm-argument-clause ) asm-operand-clause ;
             */
            Token::Asm => {
                return self.parseAsmDeclaration(lexpos, attr);
            }
            /**
             * enum-declaration:
             */
            Token::Enum => todo!(),

            Token::Identifier(_) => todo!(),
            Token::Alignof => todo!(),
            Token::Auto => todo!(),
            Token::Bool => todo!(),
            Token::Char => todo!(),
            Token::Char8_t => todo!(),
            Token::Char16_t => todo!(),
            Token::Char32_t => todo!(),
            Token::Class => todo!(),
            Token::Const => todo!(),
            Token::Consteval => todo!(),
            Token::Constexpr => todo!(),
            Token::Constinit => todo!(),
            Token::Decltype => todo!(),
            Token::Double => todo!(),
            Token::Explicit => todo!(),
            Token::Export => todo!(),
            Token::Extern => todo!(),
            Token::Float => todo!(),
            Token::Friend => todo!(),
            Token::Inline => {
                if self.lexer().ifEqOffset(lexpos, Token::Namespace, 1) {
                    self.parseNamespaceDeclaration(lexpos, attr)
                } else {
                    todo!()
                }
            }
            Token::Int => todo!(),
            Token::Long => todo!(),
            Token::Mutable => todo!(),
            Token::Namespace => self.parseNamespaceDeclaration(lexpos, attr),
            Token::Register => todo!(),
            Token::Short => todo!(),
            Token::Signed => todo!(),
            Token::Static => todo!(),
            Token::Static_assert => todo!(),
            Token::Struct => todo!(),
            Token::Template => todo!(),
            Token::Thread_local => todo!(),
            Token::Typedef => todo!(),
            Token::Typename => todo!(),
            Token::Union => todo!(),
            Token::Unsigned => todo!(),
            Token::Using => todo!(),
            Token::Virtual => todo!(),
            Token::Void => todo!(),
            Token::Volatile => todo!(),
            Token::Wchar_t => todo!(),
            Token::LBrace => todo!(),
            Token::LBracket => todo!(),
            Token::LParen => todo!(),
            Token::DoubleColon => todo!(),
            Token::Less => todo!(),
            Token::Import => todo!(),
            Token::ImportableHeaderName(_) => todo!(),
            Token::__rustycpp__ => self.parseCustom__rustycpp__Decl(lexpos, attr),
        }
    }

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
        attr: &[&'static AstAttribute],
    ) -> Vec<&'static AstDecl> {
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

        let attr = self.parseAttributes(lexpos);
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
                            &attr,
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
    pub fn parseNamespaceBody(&mut self, lexpos: &mut StateBufferedLexer) -> Vec<&'static AstDecl> {
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
                let newDecls = self.parseDeclaration(lexpos, &attrs);
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

    /**
     * __rustycpp__ (stuff) custom operator
     */
    fn parseCustom__rustycpp__Decl(
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
