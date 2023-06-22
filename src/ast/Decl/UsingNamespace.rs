use crate::ast::common::AstAttribute;
use crate::ast::common::AstDeclUsingNamespace;
use crate::ast::common::AstDeclUsingNamespaceStructNode;
use crate::sema::scope::ScopeRef;
use crate::utils::structs::SourceRange;
use crate::Base;
use crate::Parent;
use crate::{ast::NestedNameSpecifier::AstNestedNameSpecifier, utils::stringref::StringRef};
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
