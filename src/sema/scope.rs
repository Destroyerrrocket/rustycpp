use std::{cell::RefCell, collections::HashMap, rc::Rc};

/**
 * This is inspired by clang's [scope flags](https://github.com/llvm/llvm-project/blob/fec5ff2a3230ac9214891879e97b67dd6db833ed/clang/include/clang/Sema/Scope.h)
 * enum, so I don't have to check which scopes I'll need.
 */
use bitflags::bitflags;

use crate::{ast::Decl::AstDecl, utils::stringref::StringRef};
bitflags! {
/// `ScopeKind` - This bitflag is used to configure the kind of scope we have. Based on clang's ScopeFlags (comments from them).
pub struct ScopeKind : u32 {
    const FUNCTION = 0x01;

    /// This is a while, do, switch, for, etc that can have break
    /// statements embedded into it.
    const CAN_BREAK    = 0x02;

    /// This is a while, do, for, which can have continue statements
    /// embedded into it.
    const CAN_CONTINUE = 0x04;

    /// This is a scope that can contain a declaration.  Some scopes
    /// just contain loop constructs but don't contain decls.
    const CAN_DECL = 0x08;

    /// The controlling scope in a if/switch/while/for statement.
    const CONTROL = 0x10;

    /// The scope of a struct/union/class definition.
    const CLASS = 0x20;

    /// This is a scope that corresponds to a block/closure object.
    /// Blocks serve as top-level scopes for some objects like labels, they
    /// also prevent things like break and continue.  BlockScopes always have
    /// the FUNCTION and CAN_DECL flags set as well.
    const BLOCK = 0x40;

    /// This is a scope that corresponds to the
    /// template parameters of a C++ template. Template parameter
    /// scope starts at the 'template' keyword and ends when the
    /// template declaration ends.
    const TEMPLATE_PARAMETER = 0x80;

    /// This is a scope that corresponds to the
    /// parameters within a function prototype.
    const FUNCTION_PROTOTYPE_PARAMS = 0x100;

    /// This is a scope that corresponds to the parameters within
    /// a function prototype for a function declaration (as opposed to any
    /// other kind of function declarator). Always has FunctionPrototypeScope
    /// set as well.
    const FUNCTION_DECLARATION_PARAMS = 0x200;

    /// This is a scope that corresponds to a switch statement.
    const SWITCH = 0x400;

    /// This is the scope of a C++ try statement.
    const TRY = 0x800;

    /// This is the scope of a C++ catch statement.
    const CATCH = 0x1000;


    /// This is the scope for a function-level C++ try or catch scope.
    const FUNCTION_TRY_CATCH = 0x2000;

    /// This scope corresponds to an enum.
    const ENUM = 0x4000;

    /// This is a compound statement scope.
    const COMPOUND_STMT = 0x8000;

    /// We are between inheritance colon and the real class/struct definition
    /// scope.
    const CLASS_INHERITANCE_COLON = 0x10000;

    /// This is a C++ namespace
    const NAMESPACE = 0x20000;
}
}

#[derive(Clone)]
pub enum Child {
    /**
     * This is a child that is a declaration and can't have further children, like a variable.
     */
    Decl(&'static AstDecl),
    /**
     * This is a child that can have further children, like a function, class, or namespace.
     */
    Scope(ScopeRef),
}

impl Child {
    pub fn getScope(&self) -> Option<ScopeRef> {
        match self {
            Self::Decl(_) => None,
            Self::Scope(scope) => Some(scope.clone()),
        }
    }

    pub fn getDecl(&self) -> &'static AstDecl {
        match self {
            Self::Decl(res) => res,
            Self::Scope(scope) => scope.borrow().causingDecl.unwrap(),
        }
    }
}

pub struct Scope {
    /**
     * This is the kind of scope this is.
     */
    pub flags: ScopeKind,
    /**
     * This is the parent of this scope.
     * Only the root scope has no parent.
     */
    pub parent: Option<ScopeRef>,
    /**
     * This is a map of all the children in this scope.
     * The key is the name of the child, and the value is a vector of all the children with that name.
     * This is because a scope can have multiple children with the same name, like a set of functions with parameter overloading.
     */
    pub childs: HashMap<StringRef, Vec<Child>>,

    // TODO: Does it need to be wrapped in Child? I can't think of a scope that could not have children that is nameless.
    /**
     * This is a vector of all the children in this scope that have no name.
     * This is because a compound stmt scope, for example, does not have a name.
     */
    pub namelessChilds: Vec<Child>,

    /**
     * This is the declaration that this scope is associated with.
     * For example, the causing declaration of a class scope is the class itself, of a function scope is a function, etc.
     */
    pub causingDecl: Option<&'static AstDecl>,

    // RESOLVED: Can we merge inlinedNamespaces and usingNamespaces into one vector?
    // No. Qualified name lookup in namespaces need to be able to distinguish between inlined and using namespaces.
    /**
     * These are inlined scope (by inline namespace)
     */
    pub inlinedNamespaces: Vec<ScopeRef>,

    /**
     * These are using namespace scopes
     */
    pub usingNamespaces: Vec<ScopeRef>,
}

impl Scope {
    pub fn new(flags: ScopeKind) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            flags,
            parent: None,
            childs: HashMap::new(),
            namelessChilds: Vec::new(),
            causingDecl: None,
            inlinedNamespaces: Vec::new(),
            usingNamespaces: Vec::new(),
        }))
    }

    pub fn new_unknown_cause(flags: ScopeKind) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            flags,
            parent: None,
            childs: HashMap::new(),
            namelessChilds: Vec::new(),
            causingDecl: None,
            inlinedNamespaces: Vec::new(),
            usingNamespaces: Vec::new(),
        }))
    }

    pub fn new_root() -> Rc<RefCell<Self>> {
        Self::new_unknown_cause(ScopeKind::CAN_DECL | ScopeKind::NAMESPACE)
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait RefCellScope {
    fn setCausingDecl(&self, decl: &'static AstDecl);
    fn addNamelessChild(&self, child: Child);
    fn addChild(&self, name: StringRef, child: Child);
    fn addInlinedChild(&self, name: StringRef, child: ScopeRef);
    fn addUsingNamespace(&self, child: ScopeRef);
}

#[allow(clippy::module_name_repetitions)]
pub type ScopeRef = Rc<RefCell<Scope>>;

impl RefCellScope for ScopeRef {
    fn setCausingDecl(&self, decl: &'static AstDecl) {
        assert!(self.borrow().causingDecl.is_none());
        self.borrow_mut().causingDecl = Some(decl);
    }

    fn addNamelessChild(&self, child: Child) {
        if let Child::Scope(scope) = &child {
            assert!(scope.borrow().parent.is_none());
            scope.borrow_mut().parent = Some(self.clone());
        }
        self.borrow_mut().namelessChilds.push(child);
    }

    fn addChild(&self, name: StringRef, child: Child) {
        if let Child::Scope(scope) = &child {
            assert!(scope.borrow().parent.is_none());
            scope.borrow_mut().parent = Some(self.clone());
        }

        let mut this = self.borrow_mut();
        if let Some(children) = this.childs.get_mut(&name) {
            children.push(child);
        } else {
            this.childs.insert(name, vec![child]);
        }
    }

    fn addInlinedChild(&self, name: StringRef, child: ScopeRef) {
        let mut this = self.borrow_mut();
        assert!(child.borrow().parent.is_none());
        child.borrow_mut().parent = Some(self.clone());
        this.inlinedNamespaces.push(child.clone());

        if let Some(children) = this.childs.get_mut(&name) {
            children.push(Child::Scope(child));
        } else {
            this.childs.insert(name, vec![Child::Scope(child)]);
        }
    }

    fn addUsingNamespace(&self, child: ScopeRef) {
        let mut this = self.borrow_mut();
        this.usingNamespaces.push(child);
    }
}
