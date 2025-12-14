use std::{collections::HashMap, fs, sync::OnceLock};

const PCI_IDS_PATH: &str = "/usr/share/misc/pci.ids";

struct PciIds {
    vendor_names: HashMap<u16, String>,
    device_names: HashMap<(u16, u16), String>,
}

static PCI_IDS: OnceLock<Option<PciIds>> = OnceLock::new();

fn pci_ids() -> Option<&'static PciIds> {
    PCI_IDS.get_or_init(load_pci_ids).as_ref()
}

fn load_pci_ids() -> Option<PciIds> {
    let content = fs::read_to_string(PCI_IDS_PATH).ok()?;
    let mut vendor_names = HashMap::new();
    let mut device_names = HashMap::new();
    let mut current_vendor: Option<u16> = None;

    for line in content.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if !line.starts_with(|c: char| c.is_whitespace()) {
            let mut parts = line.splitn(2, char::is_whitespace);
            let id_str = parts.next().unwrap_or_default();
            let name = parts.next().unwrap_or("").trim();
            if let Ok(id) = u16::from_str_radix(id_str, 16) {
                vendor_names.insert(id, name.to_string());
                current_vendor = Some(id);
            } else {
                current_vendor = None;
            }
        } else if line.starts_with('\t')
            && !line.starts_with("\t\t")
            && let Some(vendor) = current_vendor
        {
            let trimmed = line.trim();
            let mut parts = trimmed.splitn(2, char::is_whitespace);
            let dev_str = parts.next().unwrap_or_default();
            let name = parts.next().unwrap_or("").trim();
            if let Ok(dev_id) = u16::from_str_radix(dev_str, 16) {
                device_names.insert((vendor, dev_id), name.to_string());
            }
        }
    }

    Some(PciIds {
        vendor_names,
        device_names,
    })
}

pub fn name_suffix(vendor_id: u16, device_id: u16) -> String {
    pci_ids()
        .and_then(|ids| ids.describe_parts(vendor_id, device_id))
        .map(|(vendor, device)| {
            let rest = device.map(|d| format!(" {d}")).unwrap_or_default();
            format!(" ({vendor}{rest})")
        })
        .unwrap_or_default()
}

pub fn vendor_name(vendor_id: u16) -> Option<&'static str> {
    pci_ids()
        .and_then(|ids| ids.vendor_names.get(&vendor_id))
        .map(String::as_str)
}

pub fn device_name(vendor_id: u16, device_id: u16) -> Option<&'static str> {
    pci_ids()
        .and_then(|ids| ids.device_names.get(&(vendor_id, device_id)))
        .map(String::as_str)
}

impl PciIds {
    fn describe_parts(&self, vendor_id: u16, device_id: u16) -> Option<(String, Option<String>)> {
        let vendor_name = self.vendor_names.get(&vendor_id)?.clone();
        let device_name = self
            .device_names
            .get(&(vendor_id, device_id))
            .cloned()
            .filter(|s| !s.is_empty());
        Some((vendor_name, device_name))
    }
}
