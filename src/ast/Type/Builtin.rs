use std::fmt::Display;

use deriveMacros::{CommonAst, RustycppInheritanceConstructors};
use strum_macros::EnumIter;

use crate::ast::common::AstTypeBuiltin;
use crate::ast::{
    common::{AstTypeBuiltinStructNode, AstTypeStructNode},
    Type::{BaseType, TypeAst},
};

#[derive(EnumIter, Copy, Clone)]
pub enum BuiltinTypeKind {
    Void,
    VoidPtr,
    Bool,
    Char,
    SChar,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    LongLong,
    ULongLong,
    Float,
    Double,
    LongDouble,
    WChar,
    Char16,
    Char32,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Int128,
    UInt128,
    NullPtr,
    Auto,
    DecltypeAuto,
    Unknown,
}

impl Display for BuiltinTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::VoidPtr => write!(f, "void*"),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),
            Self::SChar => write!(f, "signed char"),
            Self::UChar => write!(f, "unsigned char"),
            Self::Short => write!(f, "short"),
            Self::UShort => write!(f, "unsigned short"),
            Self::Int => write!(f, "int"),
            Self::UInt => write!(f, "unsigned int"),
            Self::Long => write!(f, "long"),
            Self::ULong => write!(f, "unsigned long"),
            Self::LongLong => write!(f, "long long"),
            Self::ULongLong => write!(f, "unsigned long long"),
            Self::Float => write!(f, "float"),
            Self::Double => write!(f, "double"),
            Self::LongDouble => write!(f, "long double"),
            Self::WChar => write!(f, "wchar_t"),
            Self::Char16 => write!(f, "char16_t"),
            Self::Char32 => write!(f, "char32_t"),
            Self::Int8 => write!(f, "int8_t"),
            Self::UInt8 => write!(f, "uint8_t"),
            Self::Int16 => write!(f, "int16_t"),
            Self::UInt16 => write!(f, "uint16_t"),
            Self::Int32 => write!(f, "int32_t"),
            Self::UInt32 => write!(f, "uint32_t"),
            Self::Int64 => write!(f, "int64_t"),
            Self::UInt64 => write!(f, "uint64_t"),
            Self::Int128 => write!(f, "int128_t"),
            Self::UInt128 => write!(f, "uint128_t"),
            Self::NullPtr => write!(f, "nullptr_t"),
            Self::Auto => write!(f, "auto"),
            Self::DecltypeAuto => write!(f, "decltype(auto)"),
            Self::Unknown => write!(f, "__unknown_type"),
        }
    }
}

#[derive(CommonAst)]
pub struct AstTypeBuiltinStruct {
    #[AstToString]
    kindType: BuiltinTypeKind,
}

impl Display for AstTypeBuiltinStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kindType)
    }
}

impl AstTypeBuiltinStruct {
    pub const fn new(kindType: BuiltinTypeKind) -> Self {
        Self { kindType }
    }
}

#[RustycppInheritanceConstructors]
impl AstTypeBuiltinStructNode {
    pub fn new(kindType: BuiltinTypeKind) -> Self {
        Self {
            base: AstTypeBuiltinStruct::new(kindType),
            parent: AstTypeStructNode::new(),
        }
    }
}

impl TypeAst for &AstTypeBuiltinStructNode {
    fn getBaseType(&self) -> BaseType {
        #[allow(clippy::match_same_arms)]
        let (size, align) = match self.base.kindType {
            BuiltinTypeKind::Void => (0, 0),
            BuiltinTypeKind::VoidPtr => (8, 8),
            BuiltinTypeKind::Bool => (1, 1),
            BuiltinTypeKind::Char => (1, 1),
            BuiltinTypeKind::SChar => (1, 1),
            BuiltinTypeKind::UChar => (1, 1),
            BuiltinTypeKind::Short => (2, 2),
            BuiltinTypeKind::UShort => (2, 2),
            BuiltinTypeKind::Int => (4, 4),
            BuiltinTypeKind::UInt => (4, 4),
            BuiltinTypeKind::Long => (8, 8),
            BuiltinTypeKind::ULong => (8, 8),
            BuiltinTypeKind::LongLong => (8, 8),
            BuiltinTypeKind::ULongLong => (8, 8),
            BuiltinTypeKind::Float => (4, 4),
            BuiltinTypeKind::Double => (8, 8),
            BuiltinTypeKind::LongDouble => (16, 16),
            BuiltinTypeKind::WChar => (4, 4),
            BuiltinTypeKind::Char16 => (2, 2),
            BuiltinTypeKind::Char32 => (4, 4),
            BuiltinTypeKind::Int8 => (1, 1),
            BuiltinTypeKind::UInt8 => (1, 1),
            BuiltinTypeKind::Int16 => (2, 2),
            BuiltinTypeKind::UInt16 => (2, 2),
            BuiltinTypeKind::Int32 => (4, 4),
            BuiltinTypeKind::UInt32 => (4, 4),
            BuiltinTypeKind::Int64 => (8, 8),
            BuiltinTypeKind::UInt64 => (8, 8),
            BuiltinTypeKind::Int128 => (16, 16),
            BuiltinTypeKind::UInt128 => (16, 16),
            BuiltinTypeKind::NullPtr => (8, 8),
            BuiltinTypeKind::Auto => (0, 0),
            BuiltinTypeKind::DecltypeAuto => (0, 0),
            BuiltinTypeKind::Unknown => (0, 0),
        };
        BaseType { size, align }
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)
    }
}
