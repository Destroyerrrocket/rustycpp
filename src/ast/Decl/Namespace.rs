use crate::Base;
use crate::Parent;
use deriveMacros::CommonAst;
use deriveMacros::RustycppInheritanceConstructors;
use std::cell::RefCell;

use crate::{
    ast::common::*,
    sema::scope::ScopeRef,
    utils::{stringref::StringRef, structs::SourceRange},
};

#[derive(CommonAst)]
pub struct AstDeclNamespaceStruct {
    #[AstToString]
    name: StringRef,
    #[AstToString]
    isInline: bool,
    nextExtension: RefCell<Option<&'static AstDeclNamespaceStructNode>>,
    #[AstChildSliceCell]
    contents: RefCell<&'static [AstDecl]>,
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

impl AstDeclNamespaceStruct {
    pub fn new(name: StringRef, isInline: bool, parentScope: ScopeRef) -> Self {
        Self {
            name,
            isInline,
            nextExtension: RefCell::new(None),
            contents: RefCell::default(),
            parentScope,
        }
    }

    pub fn addExtension(&self, extension: &'static AstDeclNamespaceStructNode) {
        // This is like a single-linked-list, basically.
        let mut next = self.nextExtension.borrow_mut();
        while next.is_some() {
            next = next.unwrap().base.nextExtension.borrow_mut();
        }

        *next = Some(extension);
    }

    pub fn setContents(&self, newContents: &'static [AstDecl]) {
        let mut contents = self.contents.borrow_mut();
        *contents = newContents;
    }

    pub const fn isInline(&self) -> bool {
        self.isInline
    }

    pub const fn parentScope(&self) -> &ScopeRef {
        &self.parentScope
    }
}

#[RustycppInheritanceConstructors]
impl AstDeclNamespaceStructNode {
    pub fn new(
        sourceRange: SourceRange,
        scope: ScopeRef,
        attrs: &'static [AstAttribute],
        name: StringRef,
        isInline: bool,
        parentScope: ScopeRef,
    ) -> Self {
        Self {
            parent: <Parent!()>::new(sourceRange, scope, attrs),
            base: <Base!()>::new(name, isInline, parentScope),
        }
    }

    pub fn addExtension(&self, extension: &AstDeclNamespace) {
        self.base.addExtension(extension.getStatic());
    }

    pub fn setContents(&self, newContents: &'static [AstDecl]) {
        self.base.setContents(newContents);
    }

    pub const fn isInline(&self) -> bool {
        self.base.isInline()
    }

    pub const fn parentScope(&self) -> &ScopeRef {
        self.base.parentScope()
    }
}
