use std::fmt::Display;

use crate::{
    ast::common::{AstAttribute, AstDeclStructNode},
    Parent,
};
use bitflags::bitflags;
use deriveMacros::CommonAst;

use crate::sema::scope::ScopeRef;
use crate::utils::structs::SourceRange;

pub mod Asm;
pub mod Empty;
pub mod Enum;
pub mod Namespace;
pub mod UsingNamespace;

bitflags! {
    pub struct MyFlags: u8 {
        const INVALID_DECL    = 0b1;
        const USED           = 0b10;
        const REFERENCED     = 0b100;
    }
}

impl Display for MyFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut written = false;
        if self.contains(Self::INVALID_DECL) {
            write!(f, "INVALID_DECL")?;
            written = true;
        }
        if self.contains(Self::USED) {
            if written {
                write!(f, " | ")?;
            }
            write!(f, "USED")?;
        }
        if self.contains(Self::REFERENCED) {
            if written {
                write!(f, " | ")?;
            }
            write!(f, "REFERENCED")?;
        }
        Ok(())
    }
}

#[derive(CommonAst)]
pub struct AstDeclStruct {
    pub sourceRange: SourceRange,
    pub scope: ScopeRef,
    #[AstToString]
    pub flags: MyFlags,
    #[AstChildSlice]
    pub attrs: &'static [AstAttribute],
}

impl AstDeclStruct {
    pub fn new(sourceRange: SourceRange, scope: ScopeRef, attrs: &'static [AstAttribute]) -> Self {
        Self {
            sourceRange,
            scope,
            flags: MyFlags::empty(),
            attrs,
        }
    }
}

impl AstDeclStructNode {
    pub fn new(sourceRange: SourceRange, scope: ScopeRef, attrs: &'static [AstAttribute]) -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: AstDeclStruct::new(sourceRange, scope, attrs),
        }
    }

    pub fn getAttributes(&self) -> &'static [AstAttribute] {
        self.base.attrs
    }

    pub fn getScope(&self) -> ScopeRef {
        self.base.scope.clone()
    }

    pub fn getSourceRange(&self) -> SourceRange {
        self.base.sourceRange
    }

    pub fn getFlags(&self) -> MyFlags {
        self.base.flags
    }
}
