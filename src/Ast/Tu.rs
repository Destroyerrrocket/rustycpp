use crate::Ast::Common::AstTu;
use crate::Ast::Common::AstTuStructNode;
use std::rc::Rc;

use deriveMacros::{CommonAst, RustycppInheritanceConstructors};

use crate::{
    Ast::{Common, Type::TypeDict},
    Base, Parent,
    Utils::UnsafeAllocator::UnsafeAllocator,
};

#[derive(Clone, CommonAst)]
pub struct AstTuStruct {
    #[AstChildSlice]
    globalDecl: &'static [Common::AstDecl],
    typeDict: TypeDict,
    alloc: Rc<UnsafeAllocator>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for AstTuStruct {}

impl AstTuStruct {
    pub fn new(alloc: Rc<UnsafeAllocator>, typeDict: TypeDict, global: &[Common::AstDecl]) -> Self {
        let globalDecl = alloc.alloc().alloc_slice_clone(global);
        Self {
            globalDecl,
            typeDict,
            alloc,
        }
    }
}

#[RustycppInheritanceConstructors]
impl AstTuStructNode {
    pub fn new(alloc: Rc<UnsafeAllocator>, typeDict: TypeDict, global: &[Common::AstDecl]) -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::new(alloc, typeDict, global),
        }
    }
}
