use crate::Ast::Common::AstTu;
use crate::Ast::Common::AstTuStructNode;
use crate::Sema::AstContext::AstContext;

use deriveMacros::{CommonAst, RustycppInheritanceConstructors};

use crate::{Ast::Common, Base, Parent};

#[derive(CommonAst)]
pub struct AstTuStruct {
    #[AstChildSlice]
    globalDecl: &'static [Common::AstDecl],
    astContext: AstContext,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for AstTuStruct {}

impl AstTuStruct {
    pub fn new(astContext: AstContext, global: &[Common::AstDecl]) -> Self {
        let globalDecl = astContext.alloc.alloc().alloc_slice_clone(global);
        Self {
            globalDecl,
            astContext,
        }
    }
}

#[RustycppInheritanceConstructors]
impl AstTuStructNode {
    pub fn new(astContext: AstContext, global: &[Common::AstDecl]) -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::new(astContext, global),
        }
    }
}
