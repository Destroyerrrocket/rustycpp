use std::{fmt::Display, rc::Rc};

use bitflags::bitflags;
use bumpalo::Bump;
use deriveMacros::CommonAst;
use enum_dispatch::enum_dispatch;

use crate::{
    ast::Type::Builtin::{BuiltinType, BuiltinTypeKind},
    utils::unsafeallocator::UnsafeAllocator,
};

pub mod Builtin;

pub struct BaseType {
    pub size: u64,
    pub align: u64,
}

impl BaseType {
    pub const fn new(size: u64, align: u64) -> Self {
        Self { size, align }
    }
}

#[enum_dispatch]
pub trait TypeAst: Display {
    fn getBaseType(&self) -> &BaseType;
}

#[allow(clippy::enum_variant_names)]
#[derive(CommonAst)]
#[enum_dispatch(TypeAst)]
pub enum AstType {
    BuiltinType,
}

impl Display for AstType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuiltinType(t) => t.fmt(f),
        }
    }
}

bitflags! {
    pub struct QualTypeFlags: u8 {
        const CONST = 1;
        const VOLATILE = 2;
        const RESTRICT = 4;
    }
}

impl Display for QualTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.contains(Self::CONST) {
            write!(f, "const ")?;
        }
        if self.contains(Self::VOLATILE) {
            write!(f, "volatile ")?;
        }
        if self.contains(Self::RESTRICT) {
            write!(f, "restrict ")?;
        }
        Ok(())
    }
}

#[derive(CommonAst)]
struct QualType {
    #[AstChild]
    unqualType: &'static AstType,
    #[AstToString]
    flags: QualTypeFlags,
}

impl TypeAst for QualType {
    fn getBaseType(&self) -> &BaseType {
        self.unqualType.getBaseType()
    }
}

impl Display for QualType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.flags, self.unqualType)
    }
}

impl QualType {
    pub const fn new(unqualType: &'static AstType, flags: QualTypeFlags) -> Self {
        Self { unqualType, flags }
    }

    pub const fn getUnqualType(&self) -> &'static AstType {
        self.unqualType
    }

    pub const fn getFlags(&self) -> QualTypeFlags {
        self.flags
    }

    pub fn setFlags(&mut self, flags: QualTypeFlags) {
        self.flags = flags;
    }

    pub fn addFlags(&mut self, flags: QualTypeFlags) {
        self.flags |= flags;
    }

    pub fn removeFlags(&mut self, flags: QualTypeFlags) {
        self.flags &= !flags;
    }

    pub const fn isConst(&self) -> bool {
        self.flags.contains(QualTypeFlags::CONST)
    }

    pub const fn isVolatile(&self) -> bool {
        self.flags.contains(QualTypeFlags::VOLATILE)
    }

    pub const fn isRestrict(&self) -> bool {
        self.flags.contains(QualTypeFlags::RESTRICT)
    }

    pub fn isConstVolatile(&self) -> bool {
        self.flags
            .contains(QualTypeFlags::CONST | QualTypeFlags::VOLATILE)
    }

    pub fn isConstRestrict(&self) -> bool {
        self.flags
            .contains(QualTypeFlags::CONST | QualTypeFlags::RESTRICT)
    }

    pub fn isVolatileRestrict(&self) -> bool {
        self.flags
            .contains(QualTypeFlags::VOLATILE | QualTypeFlags::RESTRICT)
    }

    pub fn isConstVolatileRestrict(&self) -> bool {
        self.flags
            .contains(QualTypeFlags::CONST | QualTypeFlags::VOLATILE | QualTypeFlags::RESTRICT)
    }

    pub fn setConst(&mut self) {
        self.flags |= QualTypeFlags::CONST;
    }

    pub fn setVolatile(&mut self) {
        self.flags |= QualTypeFlags::VOLATILE;
    }

    pub fn setRestrict(&mut self) {
        self.flags |= QualTypeFlags::RESTRICT;
    }
}

#[derive(Clone)]
pub struct TypeDict {
    builtinTypes: Vec<&'static AstType>,
    alloc: Rc<UnsafeAllocator>,
}

impl TypeDict {
    pub fn new(alloc: Rc<UnsafeAllocator>) -> Self {
        Self {
            builtinTypes: Vec::new(),
            alloc,
        }
    }

    fn alloc(&self) -> &'static Bump {
        self.alloc.alloc()
    }

    pub fn addBuiltinType(&mut self, t: BuiltinTypeKind) {
        assert!(t as usize == self.builtinTypes.len());
        let ty = self
            .alloc()
            .alloc(AstType::BuiltinType(BuiltinType::new(t)));
        self.builtinTypes.push(ty);
    }

    pub fn getBuiltinType(&self, t: BuiltinTypeKind) -> &'static AstType {
        self.builtinTypes[t as usize]
    }
}
