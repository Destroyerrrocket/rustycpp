use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{Attribute::AstAttribute, Decl::BaseDecl, NestedNameSpecifier::AstNestedNameSpecifier},
    sema::scope::ScopeRef,
    utils::{stringref::StringRef, structs::SourceRange},
};

#[derive(CommonAst, DeclAst)]
pub struct AstUsingNamespaceDecl {
    base: BaseDecl,
    #[AstToString]
    name: StringRef,
    #[DeclAttributes]
    #[AstChildSlice]
    attrs: &'static [&'static AstAttribute],
    #[AstChildSlice]
    nestedNameSpecifier: &'static [AstNestedNameSpecifier],
}

impl AstUsingNamespaceDecl {
    pub fn new(
        sourceRange: SourceRange,
        scope: ScopeRef,
        name: StringRef,
        attrs: &'static [&'static AstAttribute],
        nestedNameSpecifier: &'static [AstNestedNameSpecifier],
    ) -> Self {
        Self {
            base: BaseDecl::new(sourceRange, scope),
            name,
            attrs,
            nestedNameSpecifier,
        }
    }
}
