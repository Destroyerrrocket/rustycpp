use crate::ast::common::AstTuStructNode;
use std::rc::Rc;

use deriveMacros::CommonAst;

use crate::{
    ast::{common, Type::TypeDict},
    utils::unsafeallocator::UnsafeAllocator,
    Base, Parent,
};

#[derive(Clone, CommonAst)]
pub struct AstTuStruct {
    #[AstChildSlice]
    globalDecl: &'static [common::AstDecl],
    typeDict: TypeDict,
    alloc: Rc<UnsafeAllocator>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for AstTuStruct {}

impl AstTuStruct {
    pub fn new(alloc: Rc<UnsafeAllocator>, typeDict: TypeDict, global: &[common::AstDecl]) -> Self {
        let globalDecl = alloc.alloc().alloc_slice_clone(global);
        Self {
            globalDecl,
            typeDict,
            alloc,
        }
    }
}

impl AstTuStructNode {
    pub fn new(alloc: Rc<UnsafeAllocator>, typeDict: TypeDict, global: &[common::AstDecl]) -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::new(alloc, typeDict, global),
        }
    }
}
