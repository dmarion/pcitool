use anyhow::{Context, Result, anyhow};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::FileExt;
use std::path::Path;

use crate::pci_capa::{self, ExtCapEntry, StdCapEntry};
use crate::pci_classes;
use crate::pci_config_hdr::PciConfigHdr;
use crate::pci_ids;
use crate::tree::{PciDevice, TreeLine, TreeNode};
use crate::{
    CONFIG_READ_BYTES, DDR_OFFSET, ECH_BYTES, ECS_OFFSET, MIN_CONFIG_BYTES, SYSFS_DEVICES,
};

#[derive(Clone)]
pub struct DeviceSummary {
    pub address: String,
    pub tree_chain: Vec<String>,
    pub current_link_speed: Option<u8>,
    pub current_link_width: Option<u8>,
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
    let mut devices: Vec<(Vec<String>, String)> = Vec::new();
    for entry in fs::read_dir(SYSFS_DEVICES).context("scanning /sys/bus/pci/devices")? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            let chain = device_tree_chain(name).unwrap_or_else(|| vec![name.to_string()]);
            devices.push((chain, name.to_owned()));
        }
    }
    devices.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(devices.into_iter().map(|(_, name)| name).collect())
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
    let tree_chain = device_tree_chain(address).unwrap_or_else(|| vec![address.to_string()]);
    let (current_link_speed, current_link_width) = current_link_status(&header, &buf);
    Ok(DeviceSummary {
        address: address.to_string(),
        tree_chain,
        current_link_speed,
        current_link_width,
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

pub struct PciConfig {
    file: File,
    path: String,
}

impl PciConfig {
    pub fn open(address: &str, write: bool) -> Result<Self> {
        let device_path = Path::new(SYSFS_DEVICES).join(address);
        let config_path = device_path.join("config");
        let file = fs::OpenOptions::new()
            .read(true)
            .write(write)
            .open(&config_path)
            .with_context(|| format!("opening {}", config_path.display()))?;
        Ok(Self {
            file,
            path: config_path.display().to_string(),
        })
    }

    pub fn read_u8(&self, offset: u64) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.file
            .read_at(&mut buf, offset)
            .with_context(|| format!("reading 1 byte at 0x{:x} from {}", offset, self.path))?;
        Ok(buf[0])
    }

    pub fn read_u16(&self, offset: u64) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.file
            .read_at(&mut buf, offset)
            .with_context(|| format!("reading 2 bytes at 0x{:x} from {}", offset, self.path))?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_u32(&self, offset: u64) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.file
            .read_at(&mut buf, offset)
            .with_context(|| format!("reading 4 bytes at 0x{:x} from {}", offset, self.path))?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_u64(&self, offset: u64) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.file
            .read_at(&mut buf, offset)
            .with_context(|| format!("reading 8 bytes at 0x{:x} from {}", offset, self.path))?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn write_u16(&self, offset: u64, value: u16) -> Result<()> {
        self.file
            .write_at(&value.to_le_bytes(), offset)
            .with_context(|| {
                format!("writing 0x{:04x} to 0x{:x} in {}", value, offset, self.path)
            })?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn write_u32(&self, offset: u64, value: u32) -> Result<()> {
        self.file
            .write_at(&value.to_le_bytes(), offset)
            .with_context(|| {
                format!("writing 0x{:08x} to 0x{:x} in {}", value, offset, self.path)
            })?;
        Ok(())
    }
}

pub struct PciCapa<'a> {
    config: &'a PciConfig,
    base_offset: u64,
}

impl<'a> PciCapa<'a> {
    pub fn new(config: &'a PciConfig, base_offset: u64) -> Self {
        Self {
            config,
            base_offset,
        }
    }

    pub fn read_u8(&self, offset: u64) -> Result<u8> {
        self.config.read_u8(self.base_offset + offset)
    }

    pub fn read_u16(&self, offset: u64) -> Result<u16> {
        self.config.read_u16(self.base_offset + offset)
    }

    pub fn read_u32(&self, offset: u64) -> Result<u32> {
        self.config.read_u32(self.base_offset + offset)
    }

    pub fn read_u64(&self, offset: u64) -> Result<u64> {
        self.config.read_u64(self.base_offset + offset)
    }

    pub fn write_u16(&self, offset: u64, value: u16) -> Result<()> {
        self.config.write_u16(self.base_offset + offset, value)
    }

    #[allow(dead_code)]
    pub fn write_u32(&self, offset: u64, value: u32) -> Result<()> {
        self.config.write_u32(self.base_offset + offset, value)
    }
}

#[allow(dead_code)]
pub fn read_pci_u16(address: &str, offset: u64) -> Result<u16> {
    PciConfig::open(address, false)?.read_u16(offset)
}

#[allow(dead_code)]
pub fn read_pci_u32(address: &str, offset: u64) -> Result<u32> {
    PciConfig::open(address, false)?.read_u32(offset)
}

#[allow(dead_code)]
pub fn write_pci_u16(address: &str, offset: u64, value: u16) -> Result<()> {
    PciConfig::open(address, true)?.write_u16(offset, value)
}

#[allow(dead_code)]
pub fn write_pci_u32(address: &str, offset: u64, value: u32) -> Result<()> {
    PciConfig::open(address, true)?.write_u32(offset, value)
}

fn device_tree_chain(address: &str) -> Option<Vec<String>> {
    let device_path = Path::new(SYSFS_DEVICES).join(address);
    let link = fs::read_link(device_path).ok()?;
    let mut chain = Vec::new();
    for comp in link.components() {
        if let std::path::Component::Normal(name) = comp {
            let name_str = name.to_string_lossy();
            if is_pci_address(&name_str) {
                chain.push(name_str.to_string());
            }
        }
    }
    if chain.is_empty() {
        None
    } else {
        if chain.last().map(String::as_str) != Some(address) {
            chain.push(address.to_string());
        }
        Some(chain)
    }
}

fn is_pci_address(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() != 12 {
        return false;
    }
    if bytes[4] != b':' || bytes[7] != b':' || bytes[10] != b'.' {
        return false;
    }
    for (idx, byte) in bytes.iter().copied().enumerate() {
        if idx == 4 || idx == 7 || idx == 10 {
            continue;
        }
        if !byte.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

fn current_link_status(header: &PciConfigHdr, config: &[u8]) -> (Option<u8>, Option<u8>) {
    let caps = scan_standard_capabilities(header, config);
    for (off, id, _len) in caps {
        if id != 0x10 {
            continue;
        }
        if let Some(raw) = read_u16_le(config, off as usize + 0x12) {
            let status = raw as u16;
            let speed = (status & 0x0f) as u8;
            let width = ((status >> 4) & 0x3f) as u8;
            if speed == 0 || width == 0 {
                return (None, None);
            }
            return (Some(speed), Some(width));
        }
    }
    (None, None)
}

pub fn get_device_tree(summary: &DeviceSummary) -> Result<PciDevice> {
    let config_handle = PciConfig::open(&summary.address, true)
        .ok()
        .or_else(|| PciConfig::open(&summary.address, false).ok());
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
        items.push(TreeNode::with_value(
            TreeLine::from(label),
            TreeLine::from(format!("{}{}", value, suffix)),
        ));
    }

    let mut std_caps = scan_standard_capabilities(&header, &config);
    std_caps.sort_by_key(|(off, _, _)| *off);
    let mut ext_caps = scan_extended_capabilities(&config);
    ext_caps.sort_by_key(|(off, _, _, _)| *off);

    let mut summary_nodes = Vec::new();
    if let Some(cfg) = config_handle.as_ref() {
        for (off, id, _) in &std_caps {
            if let Some(cap) = pci_capa::STD_CAP_REGISTRY.iter().find(|cap| cap.id == *id) {
                if let Some(f) = cap.summary {
                    let pci_capa = PciCapa::new(cfg, *off as u64);
                    if let Some(nodes) = f(&pci_capa) {
                        summary_nodes.extend(nodes);
                    }
                }
            }
        }
        for (off, ver, id, _) in &ext_caps {
            if let Some(cap) = pci_capa::EXT_CAP_REGISTRY
                .iter()
                .find(|cap| cap.id == *id && cap.version == *ver)
            {
                if let Some(f) = cap.summary {
                    let pci_capa = PciCapa::new(cfg, *off as u64);
                    if let Some(nodes) = f(&pci_capa) {
                        summary_nodes.extend(nodes);
                    }
                }
            }
        }
    }
    items.extend(summary_nodes);

    let warnings = parse_capabilities(&header, &config);
    for warn in warnings {
        items.push(TreeNode::new(TreeLine::from(format!("note: {warn}"))));
    }

    if !std_caps.is_empty() {
        let mut caps_node = TreeNode::new(TreeLine::from("Capabilities:"));
        caps_node.children = build_standard_caps_nodes(config_handle.as_ref(), &std_caps);
        items.push(caps_node);
    }

    if !ext_caps.is_empty() {
        let mut ext_caps_node = TreeNode::new(TreeLine::from("Extended Capabilities:"));
        ext_caps_node.children = build_extended_caps_nodes(config_handle.as_ref(), &ext_caps);
        items.push(ext_caps_node);
    }

    let mut root = TreeNode::new(TreeLine::from(format!("PCI Device {}", summary.address)));
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
        caps.push((ptr as u8, cap_id, (end - ptr) as u8));
        if next == 0 || !(DDR_OFFSET..ECS_OFFSET).contains(&next) {
            break;
        }
        ptr = next;
    }
    caps
}

fn build_standard_caps_nodes(config: Option<&PciConfig>, caps: &[StdCapEntry]) -> Vec<TreeNode> {
    let mut nodes = Vec::new();
    for (off, id, len) in caps {
        let pci_capa = config.map(|c| PciCapa::new(c, *off as u64));
        let (name, children) =
            if let Some(cap) = pci_capa::STD_CAP_REGISTRY.iter().find(|cap| cap.id == *id) {
                let mut children = Vec::new();
                pci_capa::print_registers(
                    cap.registers,
                    |reg_off, size| {
                        if let Some(c) = &pci_capa {
                            match size {
                                pci_capa::RegisterSize::Byte => {
                                    c.read_u8(reg_off as u64).ok().map(u64::from)
                                }
                                pci_capa::RegisterSize::Word => {
                                    c.read_u16(reg_off as u64).ok().map(u64::from)
                                }
                                pci_capa::RegisterSize::Dword => {
                                    c.read_u32(reg_off as u64).ok().map(u64::from)
                                }
                                pci_capa::RegisterSize::Qword => c.read_u64(reg_off as u64).ok(),
                            }
                        } else {
                            None
                        }
                    },
                    2,
                    2,
                    &mut children,
                );
                (cap.name.to_string(), children)
            } else {
                (
                    format!("Unknown (0x{id:02x})"),
                    vec![TreeNode::with_value(
                        TreeLine::from("  Offset"),
                        TreeLine::from(format!("0x{off:02x}")),
                    )],
                )
            };

        let mut cap_node = TreeNode::new_collapsed(TreeLine::from(name));
        cap_node.children = children;

        let mut data_node = TreeNode::new_collapsed(TreeLine::from("Data"));
        data_node.children = render_cap_data_nodes(pci_capa.as_ref(), *len as u16);
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
    let end = if next > start && next <= config.len() {
        next
    } else {
        config.len()
    };
    Some(((offset, version, id, (end - start) as u16), next))
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

fn build_extended_caps_nodes(config: Option<&PciConfig>, caps: &[ExtCapEntry]) -> Vec<TreeNode> {
    let mut nodes = Vec::new();
    for (offset, version, id, len) in caps {
        let pci_capa = config.map(|c| PciCapa::new(c, *offset as u64));
        let (title, children) = if let Some(cap) = pci_capa::EXT_CAP_REGISTRY
            .iter()
            .find(|cap| cap.id == *id && cap.version == *version)
        {
            let mut children = Vec::new();
            pci_capa::print_registers(
                cap.registers,
                |reg_off, size| {
                    if let Some(c) = &pci_capa {
                        match size {
                            pci_capa::RegisterSize::Byte => {
                                c.read_u8(reg_off as u64).ok().map(u64::from)
                            }
                            pci_capa::RegisterSize::Word => {
                                c.read_u16(reg_off as u64).ok().map(u64::from)
                            }
                            pci_capa::RegisterSize::Dword => {
                                c.read_u32(reg_off as u64).ok().map(u64::from)
                            }
                            pci_capa::RegisterSize::Qword => c.read_u64(reg_off as u64).ok(),
                        }
                    } else {
                        None
                    }
                },
                3,
                2,
                &mut children,
            );
            (format!("{} (v{})", cap.name, cap.version), children)
        } else {
            (
                format!("Unknown (0x{id:04x})"),
                vec![TreeNode::with_value(
                    TreeLine::from("  Version"),
                    TreeLine::from(version.to_string()),
                )],
            )
        };

        let mut cap_node = TreeNode::new_collapsed(TreeLine::from(title));
        cap_node.children = children;

        let mut data_node = TreeNode::new_collapsed(TreeLine::from("Data"));
        data_node.children = render_cap_data_nodes(pci_capa.as_ref(), *len);
        cap_node.add_child(data_node);

        nodes.push(cap_node);
    }
    nodes
}

fn render_cap_data_nodes(capa: Option<&PciCapa>, length: u16) -> Vec<TreeNode> {
    let mut nodes = Vec::new();
    let Some(cap) = capa else {
        return nodes;
    };

    let mut offset = 0;
    while offset < length {
        let mut hex_parts = Vec::new();
        for i in 0..16 {
            if offset + i >= length {
                break;
            }
            if let Ok(val) = cap.read_u8((offset + i) as u64) {
                hex_parts.push(format!("{:02x}", val));
            } else {
                hex_parts.push("??".to_string());
            }
        }

        if hex_parts.is_empty() {
            break;
        }

        let hex = hex_parts.join(" ");
        nodes.push(TreeNode::new(TreeLine::from(format!(
            "    {:04x}: {}",
            cap.base_offset + offset as u64,
            hex
        ))));
        offset += 16;
    }
    nodes
}
