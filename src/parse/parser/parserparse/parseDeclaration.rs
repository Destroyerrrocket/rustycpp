use crate::{
    ast::{common::AstDecl, Attribute::AstAttribute},
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, CompileWarning, SourceRange},
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
    pub fn parseDeclaration(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        attr: Vec<&'static AstAttribute>,
    ) -> Vec<&'static AstDecl> {
        let tok = self.lexer().get(lexpos);
        if tok.is_none() {
            self.errors.push(CompileError::fromPreTo(
                "Unexpected end of file. Maybe you forgot a semicolon?",
                self.lexer().getWithOffsetSaturating(lexpos, 0),
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
                return vec![];
            }
            Token::RBrace => {
                self.errors
                    .push(CompileError::fromPreTo("Extra '}' found.", tok));
                self.lexer().next(lexpos);
                return vec![];
            }
            Token::RBracket => {
                self.errors
                    .push(CompileError::fromPreTo("Extra ']' found.", tok));
                self.lexer().next(lexpos);
                return vec![];
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
                return vec![];
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
                return vec![];
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
                return vec![];
            }
            Token::Module => {
                self.errors.push(CompileError::fromPreTo(
                    "Expected a declaration, but found the keyword 'module', which can't start one.",
                    tok,
                ));
                self.lexer().next(lexpos);
                return vec![];
            }
            /**
             * empty-declaration | attribute-declaration
             */
            Token::Semicolon => {
                self.lexer().next(lexpos);
                return self.actOnEmptyDecl(&attr, SourceRange::newSingleTok(tok));
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
            Token::Inline => todo!(),
            Token::Int => todo!(),
            Token::Long => todo!(),
            Token::Mutable => todo!(),
            Token::Namespace => todo!(),
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
        }
    }

    /**
     * asm-declaration:
     *    attribute-specifier-seq [opt] asm ( string-literal ) ;
     */
    pub fn parseAsmDeclaration(
        &mut self,
        lexpos: &mut StateBufferedLexer,
        attr: Vec<&'static AstAttribute>,
    ) -> Vec<&'static AstDecl> {
        let startlexpos = *lexpos;
        let startedAsm = self.lexer().consumeTokenIfEq(lexpos, Token::Asm);
        assert!(startedAsm);
        if self.lexer().getIfEq(lexpos, Token::LParen).is_none() {
            self.errors.push(CompileError::fromPreTo(
                "Expected '(' after 'asm' keyword.",
                self.lexer().getWithOffsetSaturating(lexpos, -1),
            ));
            return vec![];
        }
        let parenPos = *lexpos;
        if let Some(mut scoped) = self.parseBalancedPattern(lexpos) {
            if let Some(content) = self.lexer().getConsumeToken(&mut scoped) {
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
                return self.actOnAsmDecl(
                    &attr,
                    SourceRange::newDoubleTok(
                        self.lexer().getWithOffsetSaturating(&startlexpos, 0),
                        self.lexer().getWithOffsetSaturating(lexpos, -1),
                    ),
                    content,
                );
            } else {
                self.errors.push(CompileError::fromPreTo(
                    "Expected a string literal inside the 'asm' declaration.",
                    self.lexer().getWithOffsetSaturating(&parenPos, 0),
                ));
                return vec![];
            }
        } else {
            self.errors.push(CompileError::fromPreTo(
                "Expected a closing parentheses for this '(' while evaluating the 'asm' declaration.",
                self.lexer().getWithOffsetSaturating(&parenPos, 0),
            ));
            return vec![];
        }
    }
}
