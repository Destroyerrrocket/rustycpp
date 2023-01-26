use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{Attribute::AstAttribute, Decl::BaseDecl, NestedNameSpecifier::AstNestedNameSpecifier},
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
    scope: &'static [AstNestedNameSpecifier],
}

impl AstUsingNamespaceDecl {
    pub fn new(
        sourceRange: SourceRange,
        name: StringRef,
        attrs: &'static [&'static AstAttribute],
        scope: &'static [AstNestedNameSpecifier],
    ) -> Self {
        Self {
            base: BaseDecl::new(sourceRange),
            name,
            attrs,
            scope,
        }
    }
}
