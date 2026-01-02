use crate::capabilities;
use crate::pci_device::PciCapa;

capabilities! {
    {
        id: 0x14,
        name: "Enhanced Allocation",
        get_size: get_size,
        registers: [
            {
                name: "EA Capabilities",
                offset: 0x02,
                id: EA_CAPS,
                size: Word,
                fields: [
                    { name: "Number of Entries", lsb: 0, bits: 6 },
                ]
            },
        ]
    }
}

fn get_size(cap: &PciCapa) -> Option<u16> {
    let caps = cap.read_u16(u64::from(EA_CAPS)).ok()?;
    let entries = (caps & 0x3f) as u16;
    let header_type = cap.read_cfg_u8(0x0e).ok()? & 0x7f;
    let mut offset = if header_type == 0x01 { 8u16 } else { 4u16 };
    let mut total = offset;
    for _ in 0..entries {
        let entry = cap.read_u32(offset as u64).ok()?;
        let entry_size = (entry & 0x07) as u16;
        let entry_len = 4 + entry_size * 4;
        total = total.saturating_add(entry_len);
        offset = offset.saturating_add(entry_len);
    }
    Some(total)
}
