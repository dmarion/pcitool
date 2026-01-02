use crate::capabilities;
use crate::pci_device::PciCapa;
use crate::tree::{TreeLine, TreeNode};

capabilities! {
    {
        id: 0x0003,
        version: 1,
        is_extended: true,
        name: "Device Serial Number",
        size: 12,
        summary: serial_summary,
        registers: [
            {
                name: "Serial Number",
                offset: 0x04,
                id: SERIAL_NUMBER,
                size: Qword,
                fields: []
            }
        ]
    }
}

fn serial_summary(cap: &PciCapa) -> Option<Vec<TreeNode>> {
    let serial_raw = cap.read_u64(u64::from(SERIAL_NUMBER)).ok()?;
    let serial_bytes = serial_raw.to_le_bytes();
    let serial = serial_bytes
        .iter()
        .rev()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("-");
    Some(vec![TreeNode::with_value(
        TreeLine::from("Serial Number"),
        TreeLine::from(serial),
    )])
}
