#![allow(
    non_snake_case,
    dead_code,
    clippy::needless_return,
    clippy::redundant_else,
    clippy::manual_assert,
    clippy::needless_pass_by_value,
    clippy::missing_const_for_fn // Bugged
)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Field};

macro_rules! hasAttribute {
    ($field:ident, $attr:ident) => {
        $field
            .attrs
            .iter()
            .find(|at| at.path.is_ident(stringify!($attr)))
            .is_some()
    };
}

macro_rules! finalResult {
    (debugerr $genTs:expr) => {{
        let res = $genTs.to_string().to_token_stream();
        quote!(compile_error!(#res)).into()
    }};
    (release $genTs:expr) => {{
        $genTs.into()
    }};
}

fn impl_AstToString(_: &Field, fieldName: TokenStream) -> TokenStream {
    quote! {
        add_child(crate::utils::debugnode::DebugNode::new(concat!(stringify!(#fieldName), ": ").to_string() + &self.#fieldName.to_string()))
    }
}

fn impl_CommonAst(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match &ast.data {
        syn::Data::Struct(structData) => {
            let fields = &structData.fields;
            match fields {
                syn::Fields::Named(fields) => {
                    let mut vecTypes = vec![];
                    for field in fields.named.iter() {
                        let fieldName = field.ident.as_ref().unwrap().to_token_stream();
                        if hasAttribute!(field, AstToString) {
                            vecTypes.push(impl_AstToString(field, fieldName));
                        }
                    }
                    quote! {
                        impl crate::ast::common::CommonAst for #name {
                            fn getDebugNode(&self) -> crate::utils::debugnode::DebugNode {
                                crate::utils::debugnode::DebugNode::new(stringify!(#name).to_string())
                                #(. #vecTypes)*
                            }
                        }
                    }
                }
                syn::Fields::Unnamed(_) => {
                    quote!(compile_error!("Can't derive CommonAst for tuple struct"))
                }
                syn::Fields::Unit => quote! {
                    impl crate::ast::common::CommonAst for #name {
                        fn getDebugNode(&self) -> crate::utils::debugnode::DebugNode {
                            crate::utils::debugnode::DebugNode::new(stringify!(#name).to_string())
                        }
                    }
                },
            }
        }
        syn::Data::Enum(_) => quote!(compile_error!("Can't derive CommonAst for enum")),
        syn::Data::Union(_) => quote!(compile_error!("Can't derive CommonAst for union")),
    };
    gen.into()
}

#[proc_macro_derive(CommonAst, attributes(AstChild, AstToString))]
pub fn CommonAst_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    finalResult!(release impl_CommonAst(&ast))
}
