use bitflags::bitflags;
use deriveMacros::CommonAst;
use enum_dispatch::enum_dispatch;

use crate::utils::structs::SourceRange;
use crate::{
    ast::{
        Attribute::AstAttribute,
        Decl::{
            Asm::AstAsmDecl, Empty::AstEmptyDecl, Enum::AstCustomRustyCppEnum,
            Namespace::AstNamespaceDecl, UsingNamespace::AstUsingNamespaceDecl,
        },
    },
    sema::scope::ScopeRef,
};

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

pub struct BaseDecl {
    pub sourceRange: SourceRange,
    pub scope: ScopeRef,
    pub flags: MyFlags,
}

impl BaseDecl {
    pub fn new(sourceRange: SourceRange, scope: ScopeRef) -> Self {
        Self {
            sourceRange,
            scope,
            flags: MyFlags::empty(),
        }
    }
}

#[enum_dispatch]
pub trait DeclAst {
    fn getBaseDecl(&self) -> &BaseDecl;
    fn getAttributes(&self) -> Option<&'static [&'static AstAttribute]>;
}

#[allow(clippy::enum_variant_names)]
#[derive(CommonAst)]
#[enum_dispatch(DeclAst)]
pub enum AstDecl {
    AstEmptyDecl,
    AstAsmDecl,
    AstNamespaceDecl,
    AstCustomRustyCppEnum,
    AstUsingNamespaceDecl,
}

pub struct DeclStruct;
