use std::hash::Hash;
use std::hash::Hasher;
use std::{any::Any, collections::hash_map::DefaultHasher};

use enum_dispatch::enum_dispatch;

use crate::Utils::StringRef::StringRef;

#[derive(Hash, Eq, PartialEq, Default)]
pub struct FoldingNode {
    bytes: Vec<u8>,
}

impl FoldingNode {
    pub const fn new() -> Self {
        Self { bytes: vec![] }
    }
}

pub trait PushFoldingNode<T> {
    fn push(&mut self, param: &T);
}

#[enum_dispatch]
pub trait Foldable {
    fn foldNode(&self, node: &mut FoldingNode);

    fn newFoldNode(&self) -> FoldingNode {
        let mut node = FoldingNode::new();
        self.foldNode(&mut node);
        node
    }
}

impl<T: Foldable + 'static> PushFoldingNode<T> for FoldingNode {
    fn push(&mut self, param: &T) {
        let mut hash = DefaultHasher::new();
        param.type_id().hash(&mut hash);
        self.push(&hash.finish());
        param.foldNode(self);
    }
}

impl PushFoldingNode<u8> for FoldingNode {
    fn push(&mut self, param: &u8) {
        self.bytes.push(*param);
    }
}

macro_rules! impl_push_folding_node {
    ($($t:ty),*) => {
        $(
            impl PushFoldingNode<$t> for FoldingNode {
                fn push(&mut self, param: &$t) {
                    self.bytes.extend_from_slice(&param.to_le_bytes());
                }
            }
        )*
    };
}
impl_push_folding_node! {i8, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize, f32, f64}
impl PushFoldingNode<&str> for FoldingNode {
    fn push(&mut self, param: &&str) {
        self.bytes.extend_from_slice(param.as_bytes());
    }
}

impl PushFoldingNode<StringRef> for FoldingNode {
    fn push(&mut self, param: &StringRef) {
        let mut hasher = DefaultHasher::new();
        param.hash(&mut hasher);
        self.push(&hasher.finish());
    }
}
