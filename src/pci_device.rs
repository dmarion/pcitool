use anyhow::{Context, Result, anyhow};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use crate::pci_capa::{self, ExtCapEntry, StdCapEntry};
use crate::pci_classes;
use crate::pci_config_hdr::PciConfigHdr;
use crate::pci_ids;
use crate::tree::{PciDevice, PciNode};
use crate::{
    CONFIG_READ_BYTES, DDR_OFFSET, ECH_BYTES, ECS_OFFSET, MIN_CONFIG_BYTES, SYSFS_DEVICES,
};
use ratatui::text::Line;

#[derive(Clone)]
pub struct DeviceSummary {
    pub address: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub name_suffix: String,
    pub subvendor_id: u16,
    pub subdevice_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
}

pub fn list_devices() -> Result<Vec<String>> {
    let mut devices = Vec::new();
    for entry in fs::read_dir(SYSFS_DEVICES).context("scanning /sys/bus/pci/devices")? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            devices.push(name.to_owned());
        }
    }
    devices.sort();
    Ok(devices)
}

pub fn summarize_device(address: &str) -> Result<DeviceSummary> {
    let device_path = Path::new(SYSFS_DEVICES).join(address);
    let config_path = device_path.join("config");
    let mut buf = [0u8; MIN_CONFIG_BYTES];
    let mut file =
        File::open(&config_path).with_context(|| format!("opening {}", config_path.display()))?;
    let n = file
        .read(&mut buf)
        .with_context(|| format!("reading {}", config_path.display()))?;
    if n < MIN_CONFIG_BYTES {
        return Err(anyhow!("config space too short ({} bytes)", n));
    }
    let header = PciConfigHdr::parse(&buf)?;
    Ok(DeviceSummary {
        address: address.to_string(),
        vendor_id: header.vendor_id,
        device_id: header.device_id,
        name_suffix: pci_ids::name_suffix(header.vendor_id, header.device_id),
        subvendor_id: header.subsystem_vendor_id,
        subdevice_id: header.subsystem_id,
        class: header.class_code,
        subclass: header.subclass,
        prog_if: header.prog_if,
    })
}

pub fn get_device_tree(summary: &DeviceSummary) -> Result<PciDevice> {
    let config = read_config(&summary.address)?;
    let header = PciConfigHdr::parse(&config)?;
    let mut items = Vec::new();

    let info = [
        ("PCI Address", summary.address.clone(), String::new(), true),
        (
            "Vendor",
            pci_ids::vendor_name(summary.vendor_id)
                .unwrap_or("Unknown")
                .to_string(),
            format!("(0x{:04x})", summary.vendor_id),
            true,
        ),
        (
            "Device",
            pci_ids::device_name(summary.vendor_id, summary.device_id)
                .unwrap_or("Unknown")
                .to_string(),
            format!("(0x{:04x})", summary.device_id),
            true,
        ),
        (
            "Sub-Vendor",
            pci_ids::vendor_name(summary.subvendor_id)
                .unwrap_or("Unknown")
                .to_string(),
            format!("(0x{:04x})", summary.subvendor_id),
            summary.subvendor_id != 0,
        ),
        (
            "Sub-Device",
            pci_ids::device_name(summary.subvendor_id, summary.subdevice_id)
                .unwrap_or("Unknown")
                .to_string(),
            format!("(0x{:04x})", summary.subdevice_id),
            summary.subdevice_id != 0,
        ),
        (
            "Class",
            pci_classes::class_name(summary.class, summary.subclass, summary.prog_if),
            format!(
                "(0x{:02x}{:02x}{:02x})",
                summary.class, summary.subclass, summary.prog_if
            ),
            true,
        ),
        (
            "Driver",
            get_driver_name(&summary.address).unwrap_or_else(|| "none".to_string()),
            String::new(),
            true,
        ),
    ];

    for (label, value, suffix, condition) in info {
        if !condition {
            continue;
        }
        let suffix = if suffix.is_empty() {
            String::new()
        } else {
            format!(" {}", suffix)
        };
        items.push(PciNode::with_value(
            Line::from(label),
            Line::from(format!("{}{}", value, suffix)),
        ));
    }

    let mut std_caps = scan_standard_capabilities(&header, &config);
    std_caps.sort_by_key(|(off, _, _)| *off);
    let mut ext_caps = scan_extended_capabilities(&config);
    ext_caps.sort_by_key(|(off, _, _, _)| *off);

    let mut summary_nodes = Vec::new();
    for (off, id, bytes) in &std_caps {
        if let Some(cap) = pci_capa::STD_CAP_REGISTRY.iter().find(|cap| cap.id == *id) {
            if let Some(f) = cap.summary {
                if let Some(nodes) = f(*off, bytes, &config) {
                    summary_nodes.extend(nodes);
                }
            }
        }
    }
    for (off, ver, id, bytes) in &ext_caps {
        if let Some(cap) = pci_capa::EXT_CAP_REGISTRY
            .iter()
            .find(|cap| cap.id == *id && cap.version == *ver)
        {
            if let Some(f) = cap.summary {
                if let Some(nodes) = f(*off, *ver, bytes) {
                    summary_nodes.extend(nodes);
                }
            }
        }
    }
    items.extend(summary_nodes);

    let warnings = parse_capabilities(&header, &config);
    for warn in warnings {
        items.push(PciNode::new(Line::from(format!("note: {warn}"))));
    }

    if !std_caps.is_empty() {
        let mut caps_node = PciNode::new(Line::from("Capabilities:"));
        caps_node.children = build_standard_caps_nodes(&std_caps);
        items.push(caps_node);
    }

    if !ext_caps.is_empty() {
        let mut ext_caps_node = PciNode::new(Line::from("Extended Capabilities:"));
        ext_caps_node.children = build_extended_caps_nodes(&ext_caps);
        items.push(ext_caps_node);
    }

    let mut root = PciNode::new(Line::from(format!("PCI Device {}", summary.address)));
    root.children = items;

    Ok(PciDevice::new(vec![root]))
}

fn get_driver_name(address: &str) -> Option<String> {
    let driver_path = Path::new(SYSFS_DEVICES).join(address).join("driver");
    fs::read_link(driver_path)
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
}

fn read_config(address: &str) -> Result<Vec<u8>> {
    let device_path = Path::new(SYSFS_DEVICES).join(address);
    let config_path = device_path.join("config");
    let mut config = Vec::with_capacity(CONFIG_READ_BYTES);
    let mut file =
        File::open(&config_path).with_context(|| format!("opening {}", config_path.display()))?;
    file.by_ref()
        .take(CONFIG_READ_BYTES as u64)
        .read_to_end(&mut config)
        .with_context(|| format!("reading {}", config_path.display()))?;
    if config.len() < MIN_CONFIG_BYTES {
        return Err(anyhow!(
            "config space too short ({} bytes captured)",
            config.len()
        ));
    }
    Ok(config)
}

fn parse_capabilities(header: &PciConfigHdr, config: &[u8]) -> Vec<String> {
    let mut warnings = Vec::new();
    if !header.capabilities_list || header.capabilities_pointer == 0 {
        return warnings;
    }
    let cap_pointer = header.capabilities_pointer as usize;
    if !(DDR_OFFSET..ECS_OFFSET).contains(&cap_pointer) {
        warnings.push("capability pointer falls outside captured config space".to_string());
        return warnings;
    }
    let mut ptr = header.capabilities_pointer as usize;
    let mut pcie_cap = None;
    while ptr >= DDR_OFFSET && ptr + 2 <= ECS_OFFSET {
        let cap_id = config[ptr];
        let next = config[ptr + 1] as usize;
        if cap_id == 0x10 {
            pcie_cap = Some(ptr);
            break;
        }
        if next == 0 || !(DDR_OFFSET..ECS_OFFSET).contains(&next) {
            break;
        }
        ptr = next;
    }
    if let Some(offset) = pcie_cap {
        if read_u16_le(config, offset + 0x0a).is_none() {
            warnings.push("PCIe capability truncated".to_string());
        }
    }
    warnings
}

fn scan_standard_capabilities(header: &PciConfigHdr, config: &[u8]) -> Vec<StdCapEntry> {
    if !header.capabilities_list || header.capabilities_pointer == 0 {
        return Vec::new();
    }
    let mut caps = Vec::new();
    let mut visited = HashSet::new();
    let mut ptr = header.capabilities_pointer as usize;
    while ptr >= DDR_OFFSET && ptr + 2 <= ECS_OFFSET {
        if !visited.insert(ptr as u8) {
            break;
        }
        let cap_id = config[ptr];
        let next = config[ptr + 1] as usize;
        let end = if next > ptr && next < ECS_OFFSET {
            next
        } else {
            ECS_OFFSET
        };
        let bytes = config.get(ptr..end).unwrap_or_default().to_vec();
        caps.push((ptr as u8, cap_id, bytes));
        if next == 0 || !(DDR_OFFSET..ECS_OFFSET).contains(&next) {
            break;
        }
        ptr = next;
    }
    caps
}

fn build_standard_caps_nodes(caps: &[StdCapEntry]) -> Vec<PciNode> {
    let mut nodes = Vec::new();
    for (off, id, bytes) in caps {
        let (name, children) =
            if let Some(cap) = pci_capa::STD_CAP_REGISTRY.iter().find(|cap| cap.id == *id) {
                let mut children = Vec::new();
                pci_capa::print_registers(
                    cap.registers,
                    |off, size| pci_capa::read_raw(bytes, off, size),
                    2,
                    2,
                    &mut children,
                );
                (cap.name.to_string(), children)
            } else {
                (
                    format!("Unknown (0x{id:02x})"),
                    vec![PciNode::with_value(
                        Line::from("  Offset"),
                        Line::from(format!("0x{off:02x}")),
                    )],
                )
            };

        let mut cap_node = PciNode::new_collapsed(Line::from(name));
        cap_node.children = children;

        let mut data_node = PciNode::new_collapsed(Line::from("Data"));
        data_node.children = render_cap_data_nodes(*off as u16, bytes);
        cap_node.add_child(data_node);

        nodes.push(cap_node);
    }
    nodes
}

fn read_u16_le(bytes: &[u8], offset: usize) -> Option<u16> {
    bytes
        .get(offset..offset + 2)
        .and_then(|slice| slice.try_into().ok())
        .map(u16::from_le_bytes)
}

fn read_ext_block(config: &[u8], offset: u16) -> Option<(ExtCapEntry, usize)> {
    let start = offset as usize;
    let header_bytes: [u8; 4] = config.get(start..start + ECH_BYTES)?.try_into().ok()?;
    let header_word = u32::from_le_bytes(header_bytes);
    let next = ((header_word >> 20) & 0x0fff) as usize;
    let version = ((header_word >> 16) & 0x0f) as u8;
    let id = (header_word & 0xffff) as u16;
    if id == 0 || version == 0 || version > 4 {
        return None;
    }
    if next != 0 && (next <= start || next < ECS_OFFSET || next > config.len()) {
        return None;
    }
    let end = if next > start && next <= config.len() {
        next
    } else {
        config.len()
    };
    let bytes = config.get(start..end)?.to_vec();
    Some(((offset, version, id, bytes), next))
}

fn scan_extended_capabilities(config: &[u8]) -> Vec<ExtCapEntry> {
    let mut caps = Vec::new();
    if config.len() < ECS_OFFSET + ECH_BYTES {
        return caps;
    }
    let mut visited = HashSet::new();
    let mut offset = ECS_OFFSET as u16;
    while (offset as usize + ECH_BYTES) <= config.len() {
        if !visited.insert(offset) {
            break;
        }
        let (block, next) = match read_ext_block(config, offset) {
            Some(entry) => entry,
            None => break,
        };
        caps.push(block);
        if next == 0 || next < ECS_OFFSET || next > config.len() {
            break;
        }
        offset = next as u16;
    }
    caps
}

fn build_extended_caps_nodes(caps: &[ExtCapEntry]) -> Vec<PciNode> {
    let mut nodes = Vec::new();
    for (offset, version, id, bytes) in caps {
        let (title, children) = if let Some(cap) = pci_capa::EXT_CAP_REGISTRY
            .iter()
            .find(|cap| cap.id == *id && cap.version == *version)
        {
            let mut children = Vec::new();
            pci_capa::print_registers(
                cap.registers,
                |off, size| pci_capa::read_raw(bytes, off, size),
                3,
                2,
                &mut children,
            );
            (format!("{} (v{})", cap.name, cap.version), children)
        } else {
            (
                format!("Unknown (0x{id:04x})"),
                vec![PciNode::with_value(
                    Line::from("  Version"),
                    Line::from(version.to_string()),
                )],
            )
        };

        let mut cap_node = PciNode::new_collapsed(Line::from(title));
        cap_node.children = children;

        let mut data_node = PciNode::new_collapsed(Line::from("Data"));
        data_node.children = render_cap_data_nodes(*offset, bytes);
        cap_node.add_child(data_node);

        nodes.push(cap_node);
    }
    nodes
}

fn render_cap_data_nodes(start: u16, data: &[u8]) -> Vec<PciNode> {
    let mut nodes = Vec::new();
    let base = start as usize;
    for (i, chunk) in data.chunks(16).enumerate() {
        let offset = base + i * 16;
        let hex = chunk
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        nodes.push(PciNode::new(Line::from(format!(
            "    {:04x}: {}",
            offset, hex
        ))));
    }
    nodes
}
