use deriveMacros::CommonAst;

#[derive(Clone, Copy, CommonAst)]
pub struct AstTu {
    stub: [usize; 4],
}

impl AstTu {
    pub fn new_dont_use() -> Self {
        Self { stub: [1, 2, 3, 4] }
    }
}
