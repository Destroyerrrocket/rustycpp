use crate::{
    ast::{common::AstDecl, Attribute::AstAttribute, Decl::Empty::AstEmptyDecl},
    utils::structs::SourceRange,
};

use super::super::Parser;

impl Parser {
    /**
     * empty-declaration | attribute-declaration
     */
    pub fn actOnEmptyDecl(
        &mut self,
        attr: Vec<&'static AstAttribute>,
        location: SourceRange,
    ) -> Vec<&'static AstDecl> {
        let ast = AstEmptyDecl::new(location, self.alloc().alloc_slice_clone(attr.as_slice()));
        return vec![self.alloc().alloc(AstDecl::AstEmptyDecl(ast))];
    }
}
