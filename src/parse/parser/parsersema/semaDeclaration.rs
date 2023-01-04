use crate::{
    ast::{
        common::AstDecl,
        Attribute::AstAttribute,
        Decl::{Asm::AstAsmDecl, Empty::AstEmptyDecl},
    },
    utils::{stringref::StringRef, structs::SourceRange},
};

use super::super::Parser;

impl Parser {
    /**
     * empty-declaration | attribute-declaration
     */
    pub fn actOnEmptyDecl(
        &mut self,
        attr: &Vec<&'static AstAttribute>,
        location: SourceRange,
    ) -> Vec<&'static AstDecl> {
        let ast = AstEmptyDecl::new(location, self.alloc().alloc_slice_clone(attr.as_slice()));
        return vec![self.alloc().alloc(AstDecl::AstEmptyDecl(ast))];
    }

    /**
     * asm-declaration
     */
    pub fn actOnAsmDecl(
        &mut self,
        attr: &Vec<&'static AstAttribute>,
        location: SourceRange,
        asm: StringRef,
    ) -> Vec<&'static AstDecl> {
        let astAsm = AstAsmDecl::new(
            location,
            self.alloc().alloc_slice_clone(attr.as_slice()),
            asm,
        );
        return vec![self.alloc().alloc(AstDecl::AstAsmDecl(astAsm))];
    }
}
