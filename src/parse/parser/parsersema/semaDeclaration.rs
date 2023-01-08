use crate::{
    ast::{
        Attribute::{self, AstAttribute, CXXAttribute},
        Decl::{
            Asm::AstAsmDecl, AstDecl, Empty::AstEmptyDecl, Enum::AstCustomRustyCppEnum,
            Namespace::AstNamespaceDecl,
        },
    },
    sema::scope::{Child, RefCellScope, Scope, ScopeKind},
    utils::{
        stringref::StringRef,
        structs::{CompileError, CompileMsgImpl, SourceRange},
    },
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
        for a in attr {
            if let Attribute::Kind::Cxx(attrmembers) = a.kind {
                for attrmember in attrmembers {
                    (*attrmember).actOnAttributeDecl(self);
                }
            } else {
                self.errors.push(CompileError::fromSourceRange(
                    "Only Cxx11 attributes are allowed here.",
                    &a.sourceRange,
                ));
                continue;
            }
        }
        let ast = AstEmptyDecl::new(location, self.alloc().alloc_slice_copy(attr.as_slice()));
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
            self.alloc().alloc_slice_copy(attr.as_slice()),
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
            self.alloc().alloc_slice_copy(attr.as_slice()),
            name,
            isInline,
        );

        let astNamespaceDecl = self.alloc().alloc(AstDecl::AstNamespaceDecl(astNamespace));

        let possibleOriginalDecl = {
            let currentScope = self.currentScope.borrow();
            currentScope.childs.get(&name).and_then(|originalDecl| {
                originalDecl
                    .iter()
                    .find(|scope| {
                        let Child::Scope(scope) = scope else {return false;};
                        scope.borrow().flags == ScopeKind::NAMESPACE | ScopeKind::CAN_DECL
                    })
                    .map(|scope| {
                        let Child::Scope(scope) = scope else {unreachable!();};
                        scope.clone()
                    })
            })
        };

        if let Some(originalDecl) = possibleOriginalDecl {
            let AstDecl::AstNamespaceDecl(causingDecl) = originalDecl.borrow().causingDecl.unwrap() else {unreachable!();};
            causingDecl.addExtension(astNamespaceDecl);
            self.currentScope = originalDecl.clone();
        } else {
            let enumScope =
                Scope::new(ScopeKind::NAMESPACE | ScopeKind::CAN_DECL, astNamespaceDecl);
            self.currentScope
                .addChild(name, Child::Scope(enumScope.clone()));
            self.currentScope = enumScope;
        }
        vec![astNamespaceDecl]
    }

    /**
     * named-namespace-definition
     */
    pub fn actOnEndNamedNamespaceDefinition(
        &mut self,
        namespaceDecl: &'static AstDecl,
        contents: &[&'static AstDecl],
    ) {
        let AstDecl::AstNamespaceDecl(namespaceDecl) =
            namespaceDecl else {unreachable!();};

        let contents = self.alloc().alloc_slice_copy(contents);
        namespaceDecl.setContents(contents);

        let newCurrent = self.currentScope.borrow().parent.clone().unwrap();
        self.currentScope = newCurrent;
    }

    pub fn actOnRustyCppEnumDefinition(
        &mut self,
        name: StringRef,
        location: SourceRange,
        attr: &Vec<&'static AstAttribute>,
    ) -> Vec<&'static AstDecl> {
        let attrs = self.alloc().alloc_slice_copy(attr.as_slice());
        let astEnum = AstCustomRustyCppEnum::new(location, name, attrs);
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
