use crate::Ast::Common::AstAttribute;
use crate::Ast::Common::AstDeclEmpty;
use crate::Ast::Common::AstDeclEmptyStructNode;
use crate::Base;
use crate::Parent;
use crate::Sema::Scope::ScopeRef;
use crate::Utils::Structs::SourceRange;
use deriveMacros::{CommonAst, RustycppInheritanceConstructors};

#[derive(CommonAst)]
pub struct AstDeclEmptyStruct;

impl AstDeclEmptyStruct {
    pub const fn new() -> Self {
        Self {}
    }
}

#[RustycppInheritanceConstructors]
impl AstDeclEmptyStructNode {
    pub fn new(sourceRange: SourceRange, scope: ScopeRef, attrs: &'static [AstAttribute]) -> Self {
        Self {
            parent: <Parent!()>::new(sourceRange, scope, attrs),
            base: <Base!()>::new(),
        }
    }
}
