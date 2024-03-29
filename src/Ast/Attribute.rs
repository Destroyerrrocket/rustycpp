use crate::{Ast::Common::AstAttribute, Ast::Common::AstAttributeCXXStructNode, Base, Parent};
use std::collections::HashMap;

use deriveMacros::{CommonAst, RustycppInheritanceConstructors};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;

use crate::{
    Ast::Common::AstAttributeCXX,
    Ast::Common::{AstAttributeStructNode, CommonAst},
    Lex::Token::Token,
    Utils::Structs::{FileTokPos, SourceRange},
};
use crate::{Parse::BufferedLexer::StateBufferedLexer, Utils::StringRef::StringRef};

pub mod RustyCppUnused;
use RustyCppUnused::AstAttributeCXXRustyCppUnusedStruct;
pub mod RustyCppTagDecl;
use RustyCppTagDecl::AstAttributeCXXRustyCppTagDeclStruct;
pub mod RustyCppCheckSymbolMatchTag;
use RustyCppCheckSymbolMatchTag::AstAttributeCXXRustyCppCheckSymbolMatchTagStruct;

#[derive(Clone, Copy)]
pub enum Kind {
    AlignAs,
    Cxx(&'static [AstAttributeCXX]),
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Self::AlignAs => "alignas".to_string(),
            Self::Cxx(_) => "CXX".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AstAttributeStruct {
    /// CXX11, alignas, etc.
    pub kind: Kind,
    /// The range of the attribute in the source code. Includes the brackets/alignas/etc.
    pub sourceRange: SourceRange,
}

impl super::Common::CommonAst for AstAttributeStruct {
    fn getDebugNode(&self) -> crate::Utils::DebugNode::DebugNode {
        match self.kind {
            Kind::AlignAs => crate::Utils::DebugNode::DebugNode::new("AstAttribute".to_string())
                .add_child(crate::Utils::DebugNode::DebugNode::new(
                    "Kind: Alignas".to_string(),
                )),
            Kind::Cxx(attrs) => crate::Utils::DebugNode::DebugNode::new("AstAttribute".to_string())
                .add_children(attrs.iter().map(CommonAst::getDebugNode).collect()),
        }
    }
}

impl AstAttributeStruct {
    pub const fn new(kind: Kind, sourceRange: SourceRange) -> Self {
        Self { kind, sourceRange }
    }
}

#[RustycppInheritanceConstructors]
impl AstAttributeStructNode {
    pub const fn getKind(&self) -> Kind {
        self.base.kind
    }

    pub const fn getSourceRange(&self) -> SourceRange {
        self.base.sourceRange
    }

    pub fn new(kind: Kind, sourceRange: SourceRange) -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::new(kind, sourceRange),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AtrributeKindInfo {
    pub namespace: Option<StringRef>,
    pub name: StringRef,
    pub requiresParameters: bool,
    pub parser: fn(
        &mut crate::Parse::Parser::Parser,
        &FileTokPos<Token>,
        Option<StateBufferedLexer>,
    ) -> Option<AstAttributeCXX>,
}

pub struct AttributeDispatcher {
    pub attributeKinds: HashMap<Option<StringRef>, HashMap<StringRef, AtrributeKindInfo>>,
}

impl AttributeDispatcher {
    pub fn getAtrributeKindInfo(
        &self,
        namespace: Option<StringRef>,
        name: StringRef,
    ) -> Option<&AtrributeKindInfo> {
        self.attributeKinds
            .get(&namespace)
            .and_then(|namespace| namespace.get(&name))
    }
}

#[derive(Clone, Copy, CommonAst, Default)]
pub struct AstAttributeCXXStruct;

pub trait CXXAttributeKindInfo {
    fn getAtrributeKindInfo() -> AtrributeKindInfo;
}

impl AstAttributeCXXStructNode {
    pub fn new() -> Self {
        Self {
            parent: <Parent!()>::new(),
            base: <Base!()>::default(),
        }
    }

    // I'm leaving this here until I have a real case for using getDyn()
    pub fn actOnAttributeDeclTestRemoveMePlease(
        &'static self,
        parser: &mut crate::Parse::Parser::Parser,
    ) {
        self.getDyn().actOnAttributeDecl(parser);
    }
}

#[enum_dispatch]
pub trait CXXAttribute {
    fn actOnAttributeDecl(&self, _parser: &mut crate::Parse::Parser::Parser) {}
}

macro_rules! register_attributes {
    ($($o:ident),*) => {
        register_attributes!(@ dispatcher $($o),*);
    };
    (@ dispatcher $($o:ident),*) => {
        lazy_static! {
            pub static ref ATTRIBUTE_DISPATCHER: AttributeDispatcher = {
                let mut attributeKinds: HashMap<Option<StringRef>, HashMap<StringRef, AtrributeKindInfo>> = HashMap::new();
                $({
                    let attrInfo = $o::getAtrributeKindInfo();
                    if let Some(namespace) = attributeKinds.get_mut(&attrInfo.namespace) {
                        namespace.insert(attrInfo.name, attrInfo);
                    } else {
                        let mut namespace = HashMap::new();
                        namespace.insert(attrInfo.name, attrInfo);
                        attributeKinds.insert(attrInfo.namespace, namespace);
                    }
                })*
                AttributeDispatcher { attributeKinds }
            };
        }
    };
}

register_attributes! {
    AstAttributeCXXRustyCppUnusedStruct,
    AstAttributeCXXRustyCppTagDeclStruct,
    AstAttributeCXXRustyCppCheckSymbolMatchTagStruct
}
