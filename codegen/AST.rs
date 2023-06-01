use crate::codegen::ClassRepresentation::Class;

const fn fin(name: &'static str) -> Class {
    Class {
        name,
        dependedBy: vec![],
        traits: vec![],
    }
}

const fn abs(name: &'static str, childs: Vec<Class>) -> Class {
    Class {
        name,
        dependedBy: childs,
        traits: vec![],
    }
}

const fn absTraits(name: &'static str, childs: Vec<Class>, traits: Vec<&'static str>) -> Class {
    Class {
        name,
        dependedBy: childs,
        traits,
    }
}

/*
Empty,
Asm,
Namespace,
CustomRustyCppEnum,
UsingNamespace,
*/
pub fn getAST() -> Class {
    absTraits(
        "ASTNode",
        vec![abs(
            "Decl",
            vec![
                fin("Empty"),
                fin("Asm"),
                fin("Namespace"),
                fin("CustomRustyCppEnum"),
                fin("UsingNamespace"),
            ],
        )],
        vec!["CommonAst"],
    )
}
