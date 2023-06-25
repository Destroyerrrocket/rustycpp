use crate::{
    Ast::{
        Common::{AstTypeReference, AstTypeReferenceStructNode},
        Type::{BaseType, QualType, TypeAst},
    },
    Base, Parent,
    Utils::FoldingContainer::{Foldable, FoldingNode, PushFoldingNode},
};
use deriveMacros::{CommonAst, RustycppInheritanceConstructors};
use std::fmt::Display;

#[derive(CommonAst)]
pub struct AstTypeReferenceStruct {
    #[AstChild]
    base: QualType,
}

impl Display for AstTypeReferenceStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} *", self.base)
    }
}

impl AstTypeReferenceStruct {
    pub const fn new(base: QualType) -> Self {
        Self { base }
    }
}

#[RustycppInheritanceConstructors]
impl AstTypeReferenceStructNode {
    pub fn new(base: QualType) -> Self {
        Self {
            base: <Base!()>::new(base),
            parent: <Parent!()>::new(),
        }
    }
}

impl TypeAst for &AstTypeReferenceStructNode {
    fn getBaseType(&self) -> BaseType {
        BaseType { size: 8, align: 8 }
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)
    }
}

impl Foldable for &AstTypeReferenceStructNode {
    fn foldNode(&self, node: &mut FoldingNode) {
        node.push(&self.base.base);
    }
}
