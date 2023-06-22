use crate::ast::common::AstAttribute;
use crate::ast::common::AstDeclCustomRustyCppEnum;
use crate::ast::common::AstDeclCustomRustyCppEnumStructNode;
use crate::sema::scope::ScopeRef;
use crate::utils::stringref::StringRef;
use crate::utils::structs::SourceRange;
use crate::Base;
use crate::Parent;
use deriveMacros::CommonAst;
use deriveMacros::RustycppInheritanceConstructors;

#[derive(CommonAst)]
pub struct AstDeclCustomRustyCppEnumStruct {
    #[AstToString]
    name: StringRef,
}

impl AstDeclCustomRustyCppEnumStruct {
    pub const fn new(name: StringRef) -> Self {
        Self { name }
    }
}

#[RustycppInheritanceConstructors]
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
