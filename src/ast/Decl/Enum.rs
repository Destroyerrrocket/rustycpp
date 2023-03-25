use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::{Attribute::AstAttribute, Decl::BaseDecl},
    sema::scope::ScopeRef,
    utils::{stringref::StringRef, structs::SourceRange},
};

#[derive(CommonAst, DeclAst)]
pub struct AstCustomRustyCppEnum {
    base: BaseDecl,
    #[AstToString]
    name: StringRef,
    #[DeclAttributes]
    #[AstChildSlice]
    attrs: &'static [&'static AstAttribute],
}

impl AstCustomRustyCppEnum {
    pub fn new(
        sourceRange: SourceRange,
        scope: ScopeRef,
        name: StringRef,
        attrs: &'static [&'static AstAttribute],
    ) -> Self {
        Self {
            base: BaseDecl::new(sourceRange, scope),
            name,
            attrs,
        }
    }
}
