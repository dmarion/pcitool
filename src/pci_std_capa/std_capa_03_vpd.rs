use crate::capabilities;
use crate::pci_device::PciCapa;
use crate::tree::{TreeLine, TreeNode};
use std::thread;
use std::time::Duration;

capabilities! {
    {
        id: 0x03,
        name: "Vital Product Data",
        size: 8,
        summary: vpd_summary,
        registers: [
            {
                name: "VPD Address",
                offset: 0x02,
                id: VPD_ADDR,
                size: Word,
                fields: [
                    { name: "VPD Address", lsb: 0, bits: 15 },
                    {
                        name: "F",
                        lsb: 15,
                        bits: 1,
                        enum_values: [
                            (0x0, "Read"),
                            (0x1, "Write/Done"),
                        ]
                    },
                ]
            },
            {
                name: "VPD Data",
                offset: 0x04,
                id: VPD_DATA,
                size: Dword,
                fields: []
            }
        ]
    }
}

fn vpd_summary(capa: &PciCapa) -> Option<Vec<TreeNode>> {
    let mut nodes = Vec::new();
    let mut vpd_data = Vec::new();
    let mut addr = 0u16;

    // Read up to 32KB, but stop when we find End Tag
    while addr < 0x8000 {
        if let Some(dword) = read_vpd_dword(capa, addr) {
            let bytes = dword.to_le_bytes();
            vpd_data.extend_from_slice(&bytes);

            if vpd_data.len() > 65536 {
                break;
            }
        } else {
            break;
        }
        addr += 4;

        // Try to parse what we have to see if we reached the end
        if let Some(last_pos) = find_end_tag(&vpd_data) {
            if last_pos < vpd_data.len() {
                vpd_data.truncate(last_pos + 1);
                break;
            }
        }
    }

    if vpd_data.is_empty() {
        return None;
    }

    let parsed_nodes = parse_vpd(&vpd_data);
    nodes.extend(parsed_nodes);

    if nodes.is_empty() { None } else { Some(nodes) }
}

fn read_vpd_dword(cap: &PciCapa, vpd_addr: u16) -> Option<u32> {
    if cap
        .write_u16(u64::from(VPD_ADDR), vpd_addr & 0x7fff)
        .is_err()
    {
        return None;
    }

    for _ in 0..50 {
        thread::sleep(Duration::from_millis(1));
        if let Ok(val) = cap.read_u16(u64::from(VPD_ADDR)) {
            if val & 0x8000 != 0 {
                return cap.read_u32(u64::from(VPD_DATA)).ok();
            }
        }
    }
    None
}

fn find_end_tag(data: &[u8]) -> Option<usize> {
    let mut pos = 0;
    while pos < data.len() {
        let tag = data[pos];
        if tag == 0x78 {
            return Some(pos);
        }

        if tag & 0x80 == 0 {
            // Small Resource
            let len = (tag & 0x07) as usize;
            pos += 1 + len;
        } else {
            // Large Resource
            if pos + 2 >= data.len() {
                return None;
            }
            let len = (data[pos + 1] as usize) | ((data[pos + 2] as usize) << 8);
            pos += 3 + len;
        }
    }
    None
}

fn parse_vpd(data: &[u8]) -> Vec<TreeNode> {
    let mut nodes = Vec::new();
    let mut pos = 0;
    let mut pending_product_name: Option<String> = None;

    while pos < data.len() {
        let tag = data[pos];

        if tag == 0x78 {
            // End Tag
            break;
        }

        if tag & 0x80 == 0 {
            // Small Resource
            let name = (tag >> 3) & 0x0f;
            let len = (tag & 0x07) as usize;
            if pos + 1 + len > data.len() {
                break;
            }
            let content = &data[pos + 1..pos + 1 + len];

            if name == 0x02 {
                // Identifier String
                // Trim trailing nulls and whitespace
                let clean_content = content
                    .iter()
                    .take_while(|&&b| b != 0)
                    .cloned()
                    .collect::<Vec<u8>>();
                let id_str = String::from_utf8_lossy(&clean_content).trim().to_string();
                pending_product_name = Some(id_str);
            } else if name != 0xF {
                nodes.push(TreeNode::with_value(
                    TreeLine::from(format!("Unknown Small Resource (Tag 0x{:x})", name)),
                    TreeLine::from(format!("{:?}", content)), // Simplify
                ));
            }

            pos += 1 + len;
        } else {
            // Large Resource
            if pos + 3 > data.len() {
                break;
            }
            let name = tag & 0x7f;
            let len = (data[pos + 1] as usize) | ((data[pos + 2] as usize) << 8);
            if pos + 3 + len > data.len() {
                break;
            }
            let content = &data[pos + 3..pos + 3 + len];

            match name {
                0x02 => {
                    // Identifier String (Large Resource version - technically spec says Small 0x02, but some HW might use Large?)
                    // The spec says Tag 0x02 is "Identifier String" which is a Large Resource (Tag 0x02 | 0x80 = 0x82).
                    // Wait, Small Resource tags are 3 bits. 0x2 is "Reserved".
                    // Large Resource 0x02 is "Identifier String".
                    // My previous code handled 0x02 as Large.
                    // Let's keep logic for Large 0x02.
                    let clean_content = content
                        .iter()
                        .take_while(|&&b| b != 0)
                        .cloned()
                        .collect::<Vec<u8>>();
                    let id_str = String::from_utf8_lossy(&clean_content).trim().to_string();
                    pending_product_name = Some(id_str);
                }
                0x10 => {
                    // VPD-R
                    // Use "Product Name" as label, with the value if we have it.
                    let label = "Product Name";
                    let value = pending_product_name.take().unwrap_or_default();
                    let mut read_node = TreeNode::with_value_collapsed(
                        TreeLine::from(label),
                        TreeLine::from(value),
                    );
                    read_node.children = parse_keywords(content);
                    nodes.push(read_node);
                }
                0x11 => {
                    // VPD-W
                    let mut write_node =
                        TreeNode::new_collapsed(TreeLine::from("Read-Write Fields"));
                    write_node.children = parse_keywords(content);
                    nodes.push(write_node);
                }
                _ => {
                    nodes.push(TreeNode::with_value(
                        TreeLine::from(format!("Unknown Large Resource (Tag 0x{:x})", name)),
                        TreeLine::from(format!("Length {}", len)),
                    ));
                }
            }

            pos += 3 + len;
        }
    }

    // If we have a product name but no VPD-R block to attach it to, show it as a standalone node.
    if let Some(name) = pending_product_name {
        nodes.push(TreeNode::with_value(
            TreeLine::from("Product Name"),
            TreeLine::from(name),
        ));
    }

    nodes
}

fn parse_keywords(data: &[u8]) -> Vec<TreeNode> {
    let mut nodes = Vec::new();
    let mut pos = 0;
    while pos + 3 <= data.len() {
        let key = &data[pos..pos + 2];
        let len = data[pos + 2] as usize;
        if pos + 3 + len > data.len() {
            break;
        }
        let val = &data[pos + 3..pos + 3 + len];

        let key_str = String::from_utf8_lossy(key).to_string();

        let val_str = if val.iter().all(|b| *b >= 32 && *b <= 126) {
            String::from_utf8_lossy(val)
                .trim_matches(char::from(0))
                .trim()
                .to_string()
        } else {
            val.iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")
        };

        let desc = get_keyword_desc(&key_str);

        nodes.push(TreeNode::with_value(
            TreeLine::from(format!("{} ({})", desc, key_str)),
            TreeLine::from(val_str),
        ));

        pos += 3 + len;
    }
    nodes
}

fn get_keyword_desc(key: &str) -> &str {
    match key {
        "PN" => "Part Number",
        "EC" => "Engineering Change Level",
        "MN" => "Manufacture ID",
        "SN" => "Serial Number",
        "CP" => "Extended Capability",
        "RV" => "Reserved",
        "YA" => "Asset Tag",
        "V0" | "V1" | "V2" | "V3" | "V4" | "V5" | "V6" | "V7" | "V8" | "V9" | "VA" | "VB"
        | "VC" | "VD" | "VE" | "VF" => "Vendor Specific",
        "Y0" | "Y1" | "Y2" | "Y3" | "Y4" | "Y5" | "Y6" | "Y7" | "Y8" | "Y9" | "YB" | "YC"
        | "YD" | "YE" | "YF" => "System Specific",
        "RW" => "Read-Write Area",
        _ => "Unknown",
    }
}
