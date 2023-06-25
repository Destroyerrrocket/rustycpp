use crate::Sema::{
    Scope::{Scope, ScopeRef},
    TypeDict::TypeDict,
};
use crate::Utils::UnsafeAllocator::UnsafeAllocator;

pub struct AstContext {
    pub rootScope: ScopeRef,
    pub currentScope: ScopeRef,

    pub alloc: UnsafeAllocator,
    pub typeDict: TypeDict,
}

impl AstContext {
    pub fn new() -> Self {
        let rootScope = Scope::new_root();
        let alloc: UnsafeAllocator = UnsafeAllocator::default();
        Self {
            rootScope: rootScope.clone(),
            currentScope: rootScope,

            typeDict: TypeDict::new(alloc.alloc()),
            alloc,
        }
    }
}

impl Default for AstContext {
    fn default() -> Self {
        Self::new()
    }
}
