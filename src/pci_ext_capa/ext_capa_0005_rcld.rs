use crate::capabilities;
use crate::pci_device::PciCapa;

fn get_size(capa: &PciCapa) -> Option<u16> {
    let desc = capa.read_u32(u64::from(ELEMENT_SELF_DESC)).ok()?;
    let entries = ((desc >> 8) & 0xff) as u16;
    let entries = entries.max(1);
    Some(0x10 + entries * 0x10)
}

capabilities! {
    {
        id: 0x0005,
        version: 1,
        is_extended: true,
        name: "Root Complex Link Declaration",
        get_size: get_size,
        registers: [
            {
                name: "Element Self Description",
                offset: 0x04,
                id: ELEMENT_SELF_DESC,
                size: Dword,
                fields: [
                    { name: "Element Type", lsb: 0, bits: 4 },
                    { name: "Number of Link Entries", lsb: 8, bits: 8 },
                    { name: "Component ID", lsb: 16, bits: 8 },
                    { name: "Port Number", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "Link Entry 1 Description",
                offset: 0x0c,
                id: LINK_ENTRY_1_DESC,
                size: Dword,
                fields: [
                    { name: "Link Valid", lsb: 0, bits: 1 },
                    { name: "Link Type", lsb: 1, bits: 1 },
                    { name: "Associate RCRB Header", lsb: 2, bits: 1 },
                    { name: "Target Component ID", lsb: 8, bits: 8 },
                    { name: "Target Port Number", lsb: 16, bits: 8 },
                ]
            },
            {
                name: "Link Entry 1 Address Low",
                offset: 0x10,
                id: LINK_ENTRY_1_ADDR_LOW,
                size: Dword,
                fields: []
            },
            {
                name: "Link Entry 1 Address High",
                offset: 0x14,
                id: LINK_ENTRY_1_ADDR_HIGH,
                size: Dword,
                fields: []
            },
        ]
    }
}
