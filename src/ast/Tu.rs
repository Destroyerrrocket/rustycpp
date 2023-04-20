use std::rc::Rc;

use deriveMacros::CommonAst;

use crate::{
    ast::{Decl::AstDecl, Type::TypeDict},
    utils::unsafeallocator::UnsafeAllocator,
};

#[derive(Clone, CommonAst)]
pub struct AstTu {
    #[AstChildSlice]
    globalDecl: &'static [&'static AstDecl],
    typeDict: TypeDict,
    alloc: Rc<UnsafeAllocator>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for AstTu {}

impl AstTu {
    pub fn new(
        alloc: Rc<UnsafeAllocator>,
        typeDict: TypeDict,
        global: &[&'static AstDecl],
    ) -> Self {
        let globalDecl = alloc.alloc().alloc_slice_clone(global);
        Self {
            globalDecl,
            typeDict,
            alloc,
        }
    }
}
