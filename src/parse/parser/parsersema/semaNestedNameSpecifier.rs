use crate::{
    ast::NestedNameSpecifier::AstNestedNameSpecifier,
    sema::scope::{Child, ScopeKind},
    utils::structs::{CompileError, CompileMsgImpl},
};

use super::super::Parser;

impl Parser {
    fn resolveNestedNameSpecifier(&mut self, nestedNameSpecifier: &[AstNestedNameSpecifier]) {
        if nestedNameSpecifier.len() > 1 {
            let mut currentResolvedScope = &nestedNameSpecifier[0];
            let mut remainingNameSpecifier = &nestedNameSpecifier[1..];
            // Resolve all namespaces
            loop {
                let currentUnresolved = &remainingNameSpecifier[0];
                let currentResolvedScopeScope = currentResolvedScope.scope.borrow();
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
                    currentUnresolved.setScope(currentResolvedScopeScope.clone());
                } else if candidates.is_empty() {
                    self.errors.push(CompileError::fromSourceRange(
                        "The name could not be resolved to a type, enum or namespace.",
                        &currentUnresolved.sourceRange,
                    ));
                    currentUnresolved.setScope(currentResolvedScopeScope.clone());
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
    ) -> &'static [AstNestedNameSpecifier] {
        // We need to perform qualified name resolution
        self.resolveNestedNameSpecifier(nestedNameSpecifier);
        self.alloc().alloc_slice_clone(nestedNameSpecifier)
    }
}
