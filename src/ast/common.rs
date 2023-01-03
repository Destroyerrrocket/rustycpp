use bitflags::bitflags;
use deriveMacros::CommonAst;
use enum_dispatch::enum_dispatch;

use crate::utils::{debugnode::DebugNode, structs::SourceRange};

use crate::ast::Decl::Empty::AstEmptyDecl;

pub trait CommonAst {
    fn getDebugNode(&self) -> DebugNode;
}

bitflags! {
    pub struct MyFlags: u8 {
        const INVALID_DECL    = 0b1;
        const USED           = 0b10;
        const REFERENCED     = 0b100;
    }
}

pub struct BaseDecl {
    pub sourceRange: SourceRange,
    pub flags: MyFlags,
}

impl BaseDecl {
    pub fn new(sourceRange: SourceRange) -> Self {
        Self {
            sourceRange,
            flags: MyFlags::empty(),
        }
    }
}

#[enum_dispatch]
pub trait DeclAst<T: CommonAst = Self> {
    fn getBaseDecl(&self) -> &BaseDecl;
}

#[derive(CommonAst)]
#[enum_dispatch(DeclAst)]
pub enum AstDecl {
    AstEmptyDecl,
}
