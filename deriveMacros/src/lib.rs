#![feature(proc_macro_span)]
#![allow(
    non_snake_case,
    clippy::missing_const_for_fn // Bugged
)]

#[allow(unused_imports)]
use quote::{quote, ToTokens};

mod RustycppInheritance;

mod DeriveCommonAst;

use crate::DeriveCommonAst::impl_CommonAst;
use crate::RustycppInheritance::impl_RustycppInheritanceConstructors;

macro_rules! finalResult {
    (debugerr $genTs:expr) => {{
        let res = $genTs.to_string().to_token_stream();
        quote!(compile_error!(#res)).into()
    }};
    (release $genTs:expr) => {{
        $genTs.into()
    }};
}

#[proc_macro_derive(
    CommonAst,
    attributes(AstChild, AstChildSlice, AstChildSliceCell, AstToString)
)]
pub fn CommonAst_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    finalResult!(release impl_CommonAst(&ast))
}

#[proc_macro_attribute]
pub fn RustycppInheritanceConstructors(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::ItemImpl = syn::parse(item).unwrap();
    // Build the trait implementation
    finalResult!(release impl_RustycppInheritanceConstructors(&ast))
}
