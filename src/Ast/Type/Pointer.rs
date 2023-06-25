use crate::Utils::FoldingContainer::PushFoldingNode;
use crate::{
    Ast::{
        Common::{AstTypePointer, AstTypePointerStructNode},
        Type::{BaseType, QualType, TypeAst},
    },
    Base, Parent,
    Utils::FoldingContainer::{Foldable, FoldingNode},
};
use deriveMacros::{CommonAst, RustycppInheritanceConstructors};
use std::fmt::Display;

#[derive(CommonAst)]
pub struct AstTypePointerStruct {
    #[AstChild]
    base: QualType,
}

impl Display for AstTypePointerStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} *", self.base)
    }
}

impl AstTypePointerStruct {
    pub const fn new(base: QualType) -> Self {
        Self { base }
    }
}

#[RustycppInheritanceConstructors]
impl AstTypePointerStructNode {
    pub fn new(base: QualType) -> Self {
        Self {
            base: <Base!()>::new(base),
            parent: <Parent!()>::new(),
        }
    }
}

impl TypeAst for &AstTypePointerStructNode {
    fn getBaseType(&self) -> BaseType {
        BaseType { size: 8, align: 8 }
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)
    }
}

impl Foldable for &AstTypePointerStructNode {
    fn foldNode(&self, node: &mut FoldingNode) {
        node.push(&self.base.base);
    }
}
