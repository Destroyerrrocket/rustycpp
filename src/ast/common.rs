use deriveMacros::CommonAst;

use crate::utils::debugnode::DebugNode;

use crate::ast::Attribute::rustyCppCheckSymbolMatchTag::AstAttributeCXXRustyCppCheckSymbolMatchTagStruct;
use crate::ast::Attribute::rustyCppTagDecl::AstAttributeCXXRustyCppTagDeclStruct;
use crate::ast::Attribute::rustyCppUnused::AstAttributeCXXRustyCppUnusedStruct;
use crate::ast::Attribute::AstAttributeCXXStruct;
use crate::ast::Attribute::AstAttributeStruct;
use crate::ast::Attribute::CXXAttribute;
use crate::ast::Decl::Asm::AstDeclAsmStruct;
use crate::ast::Decl::AstDeclStruct;
use crate::ast::Decl::Empty::AstDeclEmptyStruct;
use crate::ast::Decl::Enum::AstDeclCustomRustyCppEnumStruct;
use crate::ast::Decl::Namespace::AstDeclNamespaceStruct;
use crate::ast::Decl::UsingNamespace::AstDeclUsingNamespaceStruct;
use crate::ast::Tu::AstTuStruct;
use crate::ast::Type::AstTypeStruct;
use crate::ast::Type::BaseType;
use crate::ast::Type::Builtin::AstTypeBuiltinStruct;
use crate::ast::Type::TypeAst;
use std::fmt::Display;

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn foo(d: AstDecl) {
    d.getDebugNode();
}

#[derive(Clone, Copy, CommonAst)]
pub struct AstNodeStruct;

impl AstNodeStructNode {
    pub fn new() -> Self {
        Self {
            finalType : Default::default(),
            base: AstNodeStruct,
        }
    }
}

#[enum_dispatch]
pub trait CommonAst {
    fn getDebugNode(&self) -> DebugNode;
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for AstTu {}

/// Access the Base of this node. This is only useful from the *`StructNode`
/// family of classes
#[macro_export]
macro_rules! Base {
    () => {
        <Self as $crate::ast::common::StructNodeTrait>::Base
    };
}

/// Access the Parent of this node. This is only useful from the *`StructNode`
/// family of classes. If there is no parent, this will return `()`
#[macro_export]
macro_rules! Parent {
    () => {
        <Self as $crate::ast::common::StructNodeTrait>::Parent
    };
}
