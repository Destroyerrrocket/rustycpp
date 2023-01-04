use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{common::BaseDecl, Attribute::AstAttribute},
    utils::{stringref::StringRef, structs::SourceRange},
};

#[derive(CommonAst, DeclAst)]
pub struct AstAsmDecl {
    base: BaseDecl,
    #[AstChildSlice]
    attrs: &'static [&'static AstAttribute],
    #[AstToString]
    asm: StringRef,
}

impl AstAsmDecl {
    pub fn new(
        sourceRange: SourceRange,
        attrs: &'static [&'static AstAttribute],
        asm: StringRef,
    ) -> Self {
        Self {
            base: BaseDecl::new(sourceRange),
            attrs,
            asm,
        }
    }
}
