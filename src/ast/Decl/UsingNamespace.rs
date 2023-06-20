use crate::ast::common::AstAttribute;
use crate::ast::common::AstDeclUsingNamespaceStructNode;
use crate::sema::scope::ScopeRef;
use crate::utils::structs::SourceRange;
use crate::Base;
use crate::Parent;
use deriveMacros::CommonAst;

use crate::{ast::NestedNameSpecifier::AstNestedNameSpecifier, utils::stringref::StringRef};

#[derive(CommonAst)]
pub struct AstDeclUsingNamespaceStruct {
    #[AstToString]
    name: StringRef,
    #[AstChildSlice]
    nestedNameSpecifier: &'static [AstNestedNameSpecifier],
}

impl AstDeclUsingNamespaceStruct {
    pub fn new(name: StringRef, nestedNameSpecifier: &'static [AstNestedNameSpecifier]) -> Self {
        Self {
            name,
            nestedNameSpecifier,
        }
    }
}

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
