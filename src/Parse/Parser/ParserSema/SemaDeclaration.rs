use crate::Ast::{
    Attribute,
    Common::{
        AstAttribute, AstDecl, AstDeclAsm, AstDeclCustomRustyCppEnum, AstDeclEmpty,
        AstDeclNamespace, AstDeclUsingNamespace,
    },
};
use crate::{
    Ast::NestedNameSpecifier::AstNestedNameSpecifier,
    Sema::Scope::{Child, RefCellScope, Scope, ScopeKind, ScopeRef},
    Utils::{
        StringRef::StringRef,
        Structs::{CompileError, CompileMsgImpl, SourceRange},
    },
};

use super::super::Parser;

impl Parser {
    /**
     * empty-declaration | attribute-declaration
     */
    pub fn actOnEmptyDecl(&mut self, attr: &[AstAttribute], location: SourceRange) -> Vec<AstDecl> {
        for a in attr {
            if let Attribute::Kind::Cxx(attrmembers) = a.getKind() {
                for attrmember in attrmembers {
                    (*attrmember).actOnAttributeDeclTestRemoveMePlease(self);
                }
            } else {
                self.errors.push(CompileError::fromSourceRange(
                    "Only Cxx11 attributes are allowed here.",
                    &a.getSourceRange(),
                ));
                continue;
            }
        }
        let ast = AstDeclEmpty::new(
            self.alloc(),
            location,
            self.astContext.currentScope.clone(),
            self.alloc().alloc_slice_copy(attr),
        );
        vec![ast.into()]
    }

    /**
     * asm-declaration
     */
    pub fn actOnAsmDecl(
        &mut self,
        attr: &[AstAttribute],
        location: SourceRange,
        asm: StringRef,
    ) -> Vec<AstDecl> {
        let astAsm = AstDeclAsm::new(
            self.alloc(),
            location,
            self.astContext.currentScope.clone(),
            self.alloc().alloc_slice_copy(attr),
            asm,
        );
        vec![astAsm.into()]
    }

    /**
     * named-namespace-definition
     */
    pub fn actOnStartNamedNamespaceDefinition(
        &mut self,
        isInline: bool,
        attr: &[AstAttribute],
        name: StringRef,
        locationName: SourceRange,
    ) -> Vec<AstDecl> {
        let createNamespace = |parser: &mut Self, scope: ScopeRef| -> AstDeclNamespace {
            return AstDeclNamespace::new(
                parser.alloc(),
                locationName,
                scope,
                parser.alloc().alloc_slice_copy(attr),
                name,
                isInline,
                parser.astContext.currentScope.clone(),
            );
        };
        let possibleOriginalDecl = self.namespaceExtendableLookup(name);

        if let Some(originalDecl) = possibleOriginalDecl {
            let AstDecl::AstDeclNamespace(causingDecl) = originalDecl.borrow().causingDecl.unwrap() else {unreachable!();};
            if isInline && !causingDecl.isInline() {
                self.errors.push(CompileError::fromSourceRange(
                    "Namespace redefinition with \"inline\", while original did not have it.",
                    &locationName,
                ));
            }
            let astNamespaceDecl = createNamespace(self, originalDecl.clone());
            causingDecl.addExtension(astNamespaceDecl);
            self.astContext.currentScope = originalDecl;
            return vec![astNamespaceDecl.into()];
        }
        let enumScope = Scope::new(ScopeKind::NAMESPACE | ScopeKind::CAN_DECL);
        let astNamespaceDecl = createNamespace(self, enumScope.clone());
        enumScope.setCausingDecl(astNamespaceDecl.into());

        if isInline {
            self.astContext
                .currentScope
                .addInlinedChild(name, enumScope.clone());
        } else {
            self.astContext
                .currentScope
                .addChild(name, Child::Scope(enumScope.clone()));
        }
        self.astContext.currentScope = enumScope;
        vec![astNamespaceDecl.into()]
    }

    /**
     * named-namespace-definition
     */
    pub fn actOnEndNamedNamespaceDefinition(
        &mut self,
        namespaceDecl: &AstDecl,
        contents: &[AstDecl],
    ) {
        let Ok(namespaceDecl) =
        TryInto::<AstDeclNamespace>::try_into(namespaceDecl) else {unreachable!();};

        let contents = self.alloc().alloc_slice_copy(contents);
        namespaceDecl.setContents(contents);

        let newCurrent = self
            .astContext
            .currentScope
            .borrow()
            .parent
            .clone()
            .unwrap();
        self.astContext.currentScope = newCurrent;
    }

    pub fn actOnRustyCppEnumDefinition(
        &mut self,
        name: StringRef,
        location: SourceRange,
        attr: &[AstAttribute],
    ) -> Vec<AstDecl> {
        let attrs = self.alloc().alloc_slice_copy(attr);

        let enumScope = Scope::new(ScopeKind::ENUM | ScopeKind::CAN_DECL);

        let astEnum =
            AstDeclCustomRustyCppEnum::new(self.alloc(), location, enumScope.clone(), attrs, name);
        let astEnumDecl = astEnum.into();
        enumScope.setCausingDecl(astEnumDecl);

        self.astContext
            .currentScope
            .addChild(name, Child::Scope(enumScope.clone()));
        self.astContext.currentScope = enumScope;

        // Imediately pop, for now.
        let newCurrent = self
            .astContext
            .currentScope
            .borrow()
            .parent
            .clone()
            .unwrap();
        self.astContext.currentScope = newCurrent;
        vec![astEnumDecl]
    }

    pub fn actOnUsingNamespaceDefinition(
        &mut self,
        name: StringRef,
        location: SourceRange,
        attr: &[AstAttribute],
        nestedNameSpecifier: &'static [AstNestedNameSpecifier],
        scope: Option<&ScopeRef>,
    ) -> Vec<AstDecl> {
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
        self.astContext
            .currentScope
            .addUsingNamespace(result.clone());
        let attr = self.alloc().alloc_slice_copy(attr);
        vec![AstDeclUsingNamespace::new(
            self.alloc(),
            location,
            result,
            attr,
            name,
            nestedNameSpecifier,
        )
        .into()]
    }
}
