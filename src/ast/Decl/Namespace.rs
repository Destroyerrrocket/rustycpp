use std::cell::RefCell;

use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{
        Attribute::AstAttribute,
        Decl::{AstDecl, BaseDecl},
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
    nextExtension: RefCell<Option<&'static AstNamespaceDecl>>,
    #[DeclAttributes]
    #[AstChildSlice]
    attrs: &'static [&'static AstAttribute],
    #[AstChildSliceCell]
    contents: RefCell<&'static [&'static AstDecl]>,
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
            nextExtension: RefCell::new(None),
            attrs,
            contents: RefCell::default(),
        }
    }

    pub fn addExtension(&self, extension: &'static AstDecl) {
        // This is like a single-linked-list, basically.
        let mut next = self.nextExtension.borrow_mut();
        while next.is_some() {
            next = next.unwrap().nextExtension.borrow_mut();
        }
        let AstDecl::AstNamespaceDecl(extension) = extension else {
            panic!("Expected an AstNamespaceDecl");
        };

        *next = Some(extension);
    }

    pub fn setContents(&self, newContents: &'static [&'static AstDecl]) {
        let mut contents = self.contents.borrow_mut();
        *contents = newContents;
    }
}
