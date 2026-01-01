use crate::tree::{PciDevice, TreeNode};

pub fn render(device: &PciDevice) {
    for node in &device.nodes {
        print_node(
            node,
            0,
            device.children_value_indent,
            device.children_value_indent,
        );
    }
}

fn print_node(node: &TreeNode, depth: usize, parent_column: usize, current_column: usize) {
    let prefix = " ".repeat(depth * 2);
    let expand_prefix = if !node.children.is_empty() {
        if node.expanded { "[-] " } else { "[+] " }
    } else {
        "    "
    };
    let target_column = if node.align_with_parent {
        parent_column
    } else {
        current_column
    };
    let line = node.render(target_column, depth);
    let text = line
        .spans
        .iter()
        .map(|s| s.text.as_str())
        .collect::<Vec<&str>>()
        .join("");
    println!("{}{}{}", prefix, expand_prefix, text);
    if node.expanded {
        for child in &node.children {
            print_node(child, depth + 1, target_column, node.children_value_indent);
        }
    }
}
