use crate::{
    Ast::NestedNameSpecifier::AstNestedNameSpecifier,
    Sema::Scope::{Child, ScopeKind},
    Utils::Structs::{CompileError, CompileMsgImpl},
};

use super::super::Parser;

impl Parser {
    fn resolveNestedNameSpecifier(
        &mut self,
        nestedNameSpecifier: &[AstNestedNameSpecifier],
        reportNormalErrors: bool,
    ) {
        if nestedNameSpecifier.len() > 1 {
            let mut currentResolvedScope = &nestedNameSpecifier[0];
            let mut remainingNameSpecifier = &nestedNameSpecifier[1..];
            // Resolve all namespaces
            loop {
                let currentUnresolved = &remainingNameSpecifier[0];
                let currentResolvedScopeScope = currentResolvedScope.scope.borrow();
                if currentResolvedScopeScope.is_none() {
                    return; // Previous resolution failed...
                }
                let currentResolvedScopeScope = currentResolvedScopeScope.as_ref().unwrap();
                let candidates = Self::qualifiedNameLookup(
                    currentUnresolved.getName(),
                    currentResolvedScopeScope,
                )
                .into_iter()
                /* We want only types, enums and namespaces*/
                .filter(|x| match x {
                    Child::Decl(_) => false,
                    Child::Scope(scope) => scope
                        .borrow()
                        .flags
                        .intersects(ScopeKind::NAMESPACE | ScopeKind::ENUM | ScopeKind::CLASS),
                })
                .collect::<Vec<_>>();
                if candidates.len() > 1 {
                    self.errors.push(CompileError::fromSourceRange(
                        "Ambiguous name, compiler bug, please report.",
                        &currentUnresolved.sourceRange,
                    ));
                } else if candidates.is_empty() {
                    if reportNormalErrors {
                        self.errors.push(CompileError::fromSourceRange(
                            "The name could not be resolved to a type, enum or namespace.",
                            &currentUnresolved.sourceRange,
                        ));
                    }
                } else {
                    match &candidates[0] {
                        Child::Decl(_) => unreachable!(),
                        Child::Scope(scope) => {
                            currentUnresolved.setScope(scope.clone());
                        }
                    }
                }
                if remainingNameSpecifier.len() == 1 {
                    break;
                }
                currentResolvedScope = currentUnresolved;
                remainingNameSpecifier = &remainingNameSpecifier[1..];
            }
        }
    }
    pub fn actOnNestedNameSpecifier(
        &mut self,
        nestedNameSpecifier: &[AstNestedNameSpecifier],
        reportNormalErrors: bool,
    ) -> &'static [AstNestedNameSpecifier] {
        // We need to perform qualified name resolution
        self.resolveNestedNameSpecifier(nestedNameSpecifier, reportNormalErrors);
        self.alloc().alloc_slice_clone(nestedNameSpecifier)
    }
}
