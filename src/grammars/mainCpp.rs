use std::cell::RefCell;
#[allow(non_upper_case_globals)]
use std::rc::Rc;

use super::generated::maincppparser::*;
use crate::grammars::generated::maincpplistener::mainCppListener;
use crate::lex::token::Token;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::Tree;
use antlr_rust::TidExt;
use antlr_rust::{BaseParser, TidAble};
use lazy_static::__Deref;

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
        let scope = this.deref().s.as_ref().borrow();
        for scope in scope.current.borrow().scopes.iter() {
            let scope = scope.borrow();
            if scope.typeOf == ScopeType::Typedef {
                if id == &scope.identifier {
                    return true;
                }
            }
        }
    }
    false
}

pub fn isNamespaceName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        let scope = this.deref().s.as_ref().borrow();
        for scope in scope.current.borrow().scopes.iter() {
            let scope = scope.borrow();
            if scope.typeOf == ScopeType::Namespace {
                if id == &scope.identifier {
                    return true;
                }
            }
        }
    }
    false
}

pub fn isNamespaceAlias<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        let scope = this.deref().s.as_ref().borrow();
        for scope in scope.current.borrow().scopes.iter() {
            let scope = scope.borrow();
            if scope.typeOf == ScopeType::NamespaceAlias {
                if id == &scope.identifier {
                    return true;
                }
            }
        }
    }
    false
}

pub fn isClassName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        let scope = this.deref().s.as_ref().borrow();
        for scope in scope.current.borrow().scopes.iter() {
            let scope = scope.borrow();
            if scope.typeOf == ScopeType::Class {
                if id == &scope.identifier {
                    return true;
                }
            }
        }
    }
    false
}

pub fn isEnumName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        let scope = this.deref().s.as_ref().borrow();
        for scope in scope.current.borrow().scopes.iter() {
            let scope = scope.borrow();
            if scope.typeOf == ScopeType::Enum {
                if id == &scope.identifier {
                    return true;
                }
            }
        }
    }
    false
}

pub fn isTemplateName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        let mut hasLessNext = this.input.get(this.input.index() + 1).data.tokPos.tok == Token::Less;
        let scope = this.deref().s.as_ref().borrow();
        for scope in scope.current.borrow().scopes.iter() {
            let scope = scope.borrow();
            match &scope.typeOf {
                ScopeType::Function => {
                    if id == &scope.identifier {
                        if scope
                            .parent
                            .as_ref()
                            .is_some_and(|p| p.borrow().typeOf == ScopeType::Template)
                        {
                            return true;
                        } else {
                            return hasLessNext;
                        }
                    }
                }
                ScopeType::Concept | ScopeType::Class | ScopeType::Enum => {
                    if id == &scope.identifier {
                        if scope
                            .parent
                            .as_ref()
                            .is_some_and(|p| p.borrow().typeOf == ScopeType::Template)
                        {
                            return true;
                        }
                        return false;
                    }
                }
                _ => {
                    if id == &scope.identifier {
                        return false;
                    }
                }
            }
        }
    }
    false
}

pub fn isConceptName<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &this.input.get(this.input.index()).data.tokPos.tok {
        let scope = this.deref().s.as_ref().borrow();
        for scope in scope.current.borrow().scopes.iter() {
            let scope = scope.borrow();
            if scope.typeOf == ScopeType::Concept {
                if id == &scope.identifier {
                    return true;
                }
            }
        }
    }
    false
}

pub fn inTypedef<'input, I>(this: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this.ctx.clone().unwrap();
    #[allow(non_upper_case_globals)]
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
    #[allow(non_upper_case_globals)]
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

    #[allow(non_upper_case_globals)]
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

    #[allow(non_upper_case_globals)]
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
    #[allow(non_upper_case_globals)]
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

    #[allow(non_upper_case_globals)]
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

fn getNameOfUnestedSimpleDeclarator<'input>(
    declarator: Rc<DeclaratorContextAll<'input>>,
) -> Option<String> {
    let extractPtr = |mut ptrDecl: Rc<Ptr_declaratorContextAll<'input>>| loop {
        if let Some(noptr) = ptrDecl.noptr_declarator() {
            break noptr;
        }
        ptrDecl = ptrDecl.ptr_declarator().unwrap();
    };
    let extractNoPtr = |mut noptr: Rc<Noptr_declaratorContextAll<'input>>| loop {
        if let Some(declId) = noptr.declarator_id() {
            break declId.id_expression().unwrap();
        } else if let Some(ptrDecl) = noptr.ptr_declarator() {
            noptr = extractPtr(ptrDecl);
        }
        noptr = noptr.noptr_declarator().unwrap();
    };
    let noptr = if let Some(noptr) = declarator.noptr_declarator() {
        extractNoPtr(noptr)
    } else {
        extractNoPtr(extractPtr(declarator.ptr_declarator().unwrap()))
    };
    let unqualId = noptr.unqualified_id()?;
    unqualId
        .Identifier()
        .map(|id| id.symbol.data.tokPos.tok.to_string())
}

pub fn memberDeclarationDeclareTypedef<'input, I>(this: &mut BaseParserType<'input, I>)
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this
        .ctx
        .as_ref()
        .unwrap()
        .clone()
        .downcast_rc::<Member_declarationContext>()
        .unwrap()
        .clone();
    if ctx
        .decl_specifier_seq()
        .map_or(false, |declSpecifierSeq| declSpecifierSeq.inTypedef)
    {
        let scope = this.deref().s.as_ref().borrow();
        for declarator in ctx
            .member_declarator_list()
            .map_or(vec![], |declaratorList| {
                declaratorList.member_declarator_all()
            })
        {
            if let Some(identifier) = declarator
                .declarator()
                .and_then(|decl| getNameOfUnestedSimpleDeclarator(decl))
            {
                scope.addTypedef(identifier, ctx.clone());
            } else {
                unimplemented!("Only simple declarators are supported right now. No typedef int *A for example.");
            }
        }
    }
}

pub fn simpleDeclarationDeclareTypedef<'input, I>(this: &mut BaseParserType<'input, I>)
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let ctx = this
        .ctx
        .as_ref()
        .unwrap()
        .clone()
        .downcast_rc::<Simple_declarationContext>()
        .unwrap()
        .clone();
    if ctx
        .decl_specifier_seq()
        .map_or(false, |declSpecifierSeq| declSpecifierSeq.inTypedef)
    {
        let scope = this.deref().s.as_ref().borrow();
        for declarator in ctx.init_declarator_list().map_or(vec![], |declaratorList| {
            declaratorList.init_declarator_all()
        }) {
            if let Some(identifier) = declarator
                .declarator()
                .and_then(|d| getNameOfUnestedSimpleDeclarator(d))
            {
                scope.addTypedef(identifier, ctx.clone());
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ScopeType {
    Global,
    Function,
    Class,
    Enum,
    Namespace,
    NamespaceAlias,
    Typedef,
    Template,
    Object,
    Concept,
}

pub struct Scope<'a> {
    pub parent: Option<Rc<RefCell<Scope<'a>>>>,
    pub typeOf: ScopeType,
    pub identifier: String,
    pub scopes: Vec<Rc<RefCell<Scope<'a>>>>,
    pub unamedNamespaces: Vec<Rc<RefCell<Scope<'a>>>>,
    pub ctx: Option<Rc<dyn mainCppContext<'a>>>,
}

impl<'a> std::fmt::Debug for Scope<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parentScope = &self.parent.as_ref().map(|x| x.borrow());
        f.debug_struct("Scope")
            .field("identifier", &self.identifier)
            .field("parent", &parentScope.as_ref().map(|x| &x.identifier))
            .field("typeOf", &self.typeOf)
            .field("scopes", &self.scopes)
            .field("unamedNamespaces", &self.unamedNamespaces)
            .field("ctx", &self.ctx)
            .finish()
    }
}

pub struct Scopes<'a> {
    pub global: Rc<RefCell<Scope<'a>>>,
    pub current: Rc<RefCell<Scope<'a>>>,
}

impl<'a> std::fmt::Debug for Scopes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scopes")
            .field("global", &self.global)
            .field("current", &self.current.borrow().identifier)
            .finish()
    }
}

impl<'a> Default for Scopes<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> Scopes<'a> {
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(Scope::<'a> {
            parent: None,
            typeOf: ScopeType::Global,
            identifier: String::new(),
            scopes: vec![],
            unamedNamespaces: vec![],
            ctx: None,
        }));
        Self {
            global: global.clone(),
            current: global,
        }
    }

    pub fn addTypedef(&self, name: String, ctx: Rc<dyn mainCppContext<'a>>) {
        let newScope = Rc::new(RefCell::new(Scope::<'a> {
            parent: Some(self.current.clone()),
            typeOf: ScopeType::Typedef,
            identifier: name,
            scopes: vec![],
            unamedNamespaces: vec![],
            ctx: Some(ctx),
        }));
        (*self.current).borrow_mut().scopes.push(newScope.clone());
    }
}
