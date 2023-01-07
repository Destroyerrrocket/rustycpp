#![feature(proc_macro_span)]
#![allow(
    non_snake_case,
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

fn impl_AstChild(_: &Field, fieldName: TokenStream) -> TokenStream {
    quote! {
        add_child(&self.#fieldName.getDebugNode())
    }
}

fn impl_AstChildSlice(_: &Field, fieldName: TokenStream) -> TokenStream {
    quote! {
        add_children(self.#fieldName.iter().map(|x| x.getDebugNode()).collect::<_>())
    }
}

fn impl_AstChildSliceCell(_: &Field, fieldName: TokenStream) -> TokenStream {
    quote! {

        add_children(unsafe {self.#fieldName.get().as_ref().unwrap()}.iter().map(|x| x.getDebugNode()).collect::<_>())
    }
}

fn impl_CommonAst(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics.to_token_stream();
    match &ast.data {
        syn::Data::Struct(structData) => {
            let fields = &structData.fields;
            match fields {
                syn::Fields::Named(fields) => {
                    let mut vecTypes = vec![];
                    for field in fields.named.iter() {
                        let fieldName = field.ident.as_ref().unwrap().to_token_stream();
                        if hasAttribute!(field, AstToString) {
                            vecTypes.push(impl_AstToString(field, fieldName));
                        } else if hasAttribute!(field, AstChild) {
                            vecTypes.push(impl_AstChild(field, fieldName));
                        } else if hasAttribute!(field, AstChildSlice) {
                            vecTypes.push(impl_AstChildSlice(field, fieldName));
                        } else if hasAttribute!(field, AstChildSliceCell) {
                            vecTypes.push(impl_AstChildSliceCell(field, fieldName));
                        }
                    }
                    quote! {
                        impl #generics crate::ast::common::CommonAst for #name #generics {
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
                    impl crate::ast::common::CommonAst for #name #generics {
                        fn getDebugNode(&self) -> crate::utils::debugnode::DebugNode {
                            crate::utils::debugnode::DebugNode::new(stringify!(#name).to_string())
                        }
                    }
                },
            }
        }
        syn::Data::Enum(enumy) => {
            let arms = enumy.variants.iter().map(|variant| {
                let vident = &variant.ident;
                quote!(#name::#vident(v) => v.getDebugNode())
            });
            quote!(
                impl crate::ast::common::CommonAst for #name {
                fn getDebugNode(&self) -> crate::utils::debugnode::DebugNode {
                    match self {
                        #(#arms),*
                    }
                }
                }
            )
        }
        syn::Data::Union(_) => quote!(compile_error!("Can't derive CommonAst for union")),
    }
}

fn impl_DeclAst(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics.to_token_stream();
    match &ast.data {
        syn::Data::Struct(structData) => {
            let fields = &structData.fields;
            match fields {
                syn::Fields::Named(fields) => {
                    let field = fields
                        .named
                        .iter()
                        .find(|field| field.ty.to_token_stream().to_string() == "BaseDecl");
                    if let Some(field) = field {
                        let field = field.ident.to_token_stream();
                        quote! {
                            impl #generics crate::ast::Decl::DeclAst for #name {
                                fn getBaseDecl(&self) -> &crate::ast::Decl::BaseDecl {
                                    return &self.#field;
                                }
                            }
                        }
                    } else {
                        quote!(compile_error!(
                            "Can't implement DeclAst as there is no field with type BaseDecl"
                        ))
                    }
                }
                syn::Fields::Unnamed(_) => {
                    quote!(compile_error!("Can't derive DeclAst for tuple struct"))
                }
                syn::Fields::Unit => {
                    quote!(compile_error!("Can't derive DeclAst for unit struct"))
                }
            }
        }
        syn::Data::Enum(_) => quote!(compile_error!("Can't derive DeclAst for enum")),
        syn::Data::Union(_) => quote!(compile_error!("Can't derive DeclAst for union")),
    }
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

#[proc_macro_derive(DeclAst, attributes())]
pub fn DeclAst_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    finalResult!(release impl_DeclAst(&ast))
}
