use crate::capabilities;
use crate::tree::PciNode;
use ratatui::text::Line;

capabilities! {
    ext {
        id: 0x0003,
        version: 1,
        name: "Device Serial Number",
        summary: serial_summary,
        registers: [
            {
                name: "Serial Number",
                offset: 0x04,
                size: Qword,
                fields: []
            }
        ]
    }
}

fn serial_summary(_offset: u16, _version: u8, bytes: &[u8]) -> Option<Vec<PciNode>> {
    let data = bytes.get(4..12)?;
    let serial = data
        .iter()
        .rev()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("-");
    Some(vec![PciNode::with_value(
        Line::from("Serial Number"),
        Line::from(serial),
    )])
}
