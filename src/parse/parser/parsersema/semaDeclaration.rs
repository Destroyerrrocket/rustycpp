use crate::{
    ast::{
        common::AstDecl,
        Attribute::AstAttribute,
        Decl::{Asm::AstAsmDecl, Empty::AstEmptyDecl, Namespace::AstNamespaceDecl},
    },
    sema::scope::{Child, RefCellScope, Scope, ScopeKind},
    utils::{stringref::StringRef, structs::SourceRange},
};

use super::super::Parser;

impl Parser {
    /**
     * empty-declaration | attribute-declaration
     */
    pub fn actOnEmptyDecl(
        &mut self,
        attr: &Vec<&'static AstAttribute>,
        location: SourceRange,
    ) -> Vec<&'static AstDecl> {
        let ast = AstEmptyDecl::new(location, self.alloc().alloc_slice_clone(attr.as_slice()));
        return vec![self.alloc().alloc(AstDecl::AstEmptyDecl(ast))];
    }

    /**
     * asm-declaration
     */
    pub fn actOnAsmDecl(
        &mut self,
        attr: &Vec<&'static AstAttribute>,
        location: SourceRange,
        asm: StringRef,
    ) -> Vec<&'static AstDecl> {
        let astAsm = AstAsmDecl::new(
            location,
            self.alloc().alloc_slice_clone(attr.as_slice()),
            asm,
        );
        return vec![self.alloc().alloc(AstDecl::AstAsmDecl(astAsm))];
    }

    /**
     * named-namespace-definition
     */
    pub fn actOnStartNamedNamespaceDefinition(
        &mut self,
        isInline: bool,
        attr: &Vec<&'static AstAttribute>,
        name: StringRef,
        locationName: SourceRange,
    ) -> Vec<&'static AstDecl> {
        let astNamespace = AstNamespaceDecl::new(
            locationName,
            self.alloc().alloc_slice_clone(attr.as_slice()),
            name,
            isInline,
        );

        let astNamespaceDecl = self.alloc().alloc(AstDecl::AstNamespaceDecl(astNamespace));

        let enumScope = Scope::new(ScopeKind::NAMESPACE | ScopeKind::CAN_DECL, astNamespaceDecl);
        self.currentScope
            .addChild(name, Child::Scope(enumScope.clone()));
        self.currentScope = enumScope;

        return vec![astNamespaceDecl];
    }

    /**
     * named-namespace-definition
     */
    pub fn actOnEndNamedNamespaceDefinition(&mut self, contents: Vec<&'static AstDecl>) {
        let Some(AstDecl::AstNamespaceDecl(namespaceDecl)) =
            self.currentScope.borrow().causingDecl else {unreachable!();};

        let contents = self.alloc().alloc_slice_copy(contents.as_slice());
        namespaceDecl.setContents(contents);

        let newCurrent = self.currentScope.borrow().parent.clone().unwrap();
        self.currentScope = newCurrent;
    }
}
