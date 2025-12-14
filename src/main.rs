use anyhow::{Result, anyhow};
use clap::Parser;
use rustix::process::{Uid, getuid};

mod cli;
mod dump;
mod pci_capa;
mod pci_classes;
mod pci_config_hdr;
mod pci_device;
mod pci_ids;
mod tree;
mod tui;

mod pci_ext_capa;
mod pci_std_capa;

pub const SYSFS_DEVICES: &str = "/sys/bus/pci/devices";
pub const ECH_BYTES: usize = 4;
pub const DDR_OFFSET: usize = 0x40;
pub const ECS_OFFSET: usize = 0x100;
pub const MIN_CONFIG_BYTES: usize = 256;
pub const CONFIG_READ_BYTES: usize = 4096;

use crate::cli::Args;

fn main() -> Result<()> {
    if getuid() != Uid::ROOT {
        return Err(anyhow!("pcitool must be run as root"));
    }

    let args = Args::parse();

    let all_addrs = pci_device::list_devices()?;

    if let Some(specified) = &args.address {
        if !all_addrs.contains(specified) {
            return Err(anyhow!("PCI address not found: {specified}"));
        }
    }

    let mut summaries = Vec::new();
    let targets = if args.dump {
        args.address
            .as_ref()
            .map(|addr| vec![addr.clone()])
            .unwrap_or(all_addrs)
    } else {
        all_addrs
    };

    for addr in &targets {
        match pci_device::summarize_device(addr) {
            Ok(summary) => summaries.push(summary),
            Err(err) => {
                if args.address.as_deref() == Some(addr) {
                    eprintln!("Warning: could not access device {}: {}", addr, err);
                }
            }
        }
    }

    if summaries.is_empty() {
        return Err(anyhow!("no PCI devices found or readable"));
    }

    if args.dump {
        for summary in &summaries {
            println!("Device: {}", summary.address);
            let tree = pci_device::get_device_tree(summary)?;
            dump::render(&tree);
            println!();
        }
    } else {
        // Find the index in the summaries list that corresponds to our initial choice
        let final_idx = args
            .address
            .as_deref()
            .and_then(|specified| summaries.iter().position(|s| s.address == specified))
            .unwrap_or(0);
        tui::run(summaries, final_idx, args.address.is_none())?;
    }

    Ok(())
}
