use crate::{
    ast::{Attribute::AstAttribute, Decl::AstDecl},
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, SourceRange},
};

mod parseAsmDeclaration;
mod parseCustomRustycppDeclaration;
mod parseNamespaceDeclaration;
mod parseUsingNamespaceDeclaration;

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
        attr: &[&'static AstAttribute],
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
            Token::Using => {
                if self.lexer().ifEqOffset(lexpos, Token::Namespace, 1) {
                    self.parseUsingNamespaceDeclaration(lexpos, attr)
                } else {
                    todo!()
                }
            }
            Token::Virtual => todo!(),
            Token::Void => todo!(),
            Token::Volatile => todo!(),
            Token::Wchar_t => todo!(),
            Token::LBrace => todo!(),
            Token::LBracket => todo!(),
            Token::LParen => todo!(),
            Token::DoubleColon => todo!(),
            Token::Less => todo!(),
            Token::Import => {
                // TODO: Stub until we can accually import stuff
                self.lexer().next(lexpos);
                loop {
                    if self.lexer().getIfEq(lexpos, Token::Semicolon).is_some() {
                        return self.actOnEmptyDecl(attr, SourceRange::newSingleTok(tok));
                    } else if !self.lexer().consumeToken(lexpos) {
                        let pos = self.lexer().getWithOffsetSaturating(lexpos, 0);
                        self.errors
                            .push(CompileError::fromPreTo("Expected ';'", pos));
                        return vec![];
                    }
                }
            }
            Token::ImportableHeaderName(_) => unreachable!(),
            Token::__rustycpp__ => self.parseCustom__rustycpp__Decl(lexpos, attr),
        }
    }
}
