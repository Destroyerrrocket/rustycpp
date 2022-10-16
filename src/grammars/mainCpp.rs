#[allow(non_upper_case_globals)]
use std::rc::Rc;

use super::generated::maincppparser::*;
use crate::grammars::generated::maincpplistener::mainCppListener;
use crate::lex::token::Token;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::Tree;
use antlr_rust::TidExt;
use antlr_rust::{BaseParser, TidAble};

type BaseParserType<'input, I> = BaseParser<
    'input,
    mainCppExt<'input>,
    I,
    mainCppContextType,
    dyn mainCppListener<'input> + 'input,
>;

pub fn isFinal<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        id == "final"
    } else {
        false
    }
}

pub fn isOverride<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        id == "override"
    } else {
        false
    }
}

pub fn isTypedefName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        id == "Pc"
    } else {
        false
    }
}

pub fn isNamespaceName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isNamespaceAlias<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isClassName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isEnumName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isTemplateName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn inTypedef<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.clone().unwrap();
    return match ctx.get_rule_index() {
        RULE_decl_specifier => ctx
            .downcast_rc::<Decl_specifierContext>()
            .map_or(true, |ctx| {
                ctx.get_parent()
                    .unwrap()
                    .downcast_ref::<Decl_specifier_seqContext>()
                    .map_or(false, |ctxDowncast| ctxDowncast.inTypedef)
            }),
        RULE_decl_specifier_seq => ctx
            .downcast_ref::<Decl_specifier_seqContext>()
            .map_or(true, |ctxDowncast| ctxDowncast.inTypedef),
        _ => false,
    };
}

pub fn validateTypedef<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.clone().unwrap();
    return match ctx.get_rule_index() {
        RULE_decl_specifier => ctx
            .downcast_rc::<Decl_specifierContext>()
            .map_or(false, |ctx| {
                let ctxParent = ctx.get_parent().unwrap();
                let ctxDowncast = ctxParent
                    .downcast_ref::<Decl_specifier_seqContext>()
                    .unwrap();
                let decl = ctxDowncast.decl_specifier_all();
                if decl.is_empty() {
                    return true;
                }
                Rc::ptr_eq(&ctx, decl.first().unwrap())
            }),
        RULE_decl_specifier_seq => ctx
            .downcast_ref::<Decl_specifier_seqContext>()
            .map_or(false, |ctxDowncast| {
                ctxDowncast.decl_specifier_all().is_empty()
            }),
        _ => true,
    };
}

pub fn enableTypedef<'input, I>(this: &mut BaseParserType<'input, I>)
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let parentctx = this.ctx.as_ref().unwrap().get_parent().unwrap();
    let parentctx = parentctx
        .downcast_ref::<Decl_specifier_seqContext>()
        .unwrap();
    unsafe {
        type T<'input> = Decl_specifier_seqContext<'input>;
        let parentctx = &mut *(parentctx as *const T as *mut T);
        parentctx.inTypedef = true;
    }
}

pub fn generalRuleOneDefiningTypeSpecifierDecl<'input, I>(
    this: &mut BaseParserType<'input, I>,
) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.clone().unwrap();

    if matches!(
        this.input.lt(1).unwrap().data.tokPos.tok,
        Token::Const
            | Token::Volatile
            | Token::Signed
            | Token::Unsigned
            | Token::Short
            | Token::Long
    ) {
        return true;
    }

    return match ctx.get_rule_index() {
        RULE_decl_specifier => ctx
            .downcast_rc::<Decl_specifierContext>()
            .map_or(false, |ctx| {
                let ctxParent = ctx.get_parent().unwrap();
                let ctxDowncast = ctxParent
                    .downcast_ref::<Decl_specifier_seqContext>()
                    .unwrap();
                let decl = ctxDowncast.decl_specifier_all();
                if decl.is_empty() || Rc::ptr_eq(&ctx, decl.first().unwrap()) {
                    return true;
                }

                return !ctxDowncast.foundHardDefiningTypeSpecifier;
            }),
        RULE_decl_specifier_seq => ctx
            .downcast_ref::<Decl_specifier_seqContext>()
            .map_or(false, |ctxDowncast| {
                !ctxDowncast.foundHardDefiningTypeSpecifier
            }),
        _ => true,
    };
}

pub fn generalRuleOneDefiningTypeSpecifierInSeq<'input, I>(
    this: &mut BaseParserType<'input, I>,
) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.clone().unwrap();

    if matches!(
        this.input.lt(1).unwrap().data.tokPos.tok,
        Token::Const
            | Token::Volatile
            | Token::Signed
            | Token::Unsigned
            | Token::Short
            | Token::Long
    ) {
        return true;
    }

    return match ctx.get_rule_index() {
        RULE_defining_type_specifier_seq_aux => ctx
            .downcast_rc::<Defining_type_specifier_seq_auxContext>()
            .map_or(false, |ctx| {
                let ctxParent = ctx.get_parent().unwrap();
                let ctxDowncast = ctxParent
                    .downcast_ref::<Defining_type_specifier_seqContext>()
                    .unwrap();
                let decl = ctxDowncast.defining_type_specifier_seq_aux_all();
                if decl.is_empty() || Rc::ptr_eq(&ctx, decl.first().unwrap()) {
                    return true;
                }

                return !ctxDowncast.foundHardDefiningTypeSpecifier;
            }),
        RULE_defining_type_specifier_seq => ctx
            .downcast_ref::<Defining_type_specifier_seqContext>()
            .map_or(false, |ctxDowncast| {
                !ctxDowncast.foundHardDefiningTypeSpecifier
            }),
        _ => true,
    };
}

pub fn generalRuleOneTypeSpecifierInSeq<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.clone().unwrap();

    if matches!(
        this.input.lt(1).unwrap().data.tokPos.tok,
        Token::Const
            | Token::Volatile
            | Token::Signed
            | Token::Unsigned
            | Token::Short
            | Token::Long
    ) {
        return true;
    }
    return match ctx.get_rule_index() {
        RULE_type_specifier_seq_aux => {
            ctx.downcast_rc::<Type_specifier_seq_auxContext>()
                .map_or(false, |ctx| {
                    let ctxParent = ctx.get_parent().unwrap();
                    let ctxDowncast = ctxParent
                        .downcast_ref::<Type_specifier_seqContext>()
                        .unwrap();
                    let decl = ctxDowncast.type_specifier_seq_aux_all();
                    if decl.is_empty() || Rc::ptr_eq(&ctx, decl.first().unwrap()) {
                        return true;
                    }

                    return !ctxDowncast.foundHardDefiningTypeSpecifier;
                })
        }
        RULE_type_specifier_seq => ctx
            .downcast_ref::<Type_specifier_seqContext>()
            .map_or(false, |ctxDowncast| {
                !ctxDowncast.foundHardDefiningTypeSpecifier
            }),
        _ => true,
    };
}

pub fn enableGeneralRuleOneDefiningTypeSpecifierDecl<'input, I>(
    this: &mut BaseParserType<'input, I>,
) where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.as_ref().unwrap().clone();

    let parentctx = ctx.get_parent().unwrap();
    let parentctx = parentctx
        .downcast_ref::<Decl_specifier_seqContext>()
        .unwrap();

    if parentctx.foundHardDefiningTypeSpecifier {
        return;
    }

    let ctx = ctx.downcast_ref::<Decl_specifierContext>().unwrap();
    let typeSpeficier = ctx
        .defining_type_specifier()
        .and_then(|ctx| ctx.type_specifier());
    let isCV = typeSpeficier
        .as_ref()
        .is_some_and(|ctx| ctx.cv_qualifier().is_some());
    let isSoft = isCV
        || typeSpeficier.is_some_and(|ctx| {
            ctx.simple_type_specifier().is_some_and(|ctx| {
                ctx.Signed().is_some()
                    || ctx.Unsigned().is_some()
                    || ctx.Short().is_some()
                    || ctx.Long().is_some()
            })
        });

    if !isCV {
        unsafe {
            type T<'input> = Decl_specifier_seqContext<'input>;
            let parentctx = &mut *(parentctx as *const T as *mut T);
            parentctx.foundHardDefiningTypeSpecifier |= !isSoft;
            parentctx.foundAnyDefiningTypeSpecifierOtherThanCVQualifier = true;
        }
    }
}

pub fn enableGeneralRuleOneDefiningTypeSpecifierInSeq<'input, I>(
    this: &mut BaseParserType<'input, I>,
) where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.as_ref().unwrap().clone();

    let parentctx = ctx.get_parent().unwrap();
    let parentctx = parentctx
        .downcast_ref::<Defining_type_specifier_seqContext>()
        .unwrap();

    if parentctx.foundHardDefiningTypeSpecifier {
        return;
    }

    let ctx = parentctx
        .defining_type_specifier_seq_aux_all()
        .last()
        .unwrap()
        .clone();

    let typeSpeficier = ctx
        .defining_type_specifier()
        .and_then(|ctx| ctx.type_specifier());
    let isSoft = typeSpeficier.is_some_and(|ctx| {
        ctx.cv_qualifier().is_some()
            || ctx.simple_type_specifier().is_some_and(|ctx| {
                ctx.Signed().is_some()
                    || ctx.Unsigned().is_some()
                    || ctx.Short().is_some()
                    || ctx.Long().is_some()
            })
    });

    if !isSoft {
        unsafe {
            type T<'input> = Defining_type_specifier_seqContext<'input>;
            let parentctx = &mut *(parentctx as *const T as *mut T);
            parentctx.foundHardDefiningTypeSpecifier = true;
        }
    }
}

pub fn enableGeneralRuleOneTypeSpecifierInSeq<'input, I>(this: &mut BaseParserType<'input, I>)
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.as_ref().unwrap().clone();

    let parentctx = ctx.get_parent().unwrap();
    let parentctx = parentctx
        .downcast_ref::<Type_specifier_seqContext>()
        .unwrap();

    if parentctx.foundHardDefiningTypeSpecifier {
        return;
    }

    let ctx = parentctx
        .type_specifier_seq_aux_all()
        .last()
        .unwrap()
        .clone();

    let typeSpeficier = ctx.type_specifier();
    let isSoft = typeSpeficier.is_some_and(|ctx| {
        ctx.cv_qualifier().is_some()
            || ctx.simple_type_specifier().is_some_and(|ctx| {
                ctx.Signed().is_some()
                    || ctx.Unsigned().is_some()
                    || ctx.Short().is_some()
                    || ctx.Long().is_some()
            })
    });

    if !isSoft {
        unsafe {
            type T<'input> = Type_specifier_seqContext<'input>;
            let parentctx = &mut *(parentctx as *const T as *mut T);
            parentctx.foundHardDefiningTypeSpecifier = true;
        }
    }
}

pub fn inDeclTypeSpeficierSeqButValid<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.clone().unwrap();

    return match ctx.get_rule_index() {
        RULE_decl_specifier_seq => {
            ctx.downcast_ref::<Decl_specifier_seqContext>()
                .map_or(false, |ctx| {
                    return !ctx.foundAnyDefiningTypeSpecifierOtherThanCVQualifier;
                })
        }
        RULE_decl_specifier => ctx
            .downcast_rc::<Decl_specifierContext>()
            .map_or(false, |ctx| {
                ctx.get_parent()
                    .unwrap()
                    .downcast_ref::<Decl_specifier_seqContext>()
                    .map_or(false, |ctx| {
                        return !ctx.foundAnyDefiningTypeSpecifierOtherThanCVQualifier;
                    })
            }),
        RULE_defining_type_specifier => {
            ctx.downcast_rc::<Defining_type_specifierContext>()
                .map_or(false, |ctx| {
                    ctx.get_parent()
                        .unwrap()
                        .downcast_rc::<Decl_specifierContext>()
                        .map_or(true, |ctx| {
                            ctx.get_parent()
                                .unwrap()
                                .downcast_ref::<Decl_specifier_seqContext>()
                                .map_or(false, |ctx| {
                                    return !ctx.foundAnyDefiningTypeSpecifierOtherThanCVQualifier;
                                })
                        })
                })
        }
        RULE_type_specifier => ctx
            .downcast_rc::<Type_specifierContext>()
            .map_or(false, |ctx| {
                ctx.get_parent()
                    .unwrap()
                    .downcast_rc::<Defining_type_specifierContext>()
                    .map_or(true, |ctx| {
                        ctx.get_parent()
                            .unwrap()
                            .downcast_rc::<Decl_specifierContext>()
                            .map_or(true, |ctx| {
                                ctx.get_parent()
                                    .unwrap()
                                    .downcast_ref::<Decl_specifier_seqContext>()
                                    .map_or(false, |ctx| {
                                        return !ctx
                                            .foundAnyDefiningTypeSpecifierOtherThanCVQualifier;
                                    })
                            })
                    })
            }),
        RULE_simple_type_specifier => ctx
            .get_parent()
            .unwrap()
            .downcast_rc::<Type_specifierContext>()
            .map_or(true, |ctx| {
                ctx.get_parent()
                    .unwrap()
                    .downcast_rc::<Defining_type_specifierContext>()
                    .map_or(true, |ctx| {
                        ctx.get_parent()
                            .unwrap()
                            .downcast_rc::<Decl_specifierContext>()
                            .map_or(true, |ctx| {
                                ctx.get_parent()
                                    .unwrap()
                                    .downcast_ref::<Decl_specifier_seqContext>()
                                    .map_or(false, |ctx| {
                                        return !ctx
                                            .foundAnyDefiningTypeSpecifierOtherThanCVQualifier;
                                    })
                            })
                    })
            }),
        _ => true,
    };
}

pub struct Scopes;
impl Scopes {
    pub fn new() -> Self {
        Self {}
    }
}
