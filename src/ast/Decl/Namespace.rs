use std::{cell::RefCell};

use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{
        Attribute::AstAttribute,
        Decl::{AstDecl, BaseDecl},
    },
    sema::scope::{ ScopeRef},
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
    /**
     * This is always the parent scope of where this was declared. This is
     * specially useful when dealing with namespaces that extend other
     * namespaces.
     * For example:
     * inline namespace Special {
     *    namespace ExtendMe {/*Stuff*/}
     * }
     * namespace ExtendMe {/*Stuff*/}
     * In the second declaration, parent scope will be the root scope,
     * despite the fact that this namespace semantically has the "Special"
     * namespace as its parent.
     */
    parentScope: ScopeRef,
}

impl AstNamespaceDecl {
    pub fn new(
        sourceRange: SourceRange,
        myScope: ScopeRef,
        attrs: &'static [&'static AstAttribute],
        name: StringRef,
        isInline: bool,
        parentScope: ScopeRef,
    ) -> Self {
        Self {
            base: BaseDecl::new(sourceRange, myScope),
            name,
            isInline,
            nextExtension: RefCell::new(None),
            attrs,
            contents: RefCell::default(),
            parentScope,
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

    pub fn isInline(&self) -> bool {
        self.isInline
    }

    pub fn parentScope(&self) -> &ScopeRef {
        &self.parentScope
    }

    pub fn scope(&self) -> &ScopeRef {
        &self.base.scope
    }
}
