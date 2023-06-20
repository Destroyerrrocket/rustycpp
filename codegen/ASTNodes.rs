use crate::codegen::ClassRepresentation::{Class, FuncCustomCodegen};

#[must_use]
fn getAllConcreteClassNames(classes: &Class) -> Vec<&str> {
    let mut concreteClasses = Vec::new();
    for class in &classes.dependedBy {
        if class.dependedBy.is_empty() {
            concreteClasses.push(class.name);
        } else {
            concreteClasses.append(&mut getAllConcreteClassNames(class));
        }
    }
    concreteClasses
}

fn getAllTraitsToImplement(classes: &[&Class], class: &Class) -> Vec<&'static str> {
    let mut traits = class.traits.clone();
    for class in classes.iter().rev() {
        for t in &class.traits {
            if !traits.contains(t) {
                traits.push(t);
            }
        }
    }
    traits
}

fn getAllCustomGenerators(classes: &[&Class], class: &Class) -> Vec<FuncCustomCodegen> {
    let mut code: Vec<FuncCustomCodegen> = class.customCodegen.clone();
    for class in classes.iter().rev() {
        code.extend(class.customCodegen.clone());
    }
    code
}

fn generateClassEnum(
    parents: &[&Class],
    class: &Class,
    name: &str,
    childNames: &Vec<&str>,
    output: &mut String,
) {
    output.push_str("#[derive(Clone, Copy)]\n");

    output.push_str("#[enum_dispatch(");
    for implementTrait in getAllTraitsToImplement(parents, class) {
        output.push_str(&format!("{implementTrait}, "));
    }
    output.push_str(")]\n");
    output.push_str(&format!("pub enum {name} {{\n"));

    for childName in childNames {
        output.push_str(&format!(
            "    {childName}(&'static {childName}StructNode),\n"
        ));
    }
    output.push_str("}\n");
}

fn generateStructNodes(
    parents: &Vec<&Class>,
    _: &Class,
    name: &str,
    _: &Vec<&str>,
    output: &mut String,
) {
    output.push_str("#[allow(dead_code)]\n");
    if parents.is_empty() {
        output.push_str(&format!(
            "pub type {name}StructNode = StructParent<{name}Struct>;\n"
        ));
    } else {
        output.push_str(&format!(
            "pub type {name}StructNode = StructNode<{}StructNode, {name}Struct>;\n",
            &parents.last().unwrap().name
        ));
    }

    output.push_str(&format!("impl StructNodeTrait for {name}StructNode {{\n"));
    output.push_str(&format!("    type Base = {name}Struct;\n"));
    if parents.is_empty() {
        output.push_str("    type Parent = ();\n");
    } else {
        output.push_str(&format!(
            "    type Parent = {}StructNode;\n",
            &parents.last().unwrap().name
        ));
    }
    output.push_str("}\n");
}

fn generateFroms(
    parents: &Vec<&Class>,
    class: &Class,
    name: &str,
    childNames: &Vec<&str>,
    output: &mut String,
) {
    if !parents.is_empty() {
        for parent in parents.iter() {
            let parentName = parent.name;

            let mut fromFunc = |dec: &str| {
                output.push_str(&format!("impl From<{dec}{name}> for {parentName} {{\n"));
                output.push_str(&format!("    fn from(value: {dec}{name}) -> Self {{\n"));
                output.push_str("        match value {\n");
                for childName in childNames {
                    output.push_str(&format!(
                        "            {name}::{childName}(node) => Self::{childName}(node),\n"
                    ));
                }
                output.push_str("        }\n");
                output.push_str("    }\n");
                output.push_str("}\n");
            };
            fromFunc("");
            fromFunc("&");

            if class.dependedBy.is_empty() {
                let mut fromFunc = |dec: &str| {
                    output.push_str(&format!(
                        "impl From<{dec} {name}StructNode> for {parentName} {{\n"
                    ));
                    output.push_str(&format!(
                        "    fn from(value: {dec} {name}StructNode) -> Self {{\n"
                    ));
                    output.push_str(&format!("        Self::{name}(value)\n"));
                    output.push_str("    }\n");
                    output.push_str("}\n");
                };
                fromFunc("&'static mut");
            }

            let mut tryFromFunc = |dec: &str| {
                output.push_str(&format!("impl TryFrom<{dec}{parentName}> for {name} {{\n"));
                output.push_str("    type Error = ();\n");
                output.push_str("    #[allow(unreachable_patterns)]\n");
                output.push_str(&format!(
                    "    fn try_from(value: {dec}{parentName}) -> Result<Self, Self::Error> {{\n"
                ));
                output.push_str("        match value {\n");
                for childName in childNames {
                    output.push_str(&format!(
                        "            {parentName}::{childName}(node) => Ok(Self::{childName}(node)),\n"
                    ));
                }
                output.push_str("            _ => Err(()),\n");
                output.push_str("        }\n");
                output.push_str("    }\n");
                output.push_str("}\n");
            };

            tryFromFunc("");
            tryFromFunc("&");
        }

        if class.dependedBy.is_empty() {
            let mut fromFunc = |dec: &str| {
                output.push_str(&format!(
                    "impl From<{dec} {name}StructNode> for {name} {{\n"
                ));
                output.push_str(&format!(
                    "    fn from(value: {dec} {name}StructNode) -> Self {{\n"
                ));
                output.push_str(&format!("        Self::{name}(value)\n"));
                output.push_str("    }\n");
                output.push_str("}\n");
            };
            fromFunc("&'static mut");
        }
    }
}

fn generateDerefs(
    parents: &Vec<&Class>,
    _: &Class,
    name: &str,
    childNames: &Vec<&str>,
    output: &mut String,
) {
    if !parents.is_empty() {
        let parentName = &parents.last().unwrap().name;
        output.push_str(&format!("impl Deref for {name}StructNode {{\n"));
        output.push_str(&format!("    type Target = {parentName}StructNode;\n"));
        output.push_str("    fn deref(&self) -> &Self::Target {\n");
        output.push_str("        &self.parent\n");
        output.push_str("    }\n");
        output.push_str("}\n");
    }

    output.push_str(&format!("impl Deref for {name} {{\n"));
    output.push_str(&format!("    type Target = {name}StructNode;\n"));
    output.push_str("    fn deref(&self) -> &Self::Target {\n");
    output.push_str("        match self {\n");
    for childName in childNames {
        output.push_str(&format!("            {name}::{childName}(node) => node,\n"));
    }
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n");
}

fn generateClass(parents: &Vec<&Class>, class: &Class, output: &mut String) {
    // A concrete type!
    let isConcrete: bool = class.dependedBy.is_empty();
    let name: &str = class.name;
    let childNames: Vec<&str> = if isConcrete {
        vec![name]
    } else {
        getAllConcreteClassNames(class)
    };

    /*
    #[enum_dispatch(Trait1, Trait2)]
    enum Expr {
        Expr(&'static ExprStructNode),
    }
    */
    generateClassEnum(parents, class, name, &childNames, output);

    /*
    struct ExprStruct; // user provided
    struct ExprStructNode {
        parent: StmtStructNode,
        base: ExprStruct,
    }
    impl StructNodeTrait for ExprStructNode {
        type Base = ExprStruct;
        type Parent = ();
    }
    */
    generateStructNodes(parents, class, name, &childNames, output);

    /*
    impl From<Expr> for Stmt {
        fn from(value: Expr) -> Self {
            match value {
                Expr::Expr(node) => Stmt::Expr(node),
                Expr::Expr2(node) => Stmt::Expr2(node),
            }
        }
    }

    impl TryFrom<Stmt> for Expr {
        type Error = ();

        fn try_from(value: Stmt) -> Result<Self, Self::Error> {
            match value {
                Stmt::Expr(node) => Ok(Expr::Expr(node)),
                Stmt::Expr2(node) => Ok(Expr::Expr2(node)),
                _ => Err(()),
            }
        }
    }
    */
    generateFroms(parents, class, name, &childNames, output);

    /*
    impl Deref for ExprStructNode {
        type Target = StmtStructNode;

        fn deref(&self) -> &Self::Target {
            &self.parent
        }
    }

    impl Deref for Expr {
        type Target = ExprStructNode;

        fn deref(&self) -> &Self::Target {
            match self {
                Self::Expr(node) => node,
                Self::Expr2(node) => node,
            }
        }
    }
    */
    generateDerefs(parents, class, name, &childNames, output);

    for gen in getAllCustomGenerators(parents, class) {
        output.push_str(gen(parents, class).as_str());
    }
}

pub fn generate<'classes>(
    parents: &mut Vec<&'classes Class>,
    class: &'classes Class,
    output: &mut String,
) {
    generateClass(parents, class, output);
    parents.push(class);
    for child in &class.dependedBy {
        generate(parents, child, output);
    }
    parents.pop();
}

#[must_use]
pub fn generateFile(classes: &Class) -> String {
    let mut output = String::new();
    output.push_str(
        "
use std::ops::Deref;
use enum_dispatch::enum_dispatch;

pub struct StructParent<Base> {
    pub base: Base,
}

pub struct StructNode<Parent, Base> {
    pub parent: Parent,
    pub base: Base,
}

pub trait StructNodeTrait {
    type Base;
    type Parent;
}
",
    );
    generate(&mut vec![], classes, &mut output);
    output
}
