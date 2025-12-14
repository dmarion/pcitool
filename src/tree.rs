use ratatui::text::{Line, Span};

const MIN_VALUE_COLUMN: usize = 28;

#[derive(Clone, Debug)]
pub struct PciNode {
    pub name: Line<'static>,
    pub value: Option<Line<'static>>,
    pub children: Vec<PciNode>,
    pub expanded: bool,
    pub children_value_indent: usize,
    pub align_with_parent: bool,
}

impl PciNode {
    pub fn new(name: Line<'static>) -> Self {
        Self {
            name,
            value: None,
            children: Vec::new(),
            expanded: true,
            children_value_indent: 0,
            align_with_parent: false,
        }
    }

    pub fn with_value(name: Line<'static>, value: Line<'static>) -> Self {
        Self {
            name,
            value: Some(value),
            children: Vec::new(),
            expanded: true,
            children_value_indent: 0,
            align_with_parent: false,
        }
    }

    pub fn with_value_collapsed(name: Line<'static>, value: Line<'static>) -> Self {
        Self {
            name,
            value: Some(value),
            children: Vec::new(),
            expanded: false,
            children_value_indent: 0,
            align_with_parent: false,
        }
    }

    pub fn new_collapsed(name: Line<'static>) -> Self {
        Self {
            name,
            value: None,
            children: Vec::new(),
            expanded: false,
            children_value_indent: 0,
            align_with_parent: false,
        }
    }

    pub fn add_child(&mut self, node: PciNode) {
        self.children.push(node);
    }

    pub fn name_len(&self) -> usize {
        self.name.to_string().len()
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

    pub fn render(&self, target_column: usize, depth: usize) -> Line<'static> {
        if let Some(ref val) = self.value {
            let target_column = target_column.max(MIN_VALUE_COLUMN);
            let mut spans = self.name.spans.clone();
            let name_end_column = depth * 2 + self.name_len();
            if target_column > name_end_column {
                spans.push(Span::raw(" ".repeat(target_column - name_end_column)));
            }
            spans.push(Span::raw(" : "));
            spans.extend(val.spans.clone());
            Line::from(spans)
        } else {
            self.name.clone()
        }
    }
}

pub struct PciDevice {
    pub nodes: Vec<PciNode>,
    pub children_value_indent: usize,
}

impl PciDevice {
    pub fn new(mut nodes: Vec<PciNode>) -> Self {
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
