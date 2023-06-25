use crate::{
    Ast::Common::{AstTypePointer, AstTypeReference},
    Utils::FoldingContainer::{Foldable, FoldingNode},
};
use std::collections::HashMap;

use bumpalo::Bump;

use crate::Ast::{
    Common::AstTypeBuiltin,
    Type::{Builtin::BuiltinTypeKind, QualType},
};

pub struct TypeDict {
    builtin: Vec<AstTypeBuiltin>,
    pointer: HashMap<FoldingNode, AstTypePointer>,
    lvalueReference: HashMap<FoldingNode, AstTypeReference>,
    alloc: &'static bumpalo::Bump,
}

impl TypeDict {
    pub fn new(alloc: &'static bumpalo::Bump) -> Self {
        Self {
            builtin: Vec::new(),
            pointer: HashMap::new(),
            lvalueReference: HashMap::new(),
            alloc,
        }
    }

    const fn alloc(&self) -> &'static Bump {
        self.alloc
    }

    pub fn addBuiltinType(&mut self, t: BuiltinTypeKind) {
        assert!(t as usize == self.builtin.len());
        self.builtin.push(AstTypeBuiltin::new(self.alloc(), t));
    }

    pub fn getBuiltinType(&self, t: BuiltinTypeKind) -> AstTypeBuiltin {
        self.builtin[t as usize]
    }

    pub fn getPtrType(&mut self, t: QualType) -> AstTypePointer {
        let nodeId = t.newFoldNode();
        let alloc = self.alloc();
        *self
            .pointer
            .entry(nodeId)
            .or_insert_with(|| AstTypePointer::new(alloc, t))
    }

    pub fn getLValueReference(&mut self, t: QualType) -> AstTypeReference {
        let nodeId = t.newFoldNode();
        let alloc = self.alloc();
        *self
            .lvalueReference
            .entry(nodeId)
            .or_insert_with(|| AstTypeReference::new(alloc, t))
    }
}
