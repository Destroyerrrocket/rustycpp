use crate::ast::common::AstAttribute;
use crate::ast::common::AstDeclEmptyStructNode;
use crate::sema::scope::ScopeRef;
use crate::utils::structs::SourceRange;
use crate::Base;
use crate::Parent;
use deriveMacros::CommonAst;

#[derive(CommonAst)]
pub struct AstDeclEmptyStruct;

impl AstDeclEmptyStruct {
    pub fn new() -> Self {
        Self {}
    }
}

impl AstDeclEmptyStructNode {
    pub fn new(sourceRange: SourceRange, scope: ScopeRef, attrs: &'static [AstAttribute]) -> Self {
        Self {
            parent: <Parent!()>::new(sourceRange, scope, attrs),
            base: <Base!()>::new(),
        }
    }
}
