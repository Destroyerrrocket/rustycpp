use crate::{
    ast::{
        Attribute::{self, AstAttribute, CXXAttribute},
        Decl::{
            Asm::AstAsmDecl, AstDecl, Empty::AstEmptyDecl, Enum::AstCustomRustyCppEnum,
            Namespace::AstNamespaceDecl, UsingNamespace::AstUsingNamespaceDecl,
        },
        NestedNameSpecifier::AstNestedNameSpecifier,
    },
    sema::scope::{Child, RefCellScope, Scope, ScopeKind, ScopeRef},
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
        attr: &[&'static AstAttribute],
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
        let ast = AstEmptyDecl::new(
            location,
            self.currentScope.clone(),
            self.alloc().alloc_slice_copy(attr),
        );
        vec![self.alloc().alloc(AstDecl::AstEmptyDecl(ast))]
    }

    /**
     * asm-declaration
     */
    pub fn actOnAsmDecl(
        &mut self,
        attr: &[&'static AstAttribute],
        location: SourceRange,
        asm: StringRef,
    ) -> Vec<&'static AstDecl> {
        let astAsm = AstAsmDecl::new(
            location,
            self.currentScope.clone(),
            self.alloc().alloc_slice_copy(attr),
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
        attr: &[&'static AstAttribute],
        name: StringRef,
        locationName: SourceRange,
    ) -> Vec<&'static AstDecl> {
        let createNamespace = |parser: &mut Self, scope: ScopeRef| {
            let astNamespace = AstNamespaceDecl::new(
                locationName,
                scope,
                parser.alloc().alloc_slice_copy(attr),
                name,
                isInline,
                parser.currentScope.clone(),
            );

            return parser
                .alloc()
                .alloc(AstDecl::AstNamespaceDecl(astNamespace));
        };
        let possibleOriginalDecl = self.namespaceExtendableLookup(name);

        if let Some(originalDecl) = possibleOriginalDecl {
            let AstDecl::AstNamespaceDecl(causingDecl) = originalDecl.borrow().causingDecl.unwrap() else {unreachable!();};
            if isInline && !causingDecl.isInline() {
                self.errors.push(CompileError::fromSourceRange(
                    "Namespace redefinition with \"inline\", while original did not have it.",
                    &locationName,
                ));
            }
            let astNamespaceDecl = createNamespace(self, originalDecl.clone());
            causingDecl.addExtension(astNamespaceDecl);
            self.currentScope = originalDecl.clone();
            return vec![astNamespaceDecl];
        }
        let enumScope = Scope::new(ScopeKind::NAMESPACE | ScopeKind::CAN_DECL);
        let astNamespaceDecl = createNamespace(self, enumScope.clone());
        enumScope.setCausingDecl(astNamespaceDecl);

        if isInline {
            self.currentScope.addInlinedChild(name, enumScope.clone());
        } else {
            self.currentScope
                .addChild(name, Child::Scope(enumScope.clone()));
        }
        self.currentScope = enumScope;
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
        attr: &[&'static AstAttribute],
    ) -> Vec<&'static AstDecl> {
        let attrs = self.alloc().alloc_slice_copy(attr);

        let enumScope = Scope::new(ScopeKind::ENUM | ScopeKind::CAN_DECL);

        let astEnum = AstCustomRustyCppEnum::new(location, enumScope.clone(), name, attrs);
        let astEnumDecl = self.alloc().alloc(AstDecl::AstCustomRustyCppEnum(astEnum));
        enumScope.setCausingDecl(astEnumDecl);

        self.currentScope
            .addChild(name, Child::Scope(enumScope.clone()));
        self.currentScope = enumScope;

        // Imediately pop, for now.
        let newCurrent = self.currentScope.borrow().parent.clone().unwrap();
        self.currentScope = newCurrent;
        vec![astEnumDecl]
    }

    pub fn actOnUsingNamespaceDefinition(
        &mut self,
        name: StringRef,
        location: SourceRange,
        attr: &[&'static AstAttribute],
        nestedNameSpecifier: &'static [AstNestedNameSpecifier],
        scope: Option<&ScopeRef>,
    ) -> Vec<&'static AstDecl> {
        let onlyNamespacesFunc = |child: &Child| match child {
            Child::Decl(_) => false,
            Child::Scope(scope) => scope.borrow().flags.contains(ScopeKind::NAMESPACE),
        };

        let result = if let Some(scope) = scope {
            let result = Self::qualifiedNameLookupWithCond(name, scope, onlyNamespacesFunc);
            let Some(Child::Scope(result)) = result.first() else {
                    self.errors.push(CompileError::fromSourceRange(
                    "We were unable to resolve this name. Something may be wrong with the nested name specifier",
                    &location,
                    ));
                    return vec![];
                };
            result.clone()
        } else {
            let Some(Child::Scope(result)) = self.unqualifiedNameLookupWithCond(name, onlyNamespacesFunc).first().cloned() else {
                    self.errors.push(CompileError::fromSourceRange(
                        "We were unable to resolve this name",
                        &location,
                    ));
                    return vec![];
                };
            result
        };
        self.currentScope.addUsingNamespace(result.clone());
        let attr = self.alloc().alloc_slice_copy(attr);
        vec![self
            .alloc()
            .alloc(AstDecl::AstUsingNamespaceDecl(AstUsingNamespaceDecl::new(
                location,
                result,
                name,
                attr,
                nestedNameSpecifier,
            )))]
    }
}
