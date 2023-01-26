use std::{cell::RefCell, rc::Rc};

use crate::{
    sema::scope::{Child, Scope, ScopeKind},
    utils::stringref::StringRef,
};

use super::super::Parser;

impl Parser {
    /**
     * Get all unqualified name results, no condition
     */
    pub fn unqualifiedNameLookup(&self, name: StringRef) -> Vec<Child> {
        self.unqualifiedNameLookupWithCond(name, |_| true)
    }

    /**
     * Implemented according to the C++ Standard 6.5.2
     *
     * While I'm aware of the rules in 6.4.10 regarding name higing,
     * I think I'd prefer for now to return all the names that match, and apply
     * name hiding later.
     */
    pub fn unqualifiedNameLookupWithCond(
        &self,
        name: StringRef,
        cond: fn(&Child) -> bool,
    ) -> Vec<Child> {
        /*
        Rules 1 to 3 are not accually rules, but comments on how to apply the rules.
        Rule 1: Apply the rules in order, and for each rule follow the statements in order.
        Rule 2: If a using namespace directive is found, the members of the namespace are added to the lookup set always.
        Rule 3: If you're looking up an unqualified name for a function call, use the rules in 6.5.3. 6.5.2 does not apply.
        */
        let scope = self.currentScope.borrow();
        /*
         * Rule 4 (basically the same) / Rule 5: Inside a namespace. Look at the current scope and all its
         * aliased/using namespaces, if not found, look at the parent scope,
         * recursively.
         */
        if scope.flags & ScopeKind::NAMESPACE == ScopeKind::NAMESPACE {
            let mut currVisitingScope =
                unsafe { self.currentScope.try_borrow_unguarded() }.unwrap();
            return loop {
                let mut result =
                    Self::getChildsAndAliased(name, currVisitingScope, cond).peekable();
                if result.peek().is_some() {
                    break result.cloned().collect::<Vec<_>>();
                }

                if currVisitingScope.parent.is_none() {
                    break vec![];
                }
                let newCurr = unsafe {
                    currVisitingScope
                        .parent
                        .as_ref()
                        .unwrap()
                        .try_borrow_unguarded()
                }
                .unwrap();
                debug_assert!(
                    {
                        newCurr.flags & ScopeKind::NAMESPACE == ScopeKind::NAMESPACE
                            || newCurr.parent.is_none()
                    },
                    "I assumed that namespaces are always nested in other namespaces."
                );
                currVisitingScope = newCurr;
            };
        }
        todo!("No more unqualified name resolution rules implemented.");
    }

    fn getChilds(
        scope: &Scope,
        name: StringRef,
        cond: fn(&Child) -> bool,
    ) -> impl Iterator<Item = &Child> {
        scope
            .childs
            .get(&name)
            .into_iter()
            .flatten()
            .filter(move |x| cond(x))
    }
    fn getChildsAndAliased<'scope>(
        name: StringRef,
        scope: &'scope Scope,
        cond: fn(&Child) -> bool,
    ) -> Box<dyn Iterator<Item = &'scope Child> + 'scope> {
        return Box::new(
            Self::getChilds(scope, name, cond)
                .chain(scope.inlinedNamespaces.iter().flat_map(move |x| {
                    Self::getChildsAndAliased(
                        name,
                        unsafe { x.try_borrow_unguarded() }.unwrap(),
                        cond,
                    )
                }))
                .chain(scope.inlinedNamespaces.iter().flat_map(move |x| {
                    Self::getChildsAndAliased(
                        name,
                        unsafe { x.try_borrow_unguarded() }.unwrap(),
                        cond,
                    )
                })),
        );
    }

    /**
     * It's important that we first return the child candidates. Namespace lookup for extension depends on it
     */
    fn getChildsAndOnlyInlined<'scope>(
        name: StringRef,
        scope: &'scope Scope,
        cond: fn(&Child) -> bool,
    ) -> Box<dyn Iterator<Item = &'scope Child> + 'scope> {
        Box::new(
            Self::getChilds(scope, name, cond).chain(scope.inlinedNamespaces.iter().flat_map(
                move |x| {
                    Self::getChildsAndOnlyInlined(
                        name,
                        unsafe { x.try_borrow_unguarded() }.unwrap(),
                        cond,
                    )
                },
            )),
        )
    }

    #[allow(clippy::needless_lifetimes)]
    fn getAllUsingNamespaceInlined<'scope>(
        scope: &'scope Scope,
    ) -> impl Iterator<Item = Rc<RefCell<Scope>>> + 'scope {
        scope
            .usingNamespaces
            .iter()
            .cloned()
            .chain(scope.inlinedNamespaces.iter().flat_map(|inlined| {
                unsafe { inlined.try_borrow_unguarded() }
                    .unwrap()
                    .usingNamespaces
                    .iter()
                    .cloned()
            }))
    }

    fn qualifiedNameLookupOnNamespace<'scope>(
        name: StringRef,
        scope: &'scope Scope,
        cond: fn(&Child) -> bool,
    ) -> Box<dyn Iterator<Item = &'scope Child> + 'scope> {
        // Rule 2: Check namespace scope and all the inlined namespaces
        let mut result = Self::getChildsAndOnlyInlined(name, scope, cond).peekable();
        if result.peek().is_some() {
            return Box::new(result);
        }
        // Rule 3: If nothing found, check the using namespaces in the same way, and make a union of them.
        Box::new(Self::getAllUsingNamespaceInlined(scope).flat_map(move |x| {
            Self::qualifiedNameLookupOnNamespace(name, unsafe { &*x.as_ptr() }, cond)
        }))
    }

    pub fn qualifiedNameLookupWithCond(
        name: StringRef,
        scope: &Rc<RefCell<Scope>>,
        cond: fn(&Child) -> bool,
    ) -> Vec<Child> {
        // Namespace qualified?
        let scope = scope.borrow();
        if scope.flags.contains(ScopeKind::NAMESPACE) {
            return Self::qualifiedNameLookupOnNamespace(name, &scope, cond)
                .cloned()
                .collect::<Vec<_>>();
        }

        todo!("Qualified name lookup not implemented for this scope.")
    }

    pub fn qualifiedNameLookup(name: StringRef, scope: &Rc<RefCell<Scope>>) -> Vec<Child> {
        Self::qualifiedNameLookupWithCond(name, scope, |_: &Child| true)
    }

    /** 9.8.2.1 namespace.def.general
     * If the identifier, when looked up (6.5.2), (note: This is the unqualified lookup, but is ineficient for what comes next)
     * refers to a namespace-name (but not a namespace-alias) that was introduced in the namespace in
     * which the named-namespace-definition appears or that was introduced in a member of the inline namespace
     * set of that namespace, the namespace-definition extends the previously-declared namespace. Otherwise, the
     * identifier is introduced as a namespace-name into the declarative region in which the named-namespace-definition
     * appears
     */
    pub fn namespaceExtendableLookup(&self, name: StringRef) -> Option<Rc<RefCell<Scope>>> {
        let currentScope = self.currentScope.borrow();
        let candidate = Self::getChildsAndOnlyInlined(name, &currentScope, |scope: &Child| {
            let Child::Scope(scope) = scope else {return false;};
            scope.borrow().flags == ScopeKind::NAMESPACE | ScopeKind::CAN_DECL
        })
        .map(|scope| {
            let Child::Scope(scope) = scope else {unreachable!();};
            scope.clone()
        })
        .next();
        candidate
    }
}
