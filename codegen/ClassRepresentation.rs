pub struct Class {
    pub name: &'static str,
    pub dependedBy: Vec<Class>,
    pub traits: Vec<&'static str>,
}
