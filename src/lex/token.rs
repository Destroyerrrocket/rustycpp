//! Tokens of a C++ file after preprocessing.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::use_self,
    non_camel_case_types
)]

use std::collections::VecDeque;

use crate::preprocessor::pretoken::PreToken;
use crate::utils::stringref::{StringRef, ToStringRef};
use crate::utils::structs::{CompileError, CompileMsg, CompileMsgImpl, FileTokPos, TokPos};
use lazy_regex::regex_captures;
use logos::Logos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingPrefix {
    None,
    u8,
    u,
    U,
    L,
}

impl std::fmt::Display for EncodingPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingPrefix::None => write!(f, ""),
            EncodingPrefix::u8 => write!(f, "u8"),
            EncodingPrefix::u => write!(f, "u"),
            EncodingPrefix::U => write!(f, "U"),
            EncodingPrefix::L => write!(f, "L"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerSuffixLength {
    Long,
    LongLong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerSuffixSignedness {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerSuffix(pub Option<IntegerSuffixLength>, pub IntegerSuffixSignedness);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSuffix {
    None,
    F,
    L,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    // Identifiers
    Identifier(StringRef),

    // Keywords
    Alignas,
    Alignof,
    Asm,
    Auto,
    Bool,
    Break,
    Case,
    Catch,
    Char,
    Char8_t,
    Char16_t,
    Char32_t,
    Class,
    Concept,
    Const,
    Consteval,
    Constexpr,
    Constinit,
    Const_cast,
    Continue,
    Co_await,
    Co_return,
    Co_yield,
    Decltype,
    Default,
    Delete,
    Do,
    Double,
    Dynamic_cast,
    Else,
    Enum,
    Explicit,
    Export,
    Extern,
    //False, // Taken by bool literal
    Float,
    For,
    Friend,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Mutable,
    Namespace,
    New,
    Noexcept,
    // Nullptr, // Taken by pointer literal
    Operator,
    Private,
    Protected,
    Public,
    Register,
    Reinterpret_cast,
    Requires,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Static_assert,
    Static_cast,
    Struct,
    Switch,
    Template,
    This,
    Thread_local,
    Throw,
    //True, // Taken by bool literal
    Try,
    Typedef,
    Typeid,
    Typename,
    Union,
    Unsigned,
    Using,
    Virtual,
    Void,
    Volatile,
    Wchar_t,
    While,
    __rustycpp__,

    // Operators / Punctuators
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Semicolon,
    Colon,
    ThreeDots,
    Question,
    DoubleColon,
    Dot,
    DotStar,
    Arrow,
    ArrowStar,
    Tilde,
    Exclamation,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    CaretEqual,
    AmpersandEqual,
    PipeEqual,
    DoubleEqual,
    ExclamationEqual,
    Less,
    LessEqual,
    Spaceship,
    DoubleAmpersand,
    DoublePipe,
    DoubleLess,
    DoubleLessEqual,
    DoublePlus,
    DoubleMinus,
    Comma,
    SingleGreater,
    FirstGreater,
    SecondGreater,
    StrippedGreaterEqual,
    // Module conditional token
    Import,
    ImportableHeaderName(StringRef),
    Module,
    // Literals
    IntegerLiteral(i128, IntegerSuffix),
    FloatingPointLiteral(f128::f128, FloatSuffix),
    CharacterLiteral(EncodingPrefix, char),
    StringLiteral(EncodingPrefix, StringRef),
    BoolLiteral(bool),
    PointerLiteral,
    UdIntegerLiteral(i128, IntegerSuffix, StringRef),
    UdFloatingPointLiteral(f128::f128, FloatSuffix, StringRef),
    UdCharacterLiteral(EncodingPrefix, char, StringRef),
    UdStringLiteral(EncodingPrefix, StringRef, StringRef),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(s) => write!(f, "{s}"),
            Self::Alignas => write!(f, "alignas"),
            Self::Alignof => write!(f, "alignof"),
            Self::Asm => write!(f, "asm"),
            Self::Auto => write!(f, "auto"),
            Self::Bool => write!(f, "bool"),
            Self::Break => write!(f, "break"),
            Self::Case => write!(f, "case"),
            Self::Catch => write!(f, "catch"),
            Self::Char => write!(f, "char"),
            Self::Char8_t => write!(f, "char8_t"),
            Self::Char16_t => write!(f, "char16_t"),
            Self::Char32_t => write!(f, "char32_t"),
            Self::Class => write!(f, "class"),
            Self::Concept => write!(f, "concept"),
            Self::Const => write!(f, "const"),
            Self::Consteval => write!(f, "consteval"),
            Self::Constexpr => write!(f, "constexpr"),
            Self::Constinit => write!(f, "constinit"),
            Self::Const_cast => write!(f, "const_cast"),
            Self::Continue => write!(f, "continue"),
            Self::Co_await => write!(f, "co_await"),
            Self::Co_return => write!(f, "co_return"),
            Self::Co_yield => write!(f, "co_yield"),
            Self::Decltype => write!(f, "decltype"),
            Self::Default => write!(f, "default"),
            Self::Delete => write!(f, "delete"),
            Self::Do => write!(f, "do"),
            Self::Double => write!(f, "double"),
            Self::Dynamic_cast => write!(f, "dynamic_cast"),
            Self::Else => write!(f, "else"),
            Self::Enum => write!(f, "enum"),
            Self::Explicit => write!(f, "explicit"),
            Self::Export => write!(f, "export"),
            Self::Extern => write!(f, "extern"),
            Self::Float => write!(f, "float"),
            Self::For => write!(f, "for"),
            Self::Friend => write!(f, "friend"),
            Self::Goto => write!(f, "goto"),
            Self::If => write!(f, "if"),
            Self::Inline => write!(f, "inline"),
            Self::Int => write!(f, "int"),
            Self::Long => write!(f, "long"),
            Self::Mutable => write!(f, "mutable"),
            Self::Namespace => write!(f, "namespace"),
            Self::New => write!(f, "new"),
            Self::Noexcept => write!(f, "noexcept"),
            Self::Operator => write!(f, "operator"),
            Self::Private => write!(f, "private"),
            Self::Protected => write!(f, "protected"),
            Self::Public => write!(f, "public"),
            Self::Register => write!(f, "register"),
            Self::Reinterpret_cast => write!(f, "reinterpret_cast"),
            Self::Requires => write!(f, "requires"),
            Self::Return => write!(f, "return"),
            Self::Short => write!(f, "short"),
            Self::Signed => write!(f, "signed"),
            Self::Sizeof => write!(f, "sizeof"),
            Self::Static => write!(f, "static"),
            Self::Static_assert => write!(f, "static_assert"),
            Self::Static_cast => write!(f, "static_cast"),
            Self::Struct => write!(f, "struct"),
            Self::Switch => write!(f, "switch"),
            Self::Template => write!(f, "template"),
            Self::This => write!(f, "this"),
            Self::Thread_local => write!(f, "thread_local"),
            Self::Throw => write!(f, "throw"),
            Self::Try => write!(f, "try"),
            Self::Typedef => write!(f, "typedef"),
            Self::Typeid => write!(f, "typeid"),
            Self::Typename => write!(f, "typename"),
            Self::Union => write!(f, "union"),
            Self::Unsigned => write!(f, "unsigned"),
            Self::Using => write!(f, "using"),
            Self::Virtual => write!(f, "virtual"),
            Self::Void => write!(f, "void"),
            Self::Volatile => write!(f, "volatile"),
            Self::Wchar_t => write!(f, "wchar_t"),
            Self::While => write!(f, "while"),
            Self::__rustycpp__ => write!(f, "__rustycpp__"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::Semicolon => write!(f, ";"),
            Self::Colon => write!(f, ":"),
            Self::ThreeDots => write!(f, "..."),
            Self::Question => write!(f, "?"),
            Self::DoubleColon => write!(f, "::"),
            Self::Dot => write!(f, "."),
            Self::DotStar => write!(f, ".*"),
            Self::Arrow => write!(f, "->"),
            Self::ArrowStar => write!(f, "->*"),
            Self::Tilde => write!(f, "~"),
            Self::Exclamation => write!(f, "!"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Caret => write!(f, "^"),
            Self::Ampersand => write!(f, "&"),
            Self::Pipe => write!(f, "|"),
            Self::Equal => write!(f, "="),
            Self::PlusEqual => write!(f, "+="),
            Self::MinusEqual => write!(f, "-="),
            Self::StarEqual => write!(f, "*="),
            Self::SlashEqual => write!(f, "/="),
            Self::PercentEqual => write!(f, "%="),
            Self::CaretEqual => write!(f, "^="),
            Self::AmpersandEqual => write!(f, "&="),
            Self::PipeEqual => write!(f, "|="),
            Self::DoubleEqual => write!(f, "=="),
            Self::ExclamationEqual => write!(f, "!="),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::Spaceship => write!(f, "<=>"),
            Self::DoubleAmpersand => write!(f, "&&"),
            Self::DoublePipe => write!(f, "||"),
            Self::DoubleLess => write!(f, "<<"),
            Self::DoubleLessEqual => write!(f, "<<="),
            Self::DoublePlus => write!(f, "++"),
            Self::DoubleMinus => write!(f, "--"),
            Self::Comma => write!(f, ","),
            Self::SingleGreater => write!(f, "(single) >"),
            Self::FirstGreater => write!(f, "(first) >"),
            Self::SecondGreater => write!(f, "(second) >"),
            Self::StrippedGreaterEqual => write!(f, "(stripped) >="),
            Self::Import => write!(f, "import"),
            Self::ImportableHeaderName(s) => write!(f, "{}", &s),
            Self::Module => write!(f, "module"),
            Self::UdIntegerLiteral(i, _, _) | Self::IntegerLiteral(i, _) => {
                write!(f, "{i}")
            }
            Self::UdFloatingPointLiteral(fu, _, _) | Self::FloatingPointLiteral(fu, _) => {
                write!(f, "{fu}")
            }
            Self::UdCharacterLiteral(_, c, _) | Self::CharacterLiteral(_, c) => {
                write!(f, "{c}")
            }
            Self::UdStringLiteral(_, s, _) | Self::StringLiteral(_, s) => {
                write!(f, "{s}")
            }
            Self::BoolLiteral(b) => write!(f, "{b}"),
            Self::PointerLiteral => write!(f, "nullptr"),
        }
    }
}

impl Token {
    pub fn from_preToken(
        preTok: FileTokPos<PreToken>,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        match preTok.tokPos.tok {
            PreToken::Unknown(ref text) => Err(Some(CompileError::fromPreTo(
                format!("Unknown token: {text}"),
                &preTok,
            ))),
            PreToken::HeaderName(ref text) => Err(Some(CompileError::fromPreTo(
                format!("Header name token cannot be used at the next step of the compilation. It should be used inside a #include directive, or in a __has_include macro. Header name: {text}"),
                &preTok,
            ))),
            PreToken::Ident(text) => Ok({ let mut vec = VecDeque::new(); vec.push_back(FileTokPos::new(preTok.file, TokPos {
                tok: Self::Identifier(text.to_StringRef()),
                start: preTok.tokPos.start,
                end: preTok.tokPos.end,
            })); vec}),
            PreToken::PreprocessingOperator(_) => Err(Some(CompileError::fromPreTo(
                "Preprocessing operators cannot be used at the next step of the compilation. Make sure that any stray # and ## are no longer present after preprocessing.",
                &preTok,
            ))),
            PreToken::OperatorPunctuator(text) => Self::parseOperatorPunctuator(&preTok, text),
            PreToken::Keyword(text) => Self::parseKeyword(&preTok, text),
            PreToken::CharLiteral(ref text) => Self::parseCharLiteral(&preTok, text),
            PreToken::StringLiteral(ref text) => Self::parseStringLiteral(&preTok, text),
            PreToken::RawStringLiteral(ref text) => Self::parseRawStringLiteral(&preTok, text),
            PreToken::PPNumber(ref text) => Self::parsePPNumber(&preTok, text),
            PreToken::UdCharLiteral(ref text) => Self::parseUdCharLiteral(&preTok, text),
            PreToken::UdStringLiteral(ref text) => Self::parseUdStringLiteral(&preTok, text),
            PreToken::ValidNop | PreToken::DisableMacro(_) | PreToken::EnableMacro(_) | PreToken::Newline | PreToken::Whitespace(_) => Err(None),
            PreToken::Module => Ok({let mut vec = VecDeque::new(); vec.push_back(FileTokPos::new_meta_c(Self::Module, &preTok)); vec}),
            PreToken::Import => Ok({let mut vec = VecDeque::new(); vec.push_back(FileTokPos::new_meta_c(Self::Import, &preTok)); vec}),
            PreToken::ImportableHeaderName(text) => Ok({let mut vec = VecDeque::new(); vec.push_back(FileTokPos::new(preTok.file, TokPos {
                tok: Self::ImportableHeaderName(text.to_StringRef()),
                start: preTok.tokPos.start,
                end: preTok.tokPos.end,
            })); vec}),
        }
    }

    pub fn parseOperatorPunctuator<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        operator: &str,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        match operator {
            r"{" | r"<%" => Ok(vec![FileTokPos::new_meta_c(Self::LBrace, tok)]),
            r"}" | r"%>" => Ok(vec![FileTokPos::new_meta_c(Self::RBrace, tok)]),
            r"[" | r"<:" => Ok(vec![FileTokPos::new_meta_c(Self::LBracket, tok)]),
            r"]" | r":>" => Ok(vec![FileTokPos::new_meta_c(Self::RBracket, tok)]),
            r"(" => Ok(vec![FileTokPos::new_meta_c(Self::LParen, tok)]),
            r")" => Ok(vec![FileTokPos::new_meta_c(Self::RParen, tok)]),
            r";" => Ok(vec![FileTokPos::new_meta_c(Self::Semicolon, tok)]),
            r":" => Ok(vec![FileTokPos::new_meta_c(Self::Colon, tok)]),
            r"..." => Ok(vec![FileTokPos::new_meta_c(Self::ThreeDots, tok)]),
            r"?" => Ok(vec![FileTokPos::new_meta_c(Self::Question, tok)]),
            r"::" => Ok(vec![FileTokPos::new_meta_c(Self::DoubleColon, tok)]),
            r"." => Ok(vec![FileTokPos::new_meta_c(Self::Dot, tok)]),
            r".*" => Ok(vec![FileTokPos::new_meta_c(Self::DotStar, tok)]),
            r"->" => Ok(vec![FileTokPos::new_meta_c(Self::Arrow, tok)]),
            r"->*" => Ok(vec![FileTokPos::new_meta_c(Self::ArrowStar, tok)]),
            r"~" | r"compl" => Ok(vec![FileTokPos::new_meta_c(Self::Tilde, tok)]),
            r"!" | r"not" => Ok(vec![FileTokPos::new_meta_c(Self::Exclamation, tok)]),
            r"+" => Ok(vec![FileTokPos::new_meta_c(Self::Plus, tok)]),
            r"-" => Ok(vec![FileTokPos::new_meta_c(Self::Minus, tok)]),
            r"*" => Ok(vec![FileTokPos::new_meta_c(Self::Star, tok)]),
            r"/" => Ok(vec![FileTokPos::new_meta_c(Self::Slash, tok)]),
            r"%" => Ok(vec![FileTokPos::new_meta_c(Self::Percent, tok)]),
            r"^" | r"xor" => Ok(vec![FileTokPos::new_meta_c(Self::Caret, tok)]),
            r"&" | r"bitand" => Ok(vec![FileTokPos::new_meta_c(Self::Ampersand, tok)]),
            r"|" | r"bitor" => Ok(vec![FileTokPos::new_meta_c(Self::Pipe, tok)]),
            r"=" => Ok(vec![FileTokPos::new_meta_c(Self::Equal, tok)]),
            r"+=" => Ok(vec![FileTokPos::new_meta_c(Self::PlusEqual, tok)]),
            r"-=" => Ok(vec![FileTokPos::new_meta_c(Self::MinusEqual, tok)]),
            r"*=" => Ok(vec![FileTokPos::new_meta_c(Self::StarEqual, tok)]),
            r"/=" => Ok(vec![FileTokPos::new_meta_c(Self::SlashEqual, tok)]),
            r"%=" => Ok(vec![FileTokPos::new_meta_c(Self::PercentEqual, tok)]),
            r"^=" => Ok(vec![FileTokPos::new_meta_c(Self::CaretEqual, tok)]),
            r"&=" => Ok(vec![FileTokPos::new_meta_c(Self::AmpersandEqual, tok)]),
            r"|=" => Ok(vec![FileTokPos::new_meta_c(Self::PipeEqual, tok)]),
            r"==" => Ok(vec![FileTokPos::new_meta_c(Self::DoubleEqual, tok)]),
            r"!=" => Ok(vec![FileTokPos::new_meta_c(Self::ExclamationEqual, tok)]),
            r"<" => Ok(vec![FileTokPos::new_meta_c(Self::Less, tok)]),
            r">" => Ok(vec![FileTokPos::new_meta_c(Self::SingleGreater, tok)]),
            r"<=" => Ok(vec![FileTokPos::new_meta_c(Self::LessEqual, tok)]),
            r">=" => Ok(vec![
                FileTokPos::new_meta_c(Self::SingleGreater, tok),
                FileTokPos::new_meta_c(Self::StrippedGreaterEqual, tok),
            ]),
            r"<=>" => Ok(vec![FileTokPos::new_meta_c(Self::Spaceship, tok)]),
            r"&&" | r"and" => Ok(vec![FileTokPos::new_meta_c(Self::DoubleAmpersand, tok)]),
            r"||" | r"or" => Ok(vec![FileTokPos::new_meta_c(Self::DoublePipe, tok)]),
            r"<<" => Ok(vec![FileTokPos::new_meta_c(Self::DoubleLess, tok)]),
            r">>" => Ok(vec![
                FileTokPos::new_meta_c(Self::FirstGreater, tok),
                FileTokPos::new_meta_c(Self::SecondGreater, tok),
            ]),
            r"<<=" => Ok(vec![FileTokPos::new_meta_c(Self::DoubleLessEqual, tok)]),
            r">>=" => Ok(vec![
                FileTokPos::new_meta_c(Self::FirstGreater, tok),
                FileTokPos::new_meta_c(Self::SecondGreater, tok),
                FileTokPos::new_meta_c(Self::StrippedGreaterEqual, tok),
            ]),
            r"++" => Ok(vec![FileTokPos::new_meta_c(Self::DoublePlus, tok)]),
            r"--" => Ok(vec![FileTokPos::new_meta_c(Self::DoubleMinus, tok)]),
            r"," => Ok(vec![FileTokPos::new_meta_c(Self::Comma, tok)]),
            _ => Err(Some(CompileError::fromPreTo(
                format!("Unknown operator: {operator}"),
                tok,
            ))),
        }
        .map(VecDeque::from)
    }

    pub fn parseKeyword<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        operator: &str,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        match operator {
            r"alignas" => Ok(FileTokPos::new_meta_c(Self::Alignas, tok)),
            r"alignof" => Ok(FileTokPos::new_meta_c(Self::Alignof, tok)),
            r"asm" => Ok(FileTokPos::new_meta_c(Self::Asm, tok)),
            r"auto" => Ok(FileTokPos::new_meta_c(Self::Auto, tok)),
            r"bool" => Ok(FileTokPos::new_meta_c(Self::Bool, tok)),
            r"break" => Ok(FileTokPos::new_meta_c(Self::Break, tok)),
            r"case" => Ok(FileTokPos::new_meta_c(Self::Case, tok)),
            r"catch" => Ok(FileTokPos::new_meta_c(Self::Catch, tok)),
            r"char" => Ok(FileTokPos::new_meta_c(Self::Char, tok)),
            r"char8_t" => Ok(FileTokPos::new_meta_c(Self::Char8_t, tok)),
            r"char16_t" => Ok(FileTokPos::new_meta_c(Self::Char16_t, tok)),
            r"char32_t" => Ok(FileTokPos::new_meta_c(Self::Char32_t, tok)),
            r"class" => Ok(FileTokPos::new_meta_c(Self::Class, tok)),
            r"concept" => Ok(FileTokPos::new_meta_c(Self::Concept, tok)),
            r"const" => Ok(FileTokPos::new_meta_c(Self::Const, tok)),
            r"consteval" => Ok(FileTokPos::new_meta_c(Self::Consteval, tok)),
            r"constexpr" => Ok(FileTokPos::new_meta_c(Self::Constexpr, tok)),
            r"constinit" => Ok(FileTokPos::new_meta_c(Self::Constinit, tok)),
            r"const_cast" => Ok(FileTokPos::new_meta_c(Self::Const_cast, tok)),
            r"continue" => Ok(FileTokPos::new_meta_c(Self::Continue, tok)),
            r"co_await" => Ok(FileTokPos::new_meta_c(Self::Co_await, tok)),
            r"co_return" => Ok(FileTokPos::new_meta_c(Self::Co_return, tok)),
            r"co_yield" => Ok(FileTokPos::new_meta_c(Self::Co_yield, tok)),
            r"decltype" => Ok(FileTokPos::new_meta_c(Self::Decltype, tok)),
            r"default" => Ok(FileTokPos::new_meta_c(Self::Default, tok)),
            r"delete" => Ok(FileTokPos::new_meta_c(Self::Delete, tok)),
            r"do" => Ok(FileTokPos::new_meta_c(Self::Do, tok)),
            r"double" => Ok(FileTokPos::new_meta_c(Self::Double, tok)),
            r"dynamic_cast" => Ok(FileTokPos::new_meta_c(Self::Dynamic_cast, tok)),
            r"else" => Ok(FileTokPos::new_meta_c(Self::Else, tok)),
            r"enum" => Ok(FileTokPos::new_meta_c(Self::Enum, tok)),
            r"explicit" => Ok(FileTokPos::new_meta_c(Self::Explicit, tok)),
            r"export" => Ok(FileTokPos::new_meta_c(Self::Export, tok)),
            r"extern" => Ok(FileTokPos::new_meta_c(Self::Extern, tok)),
            r"false" => Ok(FileTokPos::new_meta_c(Self::BoolLiteral(false), tok)),
            r"float" => Ok(FileTokPos::new_meta_c(Self::Float, tok)),
            r"for" => Ok(FileTokPos::new_meta_c(Self::For, tok)),
            r"friend" => Ok(FileTokPos::new_meta_c(Self::Friend, tok)),
            r"goto" => Ok(FileTokPos::new_meta_c(Self::Goto, tok)),
            r"if" => Ok(FileTokPos::new_meta_c(Self::If, tok)),
            r"inline" => Ok(FileTokPos::new_meta_c(Self::Inline, tok)),
            r"int" => Ok(FileTokPos::new_meta_c(Self::Int, tok)),
            r"long" => Ok(FileTokPos::new_meta_c(Self::Long, tok)),
            r"mutable" => Ok(FileTokPos::new_meta_c(Self::Mutable, tok)),
            r"namespace" => Ok(FileTokPos::new_meta_c(Self::Namespace, tok)),
            r"new" => Ok(FileTokPos::new_meta_c(Self::New, tok)),
            r"noexcept" => Ok(FileTokPos::new_meta_c(Self::Noexcept, tok)),
            r"nullptr" => Ok(FileTokPos::new_meta_c(Self::PointerLiteral, tok)),
            r"operator" => Ok(FileTokPos::new_meta_c(Self::Operator, tok)),
            r"private" => Ok(FileTokPos::new_meta_c(Self::Private, tok)),
            r"protected" => Ok(FileTokPos::new_meta_c(Self::Protected, tok)),
            r"public" => Ok(FileTokPos::new_meta_c(Self::Public, tok)),
            r"register" => Ok(FileTokPos::new_meta_c(Self::Register, tok)),
            r"reinterpret_cast" => Ok(FileTokPos::new_meta_c(Self::Reinterpret_cast, tok)),
            r"requires" => Ok(FileTokPos::new_meta_c(Self::Requires, tok)),
            r"return" => Ok(FileTokPos::new_meta_c(Self::Return, tok)),
            r"short" => Ok(FileTokPos::new_meta_c(Self::Short, tok)),
            r"signed" => Ok(FileTokPos::new_meta_c(Self::Signed, tok)),
            r"sizeof" => Ok(FileTokPos::new_meta_c(Self::Sizeof, tok)),
            r"static" => Ok(FileTokPos::new_meta_c(Self::Static, tok)),
            r"static_assert" => Ok(FileTokPos::new_meta_c(Self::Static_assert, tok)),
            r"static_cast" => Ok(FileTokPos::new_meta_c(Self::Static_cast, tok)),
            r"struct" => Ok(FileTokPos::new_meta_c(Self::Struct, tok)),
            r"switch" => Ok(FileTokPos::new_meta_c(Self::Switch, tok)),
            r"template" => Ok(FileTokPos::new_meta_c(Self::Template, tok)),
            r"this" => Ok(FileTokPos::new_meta_c(Self::This, tok)),
            r"thread_local" => Ok(FileTokPos::new_meta_c(Self::Thread_local, tok)),
            r"throw" => Ok(FileTokPos::new_meta_c(Self::Throw, tok)),
            r"true" => Ok(FileTokPos::new_meta_c(Self::BoolLiteral(true), tok)),
            r"try" => Ok(FileTokPos::new_meta_c(Self::Try, tok)),
            r"typedef" => Ok(FileTokPos::new_meta_c(Self::Typedef, tok)),
            r"typeid" => Ok(FileTokPos::new_meta_c(Self::Typeid, tok)),
            r"typename" => Ok(FileTokPos::new_meta_c(Self::Typename, tok)),
            r"union" => Ok(FileTokPos::new_meta_c(Self::Union, tok)),
            r"unsigned" => Ok(FileTokPos::new_meta_c(Self::Unsigned, tok)),
            r"using" => Ok(FileTokPos::new_meta_c(Self::Using, tok)),
            r"virtual" => Ok(FileTokPos::new_meta_c(Self::Virtual, tok)),
            r"void" => Ok(FileTokPos::new_meta_c(Self::Void, tok)),
            r"volatile" => Ok(FileTokPos::new_meta_c(Self::Volatile, tok)),
            r"wchar_t" => Ok(FileTokPos::new_meta_c(Self::Wchar_t, tok)),
            r"while" => Ok(FileTokPos::new_meta_c(Self::While, tok)),
            r"__rustycpp__" => Ok(FileTokPos::new_meta_c(Self::__rustycpp__, tok)),
            _ => Err(Some(CompileError::fromPreTo(
                format!("Unknown token: {operator}"),
                tok,
            ))),
        }
        .map(|x| {
            let mut vec = VecDeque::new();
            vec.push_back(x);
            vec
        })
    }

    fn getEncodingPrefix(string: &str) -> (EncodingPrefix, &str) {
        #[allow(clippy::option_if_let_else)]
        if let Some(res) = string.strip_prefix("u8") {
            (EncodingPrefix::u8, res)
        } else if let Some(res) = string.strip_prefix('u') {
            (EncodingPrefix::u, res)
        } else if let Some(res) = string.strip_prefix('U') {
            (EncodingPrefix::U, res)
        } else if let Some(res) = string.strip_prefix('L') {
            (EncodingPrefix::L, res)
        } else {
            (EncodingPrefix::None, string)
        }
    }

    fn escapeString(msg: &str) -> Result<String, String> {
        #[derive(PartialEq, Eq, Debug, Logos)]
        enum EscapeLexer {
            #[regex(r"[^\\]")]
            Character,
            #[regex(r"\\n")]
            NL,
            #[regex(r"\\t")]
            HT,
            #[regex(r"\\v")]
            VT,
            #[regex(r"\\b")]
            BS,
            #[regex(r"\\r")]
            CR,
            #[regex(r"\\f")]
            FF,
            #[regex(r"\\a")]
            Bel,
            #[regex(r"\\\\")]
            Backslash,
            #[regex(r"\\\?")]
            Question,
            #[regex(r"\\'")]
            SingleQuote,
            #[regex(r#"\\""#)]
            DoubleQuote,
            #[regex(r#"\\[0-7][0-7]?[0-7]?"#)]
            Octal,
            #[regex(r#"\\x[0-9a-fA-F]+"#)]
            Hex,
            #[error]
            Error,
        }
        let mut result = String::new();
        let mut lexer = EscapeLexer::lexer(msg);
        loop {
            let t = lexer.next();
            if t.is_none() {
                break;
            }
            let t = t.unwrap();
            match t {
                EscapeLexer::Character => result.push(lexer.slice().chars().next().unwrap()),
                EscapeLexer::NL => result.push('\n'),
                EscapeLexer::HT => result.push('\t'),
                EscapeLexer::VT => result.push('\x0B'),
                EscapeLexer::BS => result.push('\x08'),
                EscapeLexer::CR => result.push('\r'),
                EscapeLexer::FF => result.push('\x0C'),
                EscapeLexer::Bel => result.push('\x07'),
                EscapeLexer::Backslash => result.push('\\'),
                EscapeLexer::Question => result.push('?'),
                EscapeLexer::SingleQuote => result.push('\''),
                EscapeLexer::DoubleQuote => result.push('"'),
                EscapeLexer::Octal => {
                    let octal = lexer.slice();
                    let octalNum = &octal[1..];
                    let octal = u32::from_str_radix(octalNum, 8);
                    if octal.is_err() {
                        return Err(format!(
                            "Invalid octal escape: {} for input: {}",
                            octal.unwrap_err(),
                            octalNum
                        ));
                    }
                    let octal = octal.unwrap();
                    let res = char::from_u32(octal);
                    if res.is_none() {
                        return Err(format!("out of range octal escape for input: {octalNum}"));
                    }
                    result.push(res.unwrap());
                }
                EscapeLexer::Hex => {
                    let hex = lexer.slice();
                    let hexNum = &hex[2..];
                    let hex = u32::from_str_radix(hexNum, 16);
                    if hex.is_err() {
                        return Err(format!(
                            "Invalid hex escape: {} for input: {}",
                            hex.unwrap_err(),
                            hexNum
                        ));
                    }
                    let hex = hex.unwrap();
                    let res = char::from_u32(hex);
                    if res.is_none() {
                        return Err(format!("out of range hex escape for input: {hexNum}"));
                    }
                    result.push(res.unwrap());
                }
                EscapeLexer::Error => {
                    let mut size = lexer.span();
                    if size.end < msg.len() - 1 {
                        size.end += 1;
                    }
                    return Err(format!("Invalid escape sequence: {}", &msg[size]));
                }
            }
        }
        Ok(result)
    }

    pub fn parseCharLiteral<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        operator: &str,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        let (encoding, message) = Self::getEncodingPrefix(operator);
        if message.len() < 3 {
            return Err(Some(CompileError::fromPreTo(
                format!("Invalid char literal: {operator}"),
                tok,
            )));
        }
        let msg = &message[1..message.len() - 1];
        let msg = Self::escapeString(msg).map_err(|err| {
            Some(CompileError::fromPreTo(
                format!("Invalid char literal: {err}"),
                tok,
            ))
        })?;

        if msg.len() != 1 {
            return Err(Some(CompileError::fromPreTo(
                format!("Invalid char literal: {operator}"),
                tok,
            )));
        }
        Ok(FileTokPos::new_meta_c(
            Self::CharacterLiteral(encoding, msg.as_str().chars().next().unwrap()),
            tok,
        ))
        .map(|x| {
            let mut vec = VecDeque::new();
            vec.push_back(x);
            vec
        })
    }

    pub fn parseStringLiteral<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        operator: &str,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        let (encoding, message) = Self::getEncodingPrefix(operator);
        let msg = &message[1..message.len() - 1];
        let msg = Self::escapeString(msg).map_err(|err| {
            Some(CompileError::fromPreTo(
                format!("Invalid string literal: {err}"),
                tok,
            ))
        })?;

        Ok(FileTokPos::new_meta_c(
            Self::StringLiteral(encoding, msg.to_StringRef()),
            tok,
        ))
        .map(|x| {
            let mut vec = VecDeque::new();
            vec.push_back(x);
            vec
        })
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parseRawStringLiteral<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        let (ud, _) = string.char_indices().rev().find(|x| x.1 == '"').unwrap();
        let string = &string[..ud];
        let ud = &string[ud..];
        let (prefix, mut string) = Self::getEncodingPrefix(string);
        while !string.starts_with('(') {
            string = &string[1..string.len() - 1];
        }
        let string = &string[1..string.len() - 1];
        return Ok(FileTokPos::new_meta_c(
            if ud.is_empty() {
                Self::StringLiteral(prefix, string.to_owned().to_StringRef())
            } else {
                Self::UdStringLiteral(
                    prefix,
                    string.to_owned().to_StringRef(),
                    ud.to_owned().to_StringRef(),
                )
            },
            tok,
        ))
        .map(|x| {
            let mut vec = VecDeque::new();
            vec.push_back(x);
            vec
        });
    }

    fn parseHexNumber<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
    ) -> Result<FileTokPos<Self>, Option<CompileMsg>> {
        if let Some((_, string, ub)) = regex_captures!(
            r"^((?:[\da-f]*\.?[\da-f]+|[\da-f]+\.?[\da-f]*)p[-+]?[\d]+[fl]?)([a-z0-9_]*?)?$"i,
            string
        ) {
            Self::parseHexFloat(tok, string, ub)
        } else {
            Self::parseHexInt(tok, string)
        }
    }

    fn parseIntSuffix(mut string: &str, radix: u32) -> (&str, IntegerSuffix, Option<&str>) {
        let mut ud = None;
        if let Some(idx) = string.find(|c: char| {
            let machesChars = match radix {
                16 => c.is_ascii_hexdigit(),
                _ => c.is_ascii_digit(),
            };

            !machesChars && !c.eq_ignore_ascii_case(&'u') && !c.eq_ignore_ascii_case(&'l')
        }) {
            ud = Some(&string[idx..]);
            string = &string[..idx];
        }

        let mut res = IntegerSuffix(None, IntegerSuffixSignedness::Signed);
        loop {
            if string.len() >= 2 {
                let (prefix, suffix) = string.split_at(string.len() - 2);
                if suffix.eq_ignore_ascii_case("ll") {
                    res.0 = Some(IntegerSuffixLength::LongLong);
                    string = prefix;
                    continue;
                }
            }
            if !string.is_empty() {
                let (prefix, suffix) = string.split_at(string.len() - 1);
                if suffix.eq_ignore_ascii_case("l") {
                    res.0 = Some(IntegerSuffixLength::Long);
                    string = prefix;
                    continue;
                } else if suffix.eq_ignore_ascii_case("u") {
                    res.1 = IntegerSuffixSignedness::Unsigned;
                    string = prefix;
                    continue;
                }
            }
            break;
        }
        return (string, res, ud);
    }

    // Supports the ' optional character
    fn parseBaseInt<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
        radix: u32,
    ) -> Result<i128, Option<CompileMsg>> {
        u128::from_str_radix(string, radix)
            .map_err(|x| x.to_string())
            .and_then(|x| i128::try_from(x).map_err(|x| x.to_string()))
            .map_err(|err| {
                Some(CompileError::fromPreTo(
                    format!("Invalid number: {string}, error: {err}"),
                    tok,
                ))
            })
    }

    fn parseHexInt<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
    ) -> Result<FileTokPos<Self>, Option<CompileMsg>> {
        let string = string.replace('\'', "");
        let (prefix, suffix, ud) = Self::parseIntSuffix(&string, 16);
        Self::parseBaseInt(tok, prefix, 16).map(|num| {
            FileTokPos::new_meta_c(
                ud.map_or(Self::IntegerLiteral(num, suffix), |ud| {
                    Self::UdIntegerLiteral(num, suffix, ud.to_owned().to_StringRef())
                }),
                tok,
            )
        })
    }

    fn parseBinInt<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
    ) -> Result<FileTokPos<Self>, Option<CompileMsg>> {
        let string = string.replace('\'', "");
        let (prefix, suffix, ud) = Self::parseIntSuffix(&string, 2);
        Self::parseBaseInt(tok, prefix, 2).map(|num| {
            FileTokPos::new_meta_c(
                ud.map_or(Self::IntegerLiteral(num, suffix), |ud| {
                    Self::UdIntegerLiteral(num, suffix, ud.to_owned().to_StringRef())
                }),
                tok,
            )
        })
    }

    fn parseOctalInt<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
    ) -> Result<FileTokPos<Self>, Option<CompileMsg>> {
        let string = string.replace('\'', "");
        let (prefix, suffix, ud) = Self::parseIntSuffix(&string, 8);
        Self::parseBaseInt(tok, prefix, 8).map(|num| {
            FileTokPos::new_meta_c(
                ud.map_or(Self::IntegerLiteral(num, suffix), |ud| {
                    Self::UdIntegerLiteral(num, suffix, ud.to_owned().to_StringRef())
                }),
                tok,
            )
        })
    }

    fn parseDecimalInt<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
    ) -> Result<FileTokPos<Self>, Option<CompileMsg>> {
        let string = string.replace('\'', "");
        let (prefix, suffix, ud) = Self::parseIntSuffix(&string, 10);
        Self::parseBaseInt(tok, prefix, 10).map(|num| {
            FileTokPos::new_meta_c(
                ud.map_or(Self::IntegerLiteral(num, suffix), |ud| {
                    Self::UdIntegerLiteral(num, suffix, ud.to_owned().to_StringRef())
                }),
                tok,
            )
        })
    }

    fn parseFloatSuffix(string: &str) -> (&str, FloatSuffix) {
        if !string.is_empty() {
            let (prefix, suffix) = string.split_at(string.len() - 1);
            if suffix.eq_ignore_ascii_case("f") {
                return (prefix, FloatSuffix::F);
            } else if suffix.eq_ignore_ascii_case("l") {
                return (prefix, FloatSuffix::L);
            }
        }
        return (string, FloatSuffix::None);
    }

    fn parseHexFloat<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
        ub: &str,
    ) -> Result<FileTokPos<Self>, Option<CompileMsg>> {
        let (prefix, suffix) = Self::parseFloatSuffix(string);
        f128::f128::parse("0x".to_owned() + prefix)
            .map_err(|x| x.to_string())
            .map_err(|err| {
                Some(CompileError::fromPreTo(
                    format!("Invalid number: {string}, error: {err}"),
                    tok,
                ))
            })
            .map(|num| {
                FileTokPos::new_meta_c(
                    if ub.is_empty() {
                        Self::FloatingPointLiteral(num, suffix)
                    } else {
                        Self::UdFloatingPointLiteral(num, suffix, ub.to_owned().to_StringRef())
                    },
                    tok,
                )
            })
    }

    fn parseDecimalFloating<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        mut string: &str,
    ) -> Result<FileTokPos<Self>, Option<CompileMsg>> {
        let mut ub = None;
        if let Some((_, string_, ub_)) = regex_captures!(
            r"^((?:(?:[\d']*\.[\d']+|[\d']+\.)(?:e[+-]?[\d']+)?|[\d']+e[+-]?[\d']+)[fl]?)([a-z0-9_]*?)$"i,
            string
        ) {
            if !ub_.is_empty() {
                string = string_;
                ub = Some(ub_);
            }
        }

        let (prefix, suffix) = Self::parseFloatSuffix(string);
        f128::f128::parse(prefix)
            .map_err(|x| x.to_string())
            .map_err(|err| {
                Some(CompileError::fromPreTo(
                    format!("Invalid number: {string}, error: {err}"),
                    tok,
                ))
            })
            .map(|num| {
                FileTokPos::new_meta_c(
                    ub.map_or(Self::FloatingPointLiteral(num, suffix), |ub| {
                        Self::UdFloatingPointLiteral(num, suffix, ub.to_owned().to_StringRef())
                    }),
                    tok,
                )
            })
    }

    pub fn parsePPNumber<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        string: &str,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        {
            #[allow(clippy::option_if_let_else)]
            if let Some(res) = string.strip_prefix("0x") {
                Self::parseHexNumber(tok, res)
            } else if let Some(res) = string.strip_prefix("0X") {
                Self::parseHexNumber(tok, res)
            } else if let Some(res) = string.strip_prefix("0b") {
                Self::parseBinInt(tok, res)
            } else if let Some(res) = string.strip_prefix("0B") {
                Self::parseBinInt(tok, res)
            } else if string.contains('.') || string.contains('e') || string.contains('E') {
                Self::parseDecimalFloating(tok, string)
            } else if string.starts_with('0') {
                Self::parseOctalInt(tok, string)
            } else {
                Self::parseDecimalInt(tok, string)
            }
        }
        .map(|x| {
            let mut vec = VecDeque::new();
            vec.push_back(x);
            vec
        })
    }

    pub fn parseUdCharLiteral<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        mut operator: &str,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        let ud = operator
            .chars()
            .rev()
            .take_while(|x| *x != '\'')
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        operator = operator[..operator.len() - ud.len()].trim_end();
        Self::parseCharLiteral(tok, operator).map(|x| {
            if let Self::CharacterLiteral(enc, str) = x.front().unwrap().tokPos.tok {
                let tok = FileTokPos::new_meta_c(
                    Self::UdCharacterLiteral(enc, str, ud.to_StringRef()),
                    x.front().unwrap(),
                );
                let mut vec = VecDeque::new();
                vec.push_back(tok);
                vec
            } else {
                unreachable!()
            }
        })
    }

    pub fn parseUdStringLiteral<T: Clone + std::fmt::Debug>(
        tok: &FileTokPos<T>,
        mut operator: &str,
    ) -> Result<VecDeque<FileTokPos<Self>>, Option<CompileMsg>> {
        let ud = operator
            .chars()
            .rev()
            .take_while(|x| *x != '"')
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        operator = operator[..operator.len() - ud.len()].trim_end();
        Self::parseStringLiteral(tok, operator).map(|x| {
            if let Self::StringLiteral(enc, str) = &x.front().unwrap().tokPos.tok {
                let tok = FileTokPos::new_meta_c(
                    Self::UdStringLiteral(*enc, *str, ud.to_StringRef()),
                    x.front().unwrap(),
                );
                let mut vec = VecDeque::new();
                vec.push_back(tok);
                vec
            } else {
                unreachable!()
            }
        })
    }
}
