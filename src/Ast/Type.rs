use crate::{
    Ast::Common::{AstType, AstTypeStructNode},
    Parent,
    Utils::FoldingContainer::{FoldingNode, PushFoldingNode},
};
use std::fmt::Display;

use bitflags::bitflags;
use deriveMacros::CommonAst;
use enum_dispatch::enum_dispatch;

pub mod Builtin;
pub mod Pointer;
pub mod Reference;

#[derive(Clone, Copy, CommonAst)]
pub struct AstTypeStruct;

impl AstTypeStructNode {
    pub fn new() -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: AstTypeStruct,
        }
    }
}

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
pub trait TypeAst {
    fn getBaseType(&self) -> BaseType;
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
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
pub struct QualType {
    #[AstChild]
    unqualType: AstType,
    #[AstToString]
    flags: QualTypeFlags,
}

impl TypeAst for QualType {
    fn getBaseType(&self) -> BaseType {
        self.unqualType.getBaseType()
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.flags, self.unqualType)
    }
}

impl Display for QualType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.flags, self.unqualType)
    }
}

impl QualType {
    pub const fn new(unqualType: AstType, flags: QualTypeFlags) -> Self {
        Self { unqualType, flags }
    }

    pub const fn getUnqualType(&self) -> AstType {
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

impl crate::Utils::FoldingContainer::Foldable for QualType {
    fn foldNode(&self, node: &mut FoldingNode) {
        node.push(&self.flags.bits);
        self.unqualType.foldNode(node);
    }
}
