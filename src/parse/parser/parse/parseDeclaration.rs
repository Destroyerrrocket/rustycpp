use crate::{
    ast::{common::AstDecl, Attribute::AstAttribute},
    lex::token::Token,
    parse::bufferedLexer::StateBufferedLexer,
    utils::structs::{CompileError, CompileMsgImpl, SourceRange},
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
        self.lexer().consumeToken(lexpos);
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
            /**
             * empty-declaration | attribute-declaration
             */
            Token::Semicolon => return self.actOnEmptyDecl(attr, SourceRange::newSingleTok(tok)),
            Token::Identifier(_) => todo!(),
            Token::Alignas => todo!(),
            Token::Alignof => todo!(),
            Token::Asm => todo!(),
            Token::Auto => todo!(),
            Token::Bool => todo!(),
            Token::Break => todo!(),
            Token::Case => todo!(),
            Token::Catch => todo!(),
            Token::Char => todo!(),
            Token::Char8_t => todo!(),
            Token::Char16_t => todo!(),
            Token::Char32_t => todo!(),
            Token::Class => todo!(),
            Token::Concept => todo!(),
            Token::Const => todo!(),
            Token::Consteval => todo!(),
            Token::Constexpr => todo!(),
            Token::Constinit => todo!(),
            Token::Const_cast => todo!(),
            Token::Continue => todo!(),
            Token::Co_await => todo!(),
            Token::Co_return => todo!(),
            Token::Co_yield => todo!(),
            Token::Decltype => todo!(),
            Token::Default => todo!(),
            Token::Delete => todo!(),
            Token::Do => todo!(),
            Token::Double => todo!(),
            Token::Dynamic_cast => todo!(),
            Token::Else => todo!(),
            Token::Enum => todo!(),
            Token::Explicit => todo!(),
            Token::Export => todo!(),
            Token::Extern => todo!(),
            Token::Float => todo!(),
            Token::For => todo!(),
            Token::Friend => todo!(),
            Token::Goto => todo!(),
            Token::If => todo!(),
            Token::Inline => todo!(),
            Token::Int => todo!(),
            Token::Long => todo!(),
            Token::Mutable => todo!(),
            Token::Namespace => todo!(),
            Token::New => todo!(),
            Token::Noexcept => todo!(),
            Token::Operator => todo!(),
            Token::Private => todo!(),
            Token::Protected => todo!(),
            Token::Public => todo!(),
            Token::Register => todo!(),
            Token::Reinterpret_cast => todo!(),
            Token::Requires => todo!(),
            Token::Return => todo!(),
            Token::Short => todo!(),
            Token::Signed => todo!(),
            Token::Sizeof => todo!(),
            Token::Static => todo!(),
            Token::Static_assert => todo!(),
            Token::Static_cast => todo!(),
            Token::Struct => todo!(),
            Token::Switch => todo!(),
            Token::Template => todo!(),
            Token::This => todo!(),
            Token::Thread_local => todo!(),
            Token::Throw => todo!(),
            Token::Try => todo!(),
            Token::Typedef => todo!(),
            Token::Typeid => todo!(),
            Token::Typename => todo!(),
            Token::Union => todo!(),
            Token::Unsigned => todo!(),
            Token::Using => todo!(),
            Token::Virtual => todo!(),
            Token::Void => todo!(),
            Token::Volatile => todo!(),
            Token::Wchar_t => todo!(),
            Token::While => todo!(),
            Token::LBrace => todo!(),
            Token::RBrace => todo!(),
            Token::LBracket => todo!(),
            Token::RBracket => todo!(),
            Token::LParen => todo!(),
            Token::RParen => todo!(),
            Token::Colon => todo!(),
            Token::ThreeDots => todo!(),
            Token::Question => todo!(),
            Token::DoubleColon => todo!(),
            Token::Dot => todo!(),
            Token::DotStar => todo!(),
            Token::Arrow => todo!(),
            Token::ArrowStar => todo!(),
            Token::Tilde => todo!(),
            Token::Exclamation => todo!(),
            Token::Plus => todo!(),
            Token::Minus => todo!(),
            Token::Star => todo!(),
            Token::Slash => todo!(),
            Token::Percent => todo!(),
            Token::Caret => todo!(),
            Token::Ampersand => todo!(),
            Token::Pipe => todo!(),
            Token::Equal => todo!(),
            Token::PlusEqual => todo!(),
            Token::MinusEqual => todo!(),
            Token::StarEqual => todo!(),
            Token::SlashEqual => todo!(),
            Token::PercentEqual => todo!(),
            Token::CaretEqual => todo!(),
            Token::AmpersandEqual => todo!(),
            Token::PipeEqual => todo!(),
            Token::DoubleEqual => todo!(),
            Token::ExclamationEqual => todo!(),
            Token::Less => todo!(),
            Token::LessEqual => todo!(),
            Token::Spaceship => todo!(),
            Token::DoubleAmpersand => todo!(),
            Token::DoublePipe => todo!(),
            Token::DoubleLess => todo!(),
            Token::DoubleLessEqual => todo!(),
            Token::DoublePlus => todo!(),
            Token::DoubleMinus => todo!(),
            Token::Comma => todo!(),
            Token::SingleGreater => todo!(),
            Token::FirstGreater => todo!(),
            Token::SecondGreater => todo!(),
            Token::StrippedGreaterEqual => todo!(),
            Token::Import => todo!(),
            Token::ImportableHeaderName(_) => todo!(),
            Token::Module => todo!(),
            Token::IntegerLiteral(_, _) => todo!(),
            Token::FloatingPointLiteral(_, _) => todo!(),
            Token::CharacterLiteral(_, _) => todo!(),
            Token::StringLiteral(_, _) => todo!(),
            Token::BoolLiteral(_) => todo!(),
            Token::PointerLiteral => todo!(),
            Token::UdIntegerLiteral(_, _, _) => todo!(),
            Token::UdFloatingPointLiteral(_, _, _) => todo!(),
            Token::UdCharacterLiteral(_, _, _) => todo!(),
            Token::UdStringLiteral(_, _, _) => todo!(),
        }
    }
}
