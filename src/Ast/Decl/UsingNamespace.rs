use crate::Ast::Common::AstAttribute;
use crate::Ast::Common::AstDeclUsingNamespace;
use crate::Ast::Common::AstDeclUsingNamespaceStructNode;
use crate::Base;
use crate::Parent;
use crate::Sema::Scope::ScopeRef;
use crate::Utils::Structs::SourceRange;
use crate::{Ast::NestedNameSpecifier::AstNestedNameSpecifier, Utils::StringRef::StringRef};
use deriveMacros::CommonAst;
use deriveMacros::RustycppInheritanceConstructors;

#[derive(CommonAst)]
pub struct AstDeclUsingNamespaceStruct {
    #[AstToString]
    name: StringRef,
    #[AstChildSlice]
    nestedNameSpecifier: &'static [AstNestedNameSpecifier],
}

impl AstDeclUsingNamespaceStruct {
    pub const fn new(
        name: StringRef,
        nestedNameSpecifier: &'static [AstNestedNameSpecifier],
    ) -> Self {
        Self {
            name,
            nestedNameSpecifier,
        }
    }
}

#[RustycppInheritanceConstructors]
impl AstDeclUsingNamespaceStructNode {
    pub fn new(
        sourceRange: SourceRange,
        scope: ScopeRef,
        attrs: &'static [AstAttribute],
        name: StringRef,
        nestedNameSpecifier: &'static [AstNestedNameSpecifier],
    ) -> Self {
        Self {
            parent: <Parent!()>::new(sourceRange, scope, attrs),
            base: <Base!()>::new(name, nestedNameSpecifier),
        }
    }
}
