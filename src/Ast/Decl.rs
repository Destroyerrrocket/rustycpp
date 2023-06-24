use std::fmt::Display;

use crate::{
    Ast::Common::{AstAttribute, AstDeclStructNode},
    Parent,
};
use bitflags::bitflags;
use deriveMacros::CommonAst;

use crate::Sema::Scope::ScopeRef;
use crate::Utils::Structs::SourceRange;

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
        let written = if self.contains(Self::INVALID_DECL) {
            write!(f, "INVALID_DECL")?;
            true
        } else {
            false
        };

        let written = if self.contains(Self::USED) {
            if written {
                write!(f, " | ")?;
            }
            write!(f, "USED")?;
            true
        } else {
            written
        };
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

    pub const fn getAttributes(&self) -> &'static [AstAttribute] {
        self.base.attrs
    }

    pub fn getScope(&self) -> ScopeRef {
        self.base.scope.clone()
    }

    pub const fn getSourceRange(&self) -> SourceRange {
        self.base.sourceRange
    }

    pub const fn getFlags(&self) -> MyFlags {
        self.base.flags
    }
}
