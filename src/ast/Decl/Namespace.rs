use std::cell::UnsafeCell;

use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{
        common::{AstDecl, BaseDecl},
        Attribute::AstAttribute,
    },
    utils::{stringref::StringRef, structs::SourceRange},
};

#[derive(CommonAst, DeclAst)]
pub struct AstNamespaceDecl {
    base: BaseDecl,
    #[AstToString]
    name: StringRef,
    #[AstToString]
    isInline: bool,
    nextExtension: UnsafeCell<Option<&'static AstNamespaceDecl>>,
    #[AstChildSlice]
    attrs: &'static [&'static AstAttribute],
    #[AstChildSliceCell]
    contents: UnsafeCell<&'static [&'static AstDecl]>,
}

impl AstNamespaceDecl {
    pub fn new(
        sourceRange: SourceRange,
        attrs: &'static [&'static AstAttribute],
        name: StringRef,
        isInline: bool,
    ) -> Self {
        Self {
            base: BaseDecl::new(sourceRange),
            name,
            isInline,
            nextExtension: UnsafeCell::new(None),
            attrs,
            contents: UnsafeCell::default(),
        }
    }

    pub fn addExtension(&self, extension: &'static Self) {
        // This is like a single-linked-list, basically.
        let mut next = unsafe { self.nextExtension.get().as_mut().unwrap() };
        while next.is_some() {
            next = unsafe { next.unwrap().nextExtension.get().as_mut().unwrap() };
        }
        *next = Some(extension);
    }

    pub fn setContents(&self, newContents: &'static [&'static AstDecl]) {
        let contents = unsafe { self.contents.get().as_mut().unwrap() };
        *contents = newContents;
    }
}
