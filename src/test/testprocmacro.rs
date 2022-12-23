use crate::ast::{common::CommonAst, Tu::AstTu};

#[test]
fn checkIfdef() {
    let ast = AstTu::new_dont_use();
    println!("{}", CommonAst::getDebugNode(&ast).to_string());
}
