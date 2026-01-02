use crate::capabilities;
use crate::pci_device::PciCapa;

capabilities! {
    {
        id: 0x0023,
        version: 1,
        is_extended: true,
        name: "Designated Vendor-Specific",
        get_size: get_size,
        registers: [
            {
                name: "Designated Vendor-Specific Header 1",
                offset: 0x04,
                id: DESIGNATED_VENDOR_SPECIFIC_HEADER1,
                size: Dword,
                fields: [
                    { name: "Vendor ID", lsb: 0, bits: 16 },
                    { name: "DVSEC Revision", lsb: 16, bits: 4 },
                    { name: "DVSEC Length", lsb: 20, bits: 12 },
                ]
            },
            {
                name: "Designated Vendor-Specific Header 2",
                offset: 0x08,
                id: DESIGNATED_VENDOR_SPECIFIC_HEADER2,
                size: Dword,
                fields: [
                    { name: "Designated Vendor-Specific ID", lsb: 0, bits: 16 },
                ]
            },
        ]
    }
}

fn get_size(cap: &PciCapa) -> Option<u16> {
    let val = cap
        .read_u32(u64::from(DESIGNATED_VENDOR_SPECIFIC_HEADER1))
        .ok()?;
    let len = (val >> 20) & 0xfff;
    // Length must be at least 8 (header)
    if len < 8 {
        return Some(8);
    }
    Some(len as u16)
}
