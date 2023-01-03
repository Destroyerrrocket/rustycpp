use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{common::BaseDecl, Attribute::AstAttribute},
    utils::structs::SourceRange,
};

#[derive(CommonAst, DeclAst)]
pub struct AstEmptyDecl {
    base: BaseDecl,
    #[AstChildSlice]
    attrs: &'static [&'static AstAttribute],
}

impl AstEmptyDecl {
    pub fn new(sourceRange: SourceRange, attrs: &'static [&'static AstAttribute]) -> Self {
        Self {
            base: BaseDecl::new(sourceRange),
            attrs,
        }
    }
}
