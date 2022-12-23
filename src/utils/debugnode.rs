#[derive(Clone, Eq, PartialEq)]
pub struct DebugNode {
    name: String,
    children: Vec<DebugNode>,
}

impl DebugNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: vec![],
        }
    }

    pub fn add_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_children(mut self, children: Vec<Self>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn add_children_from(mut self, children: &[Self]) -> Self {
        self.children.reserve(children.len());
        for c in children {
            self.children.push(c.clone());
        }
        self
    }

    fn to_string_rec(&self, depth: u32, isLast: bool, result: &mut String) {
        if depth > 0 {
            result.push_str(&"│  ".repeat((depth - 1).try_into().unwrap()));
            result.push_str(if isLast { "╰─ " } else { "├─ " });
        };
        result.push_str(&self.name);
        if self.children.is_empty() {
            return;
        }
        result.push('\n');
        for (i, child) in self.children.iter().enumerate() {
            let isLast = i == self.children.len() - 1;
            child.to_string_rec(depth + 1, isLast, result);
        }
    }

    fn to_string_mem(&self, depth: u32) -> usize {
        self.children
            .iter()
            .map(|c| c.to_string_mem(depth + 1))
            .sum::<usize>()
            + self.name.as_bytes().len()
            + (depth as usize) * 5
    }
}

impl ToString for DebugNode {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.reserve(self.to_string_mem(0));
        self.to_string_rec(0, true, &mut result);
        result
    }
}
