use std::cell::RefCell;

use deriveMacros::CommonAst;

use crate::{
    sema::scope::ScopeRef,
    utils::{stringref::StringRef, structs::SourceRange},
};

#[derive(Clone, Copy)]
pub enum Kind {
    Global,
    Type(StringRef),
    Namespace(StringRef),

    // DeclType not yet implemented
    //DeclType,

    // Secondaries
    Identifier(StringRef),
    // Simple Template Id not yet implemented
    // SimpleTemplateId,
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Self::Global => "<global>".to_string(),
            Self::Type(name) | Self::Namespace(name) | Self::Identifier(name) => name.to_string(),
        }
    }
}

/**
 * Beware! `AstNestedNameSpecifier` usually goes in a vector, as our
 * implementation, instead of being recursive as it is in the standard, we just
 * parse the nodes individually.
 */
#[derive(Clone, CommonAst)]
pub struct AstNestedNameSpecifier {
    #[AstToString]
    pub kind: Kind,
    /// The range of the nestedNameSpecifier. Does not include the ::
    pub sourceRange: SourceRange,
    /// Evaluated scope of the nestedNameSpecifier.
    pub scope: RefCell<Option<ScopeRef>>,
}

impl AstNestedNameSpecifier {
    pub fn new(kind: Kind, sourceRange: SourceRange) -> Self {
        Self {
            kind,
            sourceRange,
            scope: RefCell::new(None),
        }
    }
    pub fn new_scoped(kind: Kind, sourceRange: SourceRange, scope: ScopeRef) -> Self {
        Self {
            kind,
            sourceRange,
            scope: RefCell::new(Some(scope)),
        }
    }

    pub fn getName(&self) -> StringRef {
        match self.kind {
            Kind::Type(name) | Kind::Namespace(name) | Kind::Identifier(name) => name,
            Kind::Global => panic!("NestedNameSpecifier::getName called on non-identifier"),
        }
    }

    pub fn setScope(&self, scope: ScopeRef) {
        self.scope.borrow_mut().replace(scope);
    }
}
