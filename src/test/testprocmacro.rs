use std::rc::Rc;

use crate::{
    ast::{common::CommonAst, Tu::AstTu},
    utils::unsafeallocator::UnsafeAllocator,
};

#[test]
fn checkIfdef() {
    let ast = AstTu::new(Rc::new(UnsafeAllocator::new()), &[]);
    println!("{}", CommonAst::getDebugNode(&ast).to_string());
}
