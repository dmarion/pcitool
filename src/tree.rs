const MIN_VALUE_COLUMN: usize = 28;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TreeColor {
    Default,
    Red,
    Green,
    Yellow,
}

#[derive(Clone, Debug)]
pub struct TreeSpan {
    pub text: String,
    pub color: TreeColor,
}

impl TreeSpan {
    pub fn raw(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: TreeColor::Default,
        }
    }

    pub fn styled(text: impl Into<String>, color: TreeColor) -> Self {
        Self {
            text: text.into(),
            color,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TreeLine {
    pub spans: Vec<TreeSpan>,
}

impl TreeLine {}

impl From<&str> for TreeLine {
    fn from(s: &str) -> Self {
        Self {
            spans: vec![TreeSpan::raw(s)],
        }
    }
}

impl From<String> for TreeLine {
    fn from(s: String) -> Self {
        Self {
            spans: vec![TreeSpan::raw(s)],
        }
    }
}

impl From<Vec<TreeSpan>> for TreeLine {
    fn from(spans: Vec<TreeSpan>) -> Self {
        Self { spans }
    }
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub name: TreeLine,
    pub value: Option<TreeLine>,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
    pub children_value_indent: usize,
    pub align_with_parent: bool,
}

impl TreeNode {
    pub fn new(name: impl Into<TreeLine>) -> Self {
        Self {
            name: name.into(),
            value: None,
            children: Vec::new(),
            expanded: true,
            children_value_indent: 0,
            align_with_parent: false,
        }
    }

    pub fn with_value(name: impl Into<TreeLine>, value: impl Into<TreeLine>) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
            children: Vec::new(),
            expanded: true,
            children_value_indent: 0,
            align_with_parent: false,
        }
    }

    pub fn with_value_collapsed(name: impl Into<TreeLine>, value: impl Into<TreeLine>) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
            children: Vec::new(),
            expanded: false,
            children_value_indent: 0,
            align_with_parent: false,
        }
    }

    pub fn new_collapsed(name: impl Into<TreeLine>) -> Self {
        Self {
            name: name.into(),
            value: None,
            children: Vec::new(),
            expanded: false,
            children_value_indent: 0,
            align_with_parent: false,
        }
    }

    pub fn add_child(&mut self, node: TreeNode) {
        self.children.push(node);
    }

    pub fn name_len(&self) -> usize {
        self.name.spans.iter().map(|s| s.text.len()).sum()
    }

    pub fn calculate_indents(&mut self, depth: usize) -> usize {
        let mut children_max_column = 0;
        let mut my_needed_column = depth * 2 + self.name_len();

        for child in &mut self.children {
            let child_needed_column = child.calculate_indents(depth + 1);
            if child.align_with_parent {
                my_needed_column = my_needed_column.max(child_needed_column);
            } else {
                children_max_column = children_max_column.max(child_needed_column);
            }
        }
        self.children_value_indent = children_max_column;
        my_needed_column
    }

    // Returns a TreeLine representing the rendered node (name + padding + value)
    pub fn render(&self, target_column: usize, depth: usize) -> TreeLine {
        if let Some(ref val) = self.value {
            let target_column = target_column.max(MIN_VALUE_COLUMN);
            let mut spans = self.name.spans.clone();
            let name_end_column = depth * 2 + self.name_len();
            if target_column > name_end_column {
                spans.push(TreeSpan::raw(" ".repeat(target_column - name_end_column)));
            }
            spans.push(TreeSpan::raw(" : "));
            spans.extend(val.spans.clone());
            TreeLine { spans }
        } else {
            self.name.clone()
        }
    }
}

pub struct PciDevice {
    pub nodes: Vec<TreeNode>,
    pub children_value_indent: usize,
}

impl PciDevice {
    pub fn new(mut nodes: Vec<TreeNode>) -> Self {
        let mut max = 0;
        for node in &mut nodes {
            let needed = node.calculate_indents(0);
            max = max.max(needed);
        }
        Self {
            nodes,
            children_value_indent: max,
        }
    }
}
