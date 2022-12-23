
use deriveMacros::CommonAst;

#[derive(Clone, Copy, CommonAst)]
pub struct AstTu;

impl AstTu {
    pub fn new_dont_use() -> Self {
        Self {}
    }
}
