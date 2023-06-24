use crate::Ast::Common::AstAttribute;
use crate::Ast::Common::AstDeclCustomRustyCppEnum;
use crate::Ast::Common::AstDeclCustomRustyCppEnumStructNode;
use crate::Base;
use crate::Parent;
use crate::Sema::Scope::ScopeRef;
use crate::Utils::StringRef::StringRef;
use crate::Utils::Structs::SourceRange;
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
