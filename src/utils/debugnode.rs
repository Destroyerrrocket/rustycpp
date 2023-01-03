use colored::Colorize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DebugNode {
    name: String,
    children: Vec<DebugNode>,
}

impl DebugNode {
    pub fn new(name: String) -> Self {
        let mut result = String::new();
        Self::colorizedTag(&mut result, &name);
        Self {
            name: result,
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

    fn colorizedTag(result: &mut String, tag: &str) {
        colored::control::set_override(true);
        #[allow(clippy::unnecessary_to_owned)]
        match tag.char_indices().find(|(_, c)| *c == ':') {
            Some((pos, _)) => {
                let (a, b) = tag.split_at(pos);
                result.push_str(&a.to_owned().bright_cyan().bold().to_string());
                result.push_str(&":".to_owned().bright_white().bold().to_string());
                result.push_str(&b[1..].to_owned().bright_yellow().bold().to_string());
            }
            None => {
                result.push_str(&tag.to_owned().bright_blue().bold().to_string());
            }
        }
        colored::control::unset_override();
    }

    fn to_string_rec(&self, depth: &mut Vec<bool>, isLast: bool, result: &mut String) {
        if !depth.is_empty() {
            for last in &depth[1..depth.len()] {
                result.push_str(if *last { "   " } else { "│  " });
            }
            result.push_str(if isLast { "╰─ " } else { "├─ " });
        };
        result.push_str(&self.name);
        result.push('\n');

        if self.children.is_empty() {
            return;
        }

        for (i, child) in self.children.iter().enumerate() {
            depth.push(isLast);
            let isLast = i == self.children.len() - 1;
            child.to_string_rec(depth, isLast, result);
            depth.pop();
        }
    }

    fn to_string_mem(&self, depth: u32) -> usize {
        self.children
            .iter()
            .map(|c| c.to_string_mem(depth + 1))
            .sum::<usize>()
            + self.name.as_bytes().len()
            + (depth as usize) * 5
            + 1 // nl
    }
}

impl ToString for DebugNode {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.reserve(self.to_string_mem(0));
        self.to_string_rec(&mut vec![], true, &mut result);
        result.pop();
        result
    }
}

/**
 * Generate a `DebugNode` tree for testing.
 * Usage: debugTree!(name, (child1, ((child11), (child12))), (child2), (child3))
 */
#[macro_export]
macro_rules! debugTree {
    () => {
        $crate::utils::debugnode::DebugNode::new("".to_string())
    };
    (($name:literal, $($child:tt),*)) => {
        debugTree!($name, $($child),*)
    };
    ($name:literal, $($child:tt),*) => {
        $crate::utils::debugnode::DebugNode::new($name.to_string()).add_children(vec![$(debugTree!($child)),*])
    };
    (($name:literal)) => {
        debugTree!($name)
    };
    ($name:literal) => {
        $crate::utils::debugnode::DebugNode::new($name.to_string())
    };
}
