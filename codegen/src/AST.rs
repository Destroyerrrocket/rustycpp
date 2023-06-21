use std::vec;

use crate::ClassRepresentation::{Class, FuncCustomCodegen};

const fn fin(name: &'static str) -> Class {
    Class {
        name,
        dependedBy: vec![],
        traits: vec![],
        customCodegen: vec![],
    }
}

const fn abs(name: &'static str, childs: Vec<Class>, traits: Vec<&'static str>) -> Class {
    Class {
        name,
        dependedBy: childs,
        traits,
        customCodegen: vec![],
    }
}

const fn absCustom(
    name: &'static str,
    childs: Vec<Class>,
    traits: Vec<&'static str>,
    f: Vec<FuncCustomCodegen>,
) -> Class {
    Class {
        name,
        dependedBy: childs,
        traits,
        customCodegen: f,
    }
}

fn generatorType(_: &Vec<&Class>, class: &Class) -> String {
    format!(
        "
impl Display for {} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        TypeAst::fmt(self, f)
    }}
}}
",
        class.name
    ) + if class.dependedBy.is_empty() {
        format!(
            "
impl Display for {}StructNode {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        TypeAst::fmt(self, f)
    }}
}}
",
            class.name
        ) + format!(
            "
impl TypeAst for {}StructNode {{
    fn getBaseType(&self) -> BaseType {{
        TypeAst::getBaseType(&self)
    }}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        TypeAst::fmt(&self, f)
    }}
}}
",
            class.name
        )
        .as_str()
    } else {
        String::new()
    }
    .as_str()
}

fn generatorCommon(parents: &Vec<&Class>, class: &Class) -> String {
    let gen = |dec: &str| {
        format!("impl CommonAst for {dec}{}StructNode {{\n", class.name)
            + format!("   fn getDebugNode(&self) -> DebugNode {{\n").as_str()
            + format!("       let nodes = self.base.getDebugNode();\n").as_str()
            + if parents.is_empty() {
                "       nodes\n".to_string()
            } else {
                format!(
                    "       nodes.add_children(self.parent.getDebugNode().getChilds().clone())\n"
                )
            }
            .as_str()
            + format!("    }}\n").as_str()
            + format!("}}\n").as_str()
    };
    gen("") + gen("&").as_str()
}

#[must_use]
pub fn getAST() -> Class {
    absCustom(
        "AstNode",
        vec![
            abs(
                "AstDecl",
                vec![
                    fin("AstDeclEmpty"),
                    fin("AstDeclAsm"),
                    fin("AstDeclNamespace"),
                    fin("AstDeclCustomRustyCppEnum"),
                    fin("AstDeclUsingNamespace"),
                ],
                vec![],
            ),
            fin("AstAttribute"),
            abs(
                "AstAttributeCXX",
                vec![
                    fin("AstAttributeCXXRustyCppUnused"),
                    fin("AstAttributeCXXRustyCppCheckSymbolMatchTag"),
                    fin("AstAttributeCXXRustyCppTagDecl"),
                ],
                vec!["CXXAttribute"],
            ),
            absCustom(
                "AstType",
                vec![fin("AstTypeBuiltin")],
                vec!["TypeAst"],
                vec![&generatorType],
            ),
            fin("AstTu"),
        ],
        vec!["CommonAst"],
        vec![&generatorCommon],
    )
}
