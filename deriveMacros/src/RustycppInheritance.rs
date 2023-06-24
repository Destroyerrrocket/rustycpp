use core::cell::UnsafeCell;
use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    spanned::Spanned, ImplItem, ImplItemMethod, ItemImpl, ReturnType, Token, Type, Visibility,
};

#[allow(dead_code)]
pub struct ClassInfo {
    class: &'static codegen::ClassRepresentation::Class,
    children: Vec<&'static codegen::ClassRepresentation::Class>,
    parents: Vec<&'static codegen::ClassRepresentation::Class>,
}

fn getMapOfNameToClassInfoRec(
    parents: &mut Vec<&'static codegen::ClassRepresentation::Class>,
    class: &'static codegen::ClassRepresentation::Class,
    map: &mut HashMap<String, ClassInfo>,
) {
    parents.push(class);
    for child in &class.dependedBy {
        getMapOfNameToClassInfoRec(parents, child, map);
    }
    parents.pop();

    let children = class
        .dependedBy
        .iter()
        .flat_map(|child| {
            let theirChilds = &map.get(child.name).unwrap().children;
            if theirChilds.is_empty() {
                vec![child]
            } else {
                theirChilds.clone()
            }
        })
        .collect::<Vec<_>>();

    let name = class.name.to_string();
    map.insert(
        name,
        ClassInfo {
            class,
            children,
            parents: parents.clone(),
        },
    );
}

thread_local! {
    pub static CLASSARCH: UnsafeCell<codegen::ClassRepresentation::Class> = UnsafeCell::new(codegen::AST::getAST());
    pub static MAP_OF_NAME_TO_CLASS_INFO: UnsafeCell<HashMap<String, ClassInfo>> = UnsafeCell::new(getMapOfNameToClassInfo());
}

fn getMapOfNameToClassInfo() -> HashMap<String, ClassInfo> {
    let mut map = HashMap::new();
    let classes: &'static codegen::ClassRepresentation::Class =
        CLASSARCH.with(|classes| unsafe { &*classes.get() });
    getMapOfNameToClassInfoRec(&mut Vec::new(), classes, &mut map);
    map
}

fn patternGetIdent(pattern: &syn::Pat) -> Option<&Ident> {
    match pattern {
        syn::Pat::Box(_) => None,
        syn::Pat::Ident(id) => Some(&id.ident),
        syn::Pat::Lit(_) => None,
        syn::Pat::Macro(_) => None,
        syn::Pat::Or(_) => None,
        syn::Pat::Path(_) => None,
        syn::Pat::Range(_) => None,
        syn::Pat::Reference(refPat) => patternGetIdent(refPat.pat.as_ref()),
        syn::Pat::Rest(_) => None,
        syn::Pat::Slice(_) => None,
        syn::Pat::Struct(_) => None,
        syn::Pat::Tuple(_) => None,
        syn::Pat::TupleStruct(_) => None,
        syn::Pat::Type(_) => None,
        syn::Pat::Verbatim(_) => None,
        syn::Pat::Wild(_) => None,
        _ => None,
    }
}

fn getMap() -> &'static HashMap<String, ClassInfo> {
    MAP_OF_NAME_TO_CLASS_INFO.with(|map| unsafe { &*map.get() })
}
pub fn impl_RustycppInheritanceConstructors(ast: &ItemImpl) -> TokenStream {
    let name = &ast.self_ty;

    let shortName = syn::Ident::new(
        name.to_token_stream()
            .to_string()
            .trim_end_matches("StructNode"),
        ast.self_ty.as_ref().span(),
    );

    let classInfo = getMap().get(&shortName.to_string()).unwrap();

    if !classInfo.children.is_empty() {
        return quote! {{compile_error!("RustycppInheritanceConstructors is not supported for abstact classes")}};
    }

    let mut ast = ast.clone();

    let forwardFuncs = ast
        .items
        .iter_mut()
        .filter_map(|function: &mut ImplItem| match function {
            syn::ImplItem::Method(function) => {
                if !function.sig.ident.to_string().starts_with("new") {
                    return None;
                }
                let ReturnType::Type(_, otherName) = &function.sig.output else {
                    return None;
                };
                let Type::Path(otherName) = otherName.as_ref() else {
                    return None;
                };

                let otherName = otherName.to_token_stream().to_string();
                if name.to_token_stream().to_string() != otherName && "Self" != otherName {
                    return None;
                }

                if matches!(function.vis, Visibility::Public(_)) {
                    function.vis = syn::parse2(quote!(pub(self))).unwrap();
                }

                let mut foundSelf = false;

                let args = function
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        syn::FnArg::Receiver(_) => {
                            foundSelf = true;
                            None
                        }
                        syn::FnArg::Typed(t) => {
                            let nameMaybe = patternGetIdent(&t.pat);

                            let Some(name) = nameMaybe else {
                                let errMsg =
                                    "RustycppInheritanceConstructors does not support the argument "
                                        .to_string()
                                        + &t.to_token_stream().to_string();
                                return Some(quote!(compile_error!(#errMsg)));
                            };
                            Some(quote!(#name))
                        }
                    })
                    .collect::<Vec<_>>();

                if foundSelf {
                    return None;
                }

                let mut ourFunction: ImplItemMethod = function.clone();
                ourFunction.sig.inputs.insert(
                    0,
                    syn::parse2(quote!(allocator__rusycpp: &'static bumpalo::Bump)).unwrap(),
                );

                ourFunction.sig.output = ReturnType::Type(
                    Token!(->)(ourFunction.sig.output.span()),
                    Box::new(syn::parse2(quote!(#shortName)).unwrap()),
                );
                ourFunction.block.stmts.clear();
                ourFunction.vis = syn::parse2(quote!(pub)).unwrap();
                let funcName = &function.sig.ident;
                ourFunction.block.stmts.push(
                    syn::parse2::<syn::Stmt>(quote!({
                        let mut node = #name::#funcName(#(#args),*);
                        node.internalSetFinType(#name::INTERNAL_FIN_TYPE_TAG);
                        return allocator__rusycpp.alloc(node).into();
                    }))
                    .unwrap(),
                );

                Some(quote!(#ourFunction))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    quote!(
        #ast
        impl #shortName {
            #(#forwardFuncs)*
        }
    )
}
