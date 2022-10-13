use antlr_rust::token_stream::TokenStream;
use antlr_rust::TidExt;
use antlr_rust::{BaseParser, TidAble};

use crate::grammars::generated::maincpp::LocalTokenFactory;
use crate::grammars::generated::maincpplistener::mainCppListener;
use crate::grammars::generated::maincppparser::mainCppContextType;
use crate::grammars::generated::maincppparser::mainCppExt;
use crate::grammars::generated::maincppparser::Translation_unitContext;
use crate::lex::token::Token;

type BaseParserType<'input, I> = BaseParser<
    'input,
    mainCppExt<'input>,
    I,
    mainCppContextType,
    dyn mainCppListener<'input> + 'input,
>;

pub fn isFinal<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &recog.input.get(recog.input.index()).data.tokPos.tok {
        id == "final"
    } else {
        false
    }
}
pub fn isOverride<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    if let Token::Identifier(id) = &recog.input.get(recog.input.index()).data.tokPos.tok {
        id == "override"
    } else {
        false
    }
}

pub fn isOverride2<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    recog
        .ctx
        .as_ref()
        .unwrap()
        .clone()
        .downcast_rc::<Translation_unitContext<'input>>()
        .unwrap()
        .as_ref();
    if let Token::Identifier(id) = &recog.input.get(recog.input.index()).data.tokPos.tok {
        id == "override"
    } else {
        false
    }
}

pub fn isTypedefName<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isNamespaceName<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isNamespaceAlias<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isClassName<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isEnumName<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub fn isTemplateName<'input, I>(recog: &mut BaseParserType<'input, I>) -> bool
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    false
}

pub struct Scopes;

impl Scopes {
    pub fn new() -> Self {
        Self {}
    }
}
