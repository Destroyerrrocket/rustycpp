use deriveMacros::CommonAst;

use crate::Ast::Attribute::AstAttributeCXXStruct;
use crate::Ast::Attribute::AstAttributeStruct;
use crate::Ast::Attribute::CXXAttribute;
use crate::Ast::Attribute::RustyCppCheckSymbolMatchTag::AstAttributeCXXRustyCppCheckSymbolMatchTagStruct;
use crate::Ast::Attribute::RustyCppTagDecl::AstAttributeCXXRustyCppTagDeclStruct;
use crate::Ast::Attribute::RustyCppUnused::AstAttributeCXXRustyCppUnusedStruct;
use crate::Ast::Decl::Asm::AstDeclAsmStruct;
use crate::Ast::Decl::AstDeclStruct;
use crate::Ast::Decl::Empty::AstDeclEmptyStruct;
use crate::Ast::Decl::Enum::AstDeclCustomRustyCppEnumStruct;
use crate::Ast::Decl::Namespace::AstDeclNamespaceStruct;
use crate::Ast::Decl::UsingNamespace::AstDeclUsingNamespaceStruct;
use crate::Ast::Tu::AstTuStruct;
use crate::Ast::Type::AstTypeStruct;
use crate::Ast::Type::BaseType;
use crate::Ast::Type::Builtin::AstTypeBuiltinStruct;
use crate::Ast::Type::Pointer::AstTypePointerStruct;
use crate::Ast::Type::Reference::AstTypeReferenceStruct;
use crate::Ast::Type::TypeAst;
use crate::Utils::DebugNode::DebugNode;
use crate::Utils::FoldingContainer::Foldable;

use crate::Utils::FoldingContainer::FoldingNode;
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
            finalType: AstNodeFinalTypes::default(),
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
        <Self as $crate::Ast::Common::StructNodeTrait>::Base
    };
}

/// Access the Parent of this node. This is only useful from the *`StructNode`
/// family of classes. If there is no parent, this will return `()`
#[macro_export]
macro_rules! Parent {
    () => {
        <Self as $crate::Ast::Common::StructNodeTrait>::Parent
    };
}
