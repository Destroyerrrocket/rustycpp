use crate::ast::Tu::AstTu;

use super::Parser;

impl Parser {
    pub fn parseTu(&mut self) -> AstTu {
        let _ast = AstTu::new_dont_use();
        return AstTu::new_dont_use();
    }

    fn topLevelDecl(&mut self) -> Option<()> {
        return None;
    }
}
