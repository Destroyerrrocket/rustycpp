use crate::{
    ast::{
        Attribute::AstAttribute,
        Decl::{
            Asm::AstAsmDecl, AstDecl, Empty::AstEmptyDecl, Enum::AstCustomRustyCppEnum,
            Namespace::AstNamespaceDecl,
        },
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
        vec![self.alloc().alloc(AstDecl::AstEmptyDecl(ast))]
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
        vec![self.alloc().alloc(AstDecl::AstAsmDecl(astAsm))]
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

        vec![astNamespaceDecl]
    }

    /**
     * named-namespace-definition
     */
    pub fn actOnEndNamedNamespaceDefinition(&mut self, contents: &[&'static AstDecl]) {
        let Some(AstDecl::AstNamespaceDecl(namespaceDecl)) =
            self.currentScope.borrow().causingDecl else {unreachable!();};

        let contents = self.alloc().alloc_slice_copy(contents);
        namespaceDecl.setContents(contents);

        let newCurrent = self.currentScope.borrow().parent.clone().unwrap();
        self.currentScope = newCurrent;
    }

    pub fn actOnRustyCppEnumDefinition(
        &mut self,
        name: StringRef,
        location: SourceRange,
    ) -> Vec<&'static AstDecl> {
        let astEnum = AstCustomRustyCppEnum::new(location, name);
        let astEnumDecl = self.alloc().alloc(AstDecl::AstCustomRustyCppEnum(astEnum));

        let enumScope = Scope::new(ScopeKind::ENUM | ScopeKind::CAN_DECL, astEnumDecl);
        self.currentScope
            .addChild(name, Child::Scope(enumScope.clone()));
        self.currentScope = enumScope;

        // Imediately pop, for now.
        let newCurrent = self.currentScope.borrow().parent.clone().unwrap();
        self.currentScope = newCurrent;
        vec![astEnumDecl]
    }
}
