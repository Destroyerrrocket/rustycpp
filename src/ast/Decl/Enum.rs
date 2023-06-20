use crate::ast::common::AstAttribute;
use crate::ast::common::AstDeclCustomRustyCppEnumStructNode;
use crate::sema::scope::ScopeRef;
use crate::utils::structs::SourceRange;
use crate::Base;
use crate::Parent;
use deriveMacros::CommonAst;

use crate::utils::stringref::StringRef;

#[derive(CommonAst)]
pub struct AstDeclCustomRustyCppEnumStruct {
    #[AstToString]
    name: StringRef,
}

impl AstDeclCustomRustyCppEnumStruct {
    pub fn new(name: StringRef) -> Self {
        Self { name }
    }
}

impl AstDeclCustomRustyCppEnumStructNode {
    pub fn new(
        sourceRange: SourceRange,
        scope: ScopeRef,
        attrs: &'static [AstAttribute],
        name: StringRef,
    ) -> Self {
        Self {
            parent: <Parent!()>::new(sourceRange, scope, attrs),
            base: <Base!()>::new(name),
        }
    }
}
