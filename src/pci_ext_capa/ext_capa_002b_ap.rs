use crate::capabilities;
use crate::pci_device::PciCapa;

fn get_size(capa: &PciCapa) -> Option<u16> {
    let caps = capa.read_u32(u64::from(ALTERNATE_PROTOCOL_CAPS)).ok()?;
    let count = (caps & 0xff) as u32;
    let selective = ((caps >> 8) & 0x1) != 0;
    if !selective {
        return Some(0x14);
    }
    let mask_words = ((count + 31) / 32).max(1);
    let mask_bytes = u16::try_from(mask_words * 4).ok()?;
    mask_bytes.checked_add(0x14)
}

capabilities! {
    {
        id: 0x002b,
        version: 1,
        is_extended: true,
        name: "Alternate Protocol",
        get_size: get_size,
        registers: [
            {
                name: "Alternate Protocol Capabilities",
                offset: 0x04,
                id: ALTERNATE_PROTOCOL_CAPS,
                size: Dword,
                fields: [
                    { name: "Alternate Protocol Count", lsb: 0, bits: 8 },
                    { name: "Alternate Protocol Selective Enable Supported", lsb: 8, bits: 1 },
                ]
            },
            {
                name: "Alternate Protocol Control",
                offset: 0x08,
                id: ALTERNATE_PROTOCOL_CTRL,
                size: Dword,
                fields: [
                    { name: "Alternate Protocol Index Select", lsb: 0, bits: 8 },
                ]
            },
            { name: "Alternate Protocol Data 1", offset: 0x0c, id: ALTERNATE_PROTOCOL_DATA1, size: Dword, fields: [] },
            { name: "Alternate Protocol Data 2", offset: 0x10, id: ALTERNATE_PROTOCOL_DATA2, size: Dword, fields: [] },
            { name: "Alternate Protocol Selective Enable Mask", offset: 0x14, id: ALTERNATE_PROTOCOL_SELECTIVE_ENABLE_MASK, size: Dword, fields: [] },
        ]
    }
}
