use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{Attribute::AstAttribute, Decl::BaseDecl},
    sema::scope::ScopeRef,
    utils::structs::SourceRange,
};

#[derive(CommonAst, DeclAst)]
pub struct AstEmptyDecl {
    base: BaseDecl,
    #[DeclAttributes]
    #[AstChildSlice]
    attrs: &'static [&'static AstAttribute],
}

impl AstEmptyDecl {
    pub fn new(
        sourceRange: SourceRange,
        scope: ScopeRef,
        attrs: &'static [&'static AstAttribute],
    ) -> Self {
        Self {
            base: BaseDecl::new(sourceRange, scope),
            attrs,
        }
    }
}
