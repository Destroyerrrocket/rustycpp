use deriveMacros::{CommonAst, DeclAst};

use crate::{
    ast::Decl::BaseDecl,
    utils::{stringref::StringRef, structs::SourceRange},
};

#[derive(CommonAst, DeclAst)]
pub struct AstCustomRustyCppEnum {
    base: BaseDecl,
    #[AstToString]
    name: StringRef,
}

impl AstCustomRustyCppEnum {
    pub fn new(sourceRange: SourceRange, name: StringRef) -> Self {
        Self {
            base: BaseDecl::new(sourceRange),
            name,
        }
    }
}
