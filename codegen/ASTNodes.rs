use crate::codegen::ClassRepresentation::Class;
/*
#[enum_dispatch()]
enum Stmt {
    Stmt(&'static StmtStructNode),
    Expr(&'static ExprStructNode),
    Expr2(&'static Expr2StructNode),
}

struct StmtStruct;
struct StmtStructNode {
    base: StmtStruct,
}

impl StmtStructNode {
    fn function(&self, namethingy: &mut String) -> String {
        namethingy.push_str("hello");
        String::from("hello")
    }
    fn function2(&self, namethingy: &mut String) -> String {
        namethingy.push_str("hello2");
        String::from("hello2")
    }
    fn function3(&self, namethingy: &mut String) -> String {
        namethingy.push_str("hello3");
        String::from("hello3")
    }
}

impl Deref for Stmt {
    type Target = StmtStructNode;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Stmt(node) => node,
            Self::Expr(node) => node,
            Self::Expr2(node) => node,
        }
    }
}

#[enum_dispatch()]

enum Expr {
    Expr(&'static ExprStructNode),
    Expr2(&'static Expr2StructNode),
}
struct ExprStruct;
struct ExprStructNode {
    parent: StmtStructNode,
    base: ExprStruct,
}

impl ExprStructNode {
    fn function(&self, namethingy: &mut String) -> String {
        namethingy.push_str("helloSpecial");
        String::from("helloSpecial")
    }
    fn function2(&self, namethingy: &mut String) -> String {
        namethingy.push_str("helloSpecial2");
        String::from("helloSpecial2")
    }
}

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

    #[allow(unreachable_patterns)]
    fn try_from(value: Stmt) -> Result<Self, Self::Error> {
        match value {
            Stmt::Expr(node) => Ok(Expr::Expr(node)),
            Stmt::Expr2(node) => Ok(Expr::Expr2(node)),
            _ => Err(()),
        }
    }
}

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

struct Expr2Struct;
struct Expr2StructNode {
    parent: ExprStructNode,
    base: ExprStruct,
}

#[enum_dispatch()]

enum Expr2 {
    Expr2(&'static Expr2StructNode),
}

impl Expr2StructNode {
    fn function(&self, namethingy: &mut String) -> String {
        namethingy.push_str("helloSuccess");
        String::from("helloSuccess")
    }
}

impl From<Expr2> for Expr {
    fn from(value: Expr2) -> Self {
        match value {
            Expr2::Expr2(node) => Expr::Expr2(node),
        }
    }
}

impl TryFrom<Expr> for Expr2 {
    type Error = ();

    fn try_from(value: Expr) -> Result<Self, Self::Error> {
        match value {
            Expr::Expr2(node) => Ok(Expr2::Expr2(node)),
            _ => Err(()),
        }
    }
}

impl Deref for Expr2StructNode {
    type Target = ExprStructNode;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl Deref for Expr2 {
    type Target = Expr2StructNode;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Expr2(node) => node,
        }
    }
}

#[enum_dispatch]
trait Trait1 {
    fn f1(&self);
}

#[enum_dispatch]
trait Trait2 {
    fn f2(&self);
}

struct FooBar;
impl Trait1 for &FooBar {
    fn f1(&self) {
        println!("f1");
    }
}

impl Trait2 for &FooBar {
    fn f2(&self) {
        println!("f2");
    }
}

#[enum_dispatch(Trait1, Trait2)]
enum Foo {
    FooBar(&'static FooBar),
}
*/

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

fn getAllTraitsToImplement(classes: &Vec<&Class>, class: &Class) -> Vec<&'static str> {
    let mut traits = class.traits.clone();
    for class in classes {
        for t in &class.traits {
            if !traits.contains(t) {
                traits.push(t);
            }
        }
    }
    traits
}

fn generateConcrete(parents: &mut Vec<&Class>, class: &Class, output: &mut String) {
    // A concrete type!
    /*
    #[enum_dispatch(Trait1, Trait2)]
    enum Expr {
        Expr(&'static ExprStructNode),
    }
    */
    let name: &&str = &class.name;

    output.push_str("#[enum_dispatch(");
    for implementTrait in getAllTraitsToImplement(parents, class) {
        output.push_str(&format!("{implementTrait}, "));
    }
    output.push_str(")]\n");
    output.push_str(&format!("enum {name} {{\n"));
    output.push_str(&format!("    {name}(&'static {name}StructNode),\n"));
    output.push_str("}\n");
    /*
    struct ExprStruct; // user provided
    struct ExprStructNode {
        parent: StmtStructNode,
        base: ExprStruct,
    }
    */
    output.push_str("#[allow(dead_code)]\n");
    output.push_str(&format!("struct {name}StructNode {{\n"));
    if !parents.is_empty() {
        output.push_str(&format!(
            "    parent: {}StructNode,\n",
            &parents.last().unwrap().name
        ));
    }
    output.push_str(&format!("    base: {name}Struct,\n"));
    output.push_str("}\n");
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
    if !parents.is_empty() {
        for parent in parents.iter() {
            let parentName = parent.name;
            output.push_str(&format!("impl From<{name}> for {parentName} {{\n"));
            output.push_str(&format!("    fn from(value: {name}) -> Self {{\n"));
            output.push_str("        match value {\n");
            output.push_str(&format!(
                "            {name}::{name}(node) => Self::{name}(node),\n"
            ));
            output.push_str("        }\n");
            output.push_str("    }\n");
            output.push_str("}\n");
            output.push_str(&format!("impl TryFrom<{parentName}> for {name} {{\n"));
            output.push_str("    type Error = ();\n");
            output.push_str("    #[allow(unreachable_patterns)]\n");
            output.push_str(&format!(
                "    fn try_from(value: {parentName}) -> Result<Self, Self::Error> {{\n"
            ));
            output.push_str("        match value {\n");
            output.push_str(&format!(
                "            {parentName}::{name}(node) => Ok(Self::{name}(node)),\n"
            ));
            output.push_str("            _ => Err(()),\n");
            output.push_str("        }\n");
            output.push_str("    }\n");
            output.push_str("}\n");
        }
    }
    /*
    impl Deref for ExprStructNode {
        type Target = StmtStructNode;

        fn deref(&self) -> &Self::Target {
            &self.parent
        }
    }
    */
    if !parents.is_empty() {
        let parentName = &parents.last().unwrap().name;
        output.push_str(&format!("impl Deref for {name}StructNode {{\n"));
        output.push_str(&format!("    type Target = {parentName}StructNode;\n"));
        output.push_str("    fn deref(&self) -> &Self::Target {\n");
        output.push_str("        &self.parent\n");
        output.push_str("    }\n");
        output.push_str("}\n");
    }
    /*
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
    output.push_str(&format!("impl Deref for {name} {{\n"));
    output.push_str(&format!("    type Target = {name}StructNode;\n"));
    output.push_str("    fn deref(&self) -> &Self::Target {\n");
    output.push_str("        match self {\n");
    output.push_str(&format!("            {name}::{name}(node) => node,\n"));
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n");
}

fn generateAbstract(parents: &mut Vec<&Class>, class: &Class, output: &mut String) {
    /*
    #[enum_dispatch(Trait1, Trait2)]
    enum Expr {
        Expr(&'static ExprStructNode),
        Expr2(&'static Expr2StructNode),
    }
    */
    let childNames = getAllConcreteClassNames(class);
    let name = &class.name;

    output.push_str("#[enum_dispatch(");
    for implementTrait in getAllTraitsToImplement(parents, class) {
        output.push_str(&format!("{implementTrait}, "));
    }
    output.push_str(")]\n");
    output.push_str(&format!("enum {name} {{\n"));
    for childName in &childNames {
        output.push_str(&format!(
            "    {childName}(&'static {childName}StructNode),\n"
        ));
    }
    output.push_str("}\n");
    /*
    struct ExprStruct; // user provided
    struct ExprStructNode {
        parent: StmtStructNode,
        base: ExprStruct,
    }
    */
    output.push_str("#[allow(dead_code)]\n");
    output.push_str(&format!("struct {name}StructNode {{\n"));
    if !parents.is_empty() {
        output.push_str(&format!(
            "    parent: {}StructNode,\n",
            &parents.last().unwrap().name
        ));
    }
    output.push_str(&format!("    base: {name}Struct,\n"));
    output.push_str("}\n");
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
    if !parents.is_empty() {
        for parent in parents.iter() {
            let parentName = parent.name;
            output.push_str(&format!("impl From<{name}> for {parentName} {{\n"));
            output.push_str(&format!("    fn from(value: {name}) -> Self {{\n"));
            output.push_str("        match value {\n");
            for childName in &childNames {
                output.push_str(&format!(
                    "            {name}::{childName}(node) => Self::{childName}(node),\n"
                ));
            }
            output.push_str("        }\n");
            output.push_str("    }\n");
            output.push_str("}\n");
            output.push_str(&format!("impl TryFrom<{parentName}> for {name} {{\n"));
            output.push_str("    type Error = ();\n");
            output.push_str("    #[allow(unreachable_patterns)]\n");
            output.push_str(&format!(
                "    fn try_from(value: {parentName}) -> Result<Self, Self::Error> {{\n"
            ));
            output.push_str("        match value {\n");
            for childName in &childNames {
                output.push_str(&format!(
                    "            {parentName}::{childName}(node) => Ok(Self::{childName}(node)),\n"
                ));
            }
            output.push_str("            _ => Err(()),\n");
            output.push_str("        }\n");
            output.push_str("    }\n");
            output.push_str("}\n");
        }
    }

    /*
    impl Deref for ExprStructNode {
        type Target = StmtStructNode;

        fn deref(&self) -> &Self::Target {
            &self.parent
        }
    }
    */
    if !parents.is_empty() {
        let parentName = &parents.last().unwrap().name;
        output.push_str(&format!("impl Deref for {name}StructNode {{\n"));
        output.push_str(&format!("    type Target = {parentName}StructNode;\n"));
        output.push_str("    fn deref(&self) -> &Self::Target {\n");
        output.push_str("        &self.parent\n");
        output.push_str("    }\n");
        output.push_str("}\n");
    }
    /*
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

pub fn generate<'classes>(
    parents: &mut Vec<&'classes Class>,
    class: &'classes Class,
    output: &mut String,
) {
    if class.dependedBy.is_empty() {
        // A concrete type!
        generateConcrete(parents, class, output);
    } else {
        // An abstract type!
        generateAbstract(parents, class, output);
        parents.push(class);
        for child in &class.dependedBy {
            generate(parents, child, output);
        }
        parents.pop();
    }
}

#[must_use]
pub fn generateFile(classes: &Class) -> String {
    let mut output = String::new();
    output.push_str("use std::ops::Deref;\n");
    output.push_str("use enum_dispatch::enum_dispatch;\n");
    generate(&mut vec![], classes, &mut output);
    output
}
