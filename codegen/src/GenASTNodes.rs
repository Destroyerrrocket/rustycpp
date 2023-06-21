use crate::ClassRepresentation::{Class, FuncCustomCodegen};

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

/*
#[enum_dispatch(Trait1, Trait2)]
enum Expr {
    Expr(&'static ExprStructNode),
}
*/
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
fn generateStructNodes(
    parents: &Vec<&Class>,
    _: &Class,
    name: &str,
    childNames: &Vec<&str>,
    output: &mut String,
) {
    output.push_str("#[allow(dead_code)]\n");
    if parents.is_empty() {
        /*
        pub enum NameFinalTypes
        */
        /*pub struct NameStructNode {
            pub type: NameFinalTypes,
            pub base: Base,
        }*/
        output.push_str("#[derive(Default)]\n");
        output.push_str(&format!("pub enum {name}FinalTypes {{\n"));
        output.push_str("    #[default]\n");
        output.push_str("    None,\n");
        for childName in childNames {
            output.push_str(&format!("    {childName},\n"));
        }
        output.push_str("}\n");
        output.push_str(&format!("pub struct {name}StructNode {{\n"));
        output.push_str(&format!("    pub finalType: {name}FinalTypes,\n"));
        output.push_str(&format!("    pub base: {name}Struct,\n"));
        output.push_str("}\n");

        output.push_str(&format!("impl {name}StructNode {{\n"));
        output.push_str(&format!(
            "    pub fn internalSetFinType(&mut self, finalType: {name}FinalTypes) -> () {{self.finalType = finalType;}}\n",
        ));
        output.push_str("}\n");
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

            output.push_str(&format!("impl {name} {{\n"));
            output.push_str(&format!(
                "    pub fn getStatic(&self) -> &'static {name}StructNode {{\n"
            ));
            output.push_str(&format!("        let Self::{name}(node) = self;\n"));
            output.push_str("        node\n");
            output.push_str("    }\n");
            output.push_str("}\n");
        }
    }
}

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

        output.push_str(&format!("impl DerefMut for {name}StructNode {{\n"));
        output.push_str("    fn deref_mut(&mut self) -> &mut Self::Target {\n");
        output.push_str("        &mut self.parent\n");
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

/*
// For base parent
pub const OFFSET_AstNodeStructNode: usize = unsafe {
    let c = std::mem::MaybeUninit::uninit();
    let c_ptr: *const Self = c.as_mut_ptr();

    // cast to u8 pointers so we get offset in bytes
    let c_u8_ptr = c_ptr.cast::<u8>();
    let f_u8_ptr = std::ptr::addr_of!((*c_ptr).parent).cast::<u8>();

    let diff = f_u8_ptr.offset_from(c_u8_ptr);
    if diff.is_negative() {
        panic!("Offset is negative");
    } else {
        diff.unsigned_abs()
    }
};

// Once
fn getDyn(&'static self) -> AstType {
    let nodeParent: &AstNodeStructNode = self;
    let addr: *const AstNodeStructNode = nodeParent;
    let addr_bytes = addr.cast::<u8>();
    // imagine a match for each child
    unsafe {
        let realtype = addr_bytes
            .offset(-(AstTypeBuiltinStructNode::OFFSET_AstNodeStructNode as isize))
            .cast::<AstTypeBuiltinStructNode>();
        let realtype: &'static AstTypeBuiltinStructNode = &*realtype;
        return realtype.into();
    }
}
*/
fn generateGetDyn(
    parents: &Vec<&Class>,
    class: &Class,
    name: &str,
    childNames: &Vec<&str>,
    output: &mut String,
) {
    if parents.is_empty() {
        return;
    }
    output.push_str(&format!("impl {name}StructNode {{\n"));
    let firstParent = parents.first().unwrap().name;

    output.push_str("    #[allow(non_upper_case_globals)]\n");

    output.push_str(&format!(
        "    pub const OFFSET_{name}StructNode_{firstParent}StructNode: usize = unsafe {{\n"
    ));

    output.push_str("        let c = std::mem::MaybeUninit::<Self>::uninit();\n");
    output.push_str("        let c_ptr: *const Self = c.as_ptr();\n");
    output.push_str("        let c_u8_ptr = c_ptr.cast::<u8>();\n");

    output.push_str(&format!(
        "        let f_u8_ptr = std::ptr::addr_of!((*c_ptr){}).cast::<u8>();\n",
        ".parent".repeat(parents.len())
    ));

    output.push_str("        let diff = f_u8_ptr.offset_from(c_u8_ptr);\n");
    output.push_str("        if diff.is_negative() {\n");
    output.push_str("            panic!(\"Offset is negative\");\n");
    output.push_str("        } else {\n");
    output.push_str("            diff.unsigned_abs()\n");
    output.push_str("        }\n");
    output.push_str("    };\n");

    if class.dependedBy.is_empty() {
        output.push_str(&format!(
            "    pub const INTERNAL_FIN_TYPE_TAG: {firstParent}FinalTypes = {firstParent}FinalTypes::{name};\n"
        ));
    }

    output.push_str(&format!("    pub fn getDyn(&'static self) -> {name} {{\n"));
    output.push_str(&format!(
        "        let nodeParent: &'static {firstParent}StructNode = self;\n",
    ));

    output.push_str(&format!(
        "        let addr: *const {firstParent}StructNode = nodeParent;\n",
    ));

    output.push_str("        let addr_bytes = addr.cast::<u8>();\n");

    output.push_str("        match nodeParent.finalType {\n");

    for childName in childNames {
        output.push_str(&format!(
            "            {firstParent}FinalTypes::{childName} => {{\n"
        ));
        output.push_str("                unsafe {\n");
        output.push_str(&format!("                    let realtype = addr_bytes.offset(-({childName}StructNode::OFFSET_{childName}StructNode_{firstParent}StructNode as isize)).cast::<{childName}StructNode>();\n"));
        output.push_str(&format!(
            "                    let realtype: &'static {childName}StructNode = &*realtype;\n"
        ));
        output.push_str("                    return realtype.into();\n");
        output.push_str("                }}\n");
    }
    output.push_str("            _ => unreachable!(\"Could not match internal type tag to one of my types.\")\n");
    output.push_str("        }");
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

    generateClassEnum(parents, class, name, &childNames, output);
    generateStructNodes(parents, class, name, &childNames, output);
    generateFroms(parents, class, name, &childNames, output);
    generateDerefs(parents, class, name, &childNames, output);
    generateGetDyn(parents, class, name, &childNames, output);

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
use std::ops::DerefMut;
use enum_dispatch::enum_dispatch;

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
