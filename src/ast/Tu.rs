use std::rc::Rc;

use deriveMacros::CommonAst;

use crate::utils::unsafeallocator::UnsafeAllocator;

use super::common::AstDecl;

#[derive(Clone, CommonAst)]
pub struct AstTu {
    #[AstChildSlice]
    globalDecl: &'static [&'static AstDecl],
    alloc: Rc<UnsafeAllocator>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for AstTu {}

impl AstTu {
    pub fn new(alloc: Rc<UnsafeAllocator>, global: &[&'static AstDecl]) -> Self {
        let globalDecl = alloc.alloc().alloc_slice_clone(global);
        Self { globalDecl, alloc }
    }
}
