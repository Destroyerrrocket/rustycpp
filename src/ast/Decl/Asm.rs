use crate::ast::common::AstAttribute;
use crate::Parent;
use crate::{ast::common::AstDeclAsmStructNode, utils::structs::SourceRange};
use crate::{sema::scope::ScopeRef, Base};
use deriveMacros::CommonAst;

use crate::utils::stringref::StringRef;

#[derive(CommonAst)]
pub struct AstDeclAsmStruct {
    #[AstToString]
    asm: StringRef,
}

impl AstDeclAsmStruct {
    pub fn new(asm: StringRef) -> Self {
        Self { asm }
    }
}

impl AstDeclAsmStructNode {
    pub fn new(
        sourceRange: SourceRange,
        scope: ScopeRef,
        attrs: &'static [AstAttribute],
        asm: StringRef,
    ) -> Self {
        Self {
            parent: <Parent!()>::new(sourceRange, scope, attrs),
            base: <Base!()>::new(asm),
        }
    }
}
