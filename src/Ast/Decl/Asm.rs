use crate::Ast::Common::AstAttribute;
use crate::Ast::Common::AstDeclAsm;
use crate::Parent;
use crate::{Ast::Common::AstDeclAsmStructNode, Utils::Structs::SourceRange};
use crate::{Base, Sema::Scope::ScopeRef};
use deriveMacros::CommonAst;
use deriveMacros::RustycppInheritanceConstructors;

use crate::Utils::StringRef::StringRef;

#[derive(CommonAst)]
pub struct AstDeclAsmStruct {
    #[AstToString]
    asm: StringRef,
}

impl AstDeclAsmStruct {
    pub const fn new(asm: StringRef) -> Self {
        Self { asm }
    }
}

#[RustycppInheritanceConstructors]
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
