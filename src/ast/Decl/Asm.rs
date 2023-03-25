use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{Attribute::AstAttribute, Decl::BaseDecl},
    sema::scope::ScopeRef,
    utils::{stringref::StringRef, structs::SourceRange},
};

#[derive(CommonAst, DeclAst)]
pub struct AstAsmDecl {
    base: BaseDecl,
    #[DeclAttributes]
    #[AstChildSlice]
    attrs: &'static [&'static AstAttribute],
    #[AstToString]
    asm: StringRef,
}

impl AstAsmDecl {
    pub fn new(
        sourceRange: SourceRange,
        scope: ScopeRef,
        attrs: &'static [&'static AstAttribute],
        asm: StringRef,
    ) -> Self {
        Self {
            base: BaseDecl::new(sourceRange, scope),
            attrs,
            asm,
        }
    }
}
