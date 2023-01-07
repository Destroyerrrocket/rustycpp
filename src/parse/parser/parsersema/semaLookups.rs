use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::common::AstDecl,
    sema::scope::{Child, Scope, ScopeKind},
    utils::stringref::StringRef,
};

use super::super::Parser;

impl Parser {
    /**
     * Implemented according to the C++ Standard 6.5.2
     *
     * While I'm aware of the rules in 6.4.10 regarding name higing,
     * I think I'd prefer for now to return all the names that match, and apply
     * name hiding later.
     */
    fn unqualifiedNameLookup(&self, name: StringRef) -> Vec<&'static AstDecl> {
        /*
        Rules 1 to 3 are not accually rules, but comments on how to apply the rules.
        Rule 1: Apply the rules in order, and for each rule follow the statements in order.
        Rule 2: If a using namespace directive is found, the members of the namespace are added to the lookup set always.
        Rule 3: If you're looking up an unqualified name for a function call, use the rules in 6.5.3. 6.5.2 does not apply.
        */
        let scope = self.currentScope.borrow();

        let getChilds = |scope: &Rc<RefCell<Scope>>| {
            scope
                .borrow()
                .childs
                .get(&name)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|x| match x {
                    Child::Decl(decl) => decl,
                    Child::Scope(scope) => scope.borrow().causingDecl.unwrap(),
                })
                .collect::<Vec<_>>()
        };

        let getChildsAndAliased = |scope: &Rc<RefCell<Scope>>| {
            let mut result = Vec::new();
            result.extend(getChilds(scope));
            result.extend(scope.borrow().inlinedNamespaces.iter().flat_map(getChilds));
            result.extend(scope.borrow().usingNamespaces.iter().flat_map(getChilds));
            result
        };

        // Global scope, applying rule 4. Just look at this scope and all its aliased/using namespaces.
        if scope.parent.is_none() {
            return getChildsAndAliased(&self.currentScope);
        }

        /*
         * Rule 5: Inside a namespace. Look at the current scope and all its
         * aliased/using namespaces, if not found, look at the parent scope,
         * recursively.
         */
        if scope.flags & ScopeKind::NAMESPACE == ScopeKind::NAMESPACE {
            let mut currVisitingScope = self.currentScope.clone();
            return loop {
                let result = getChildsAndAliased(&currVisitingScope);
                if !result.is_empty() {
                    break result;
                }
                if currVisitingScope.borrow().parent.is_none() {
                    break vec![];
                }
                let newCurr = currVisitingScope.borrow().parent.as_ref().unwrap().clone();
                debug_assert!(
                    {
                        let newCurr = newCurr.borrow();
                        newCurr.flags & ScopeKind::NAMESPACE == ScopeKind::NAMESPACE
                            || newCurr.parent.is_none()
                    },
                    "I assumed that namespaces are always nested in other namespaces."
                );
                currVisitingScope = newCurr;
            };
        }
        todo!("No more namespace resolution rules implemented.");
    }
}
