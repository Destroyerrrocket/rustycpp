#[derive(Clone)]
pub struct Trait {
    pub name: &'static str,
    pub func: &'static dyn Fn(&str) -> String,
}

impl Trait {
    pub const fn new(name: &'static str, func: &'static dyn Fn(&str) -> String) -> Self {
        Self { name, func }
    }
}

pub type FuncCustomCodegen = &'static dyn Fn(&Vec<&Class>, &Class) -> String;

pub struct Class {
    pub name: &'static str,
    pub dependedBy: Vec<Class>,
    pub traits: Vec<&'static str>,
    pub customCodegen: Vec<FuncCustomCodegen>,
}
