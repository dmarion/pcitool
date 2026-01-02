use crate::capabilities;
use crate::pci_device::PciCapa;

capabilities! {
    {
        id: 0x000b,
        version: 1,
        is_extended: true,
        name: "Vendor Specific",
        get_size: get_size,
        registers: [
            {
                name: "Vendor-Specific Header",
                offset: 0x04,
                id: VENDOR_SPECIFIC_HEADER,
                size: Dword,
                fields: [
                    { name: "Vendor-Specific ID", lsb: 0, bits: 16 },
                    { name: "Vendor-Specific Revision", lsb: 16, bits: 4 },
                    { name: "Vendor-Specific Length", lsb: 20, bits: 12 },
                ]
            },
        ]
    }
}

fn get_size(cap: &PciCapa) -> Option<u16> {
    let val = cap.read_u32(u64::from(VENDOR_SPECIFIC_HEADER)).ok()?;
    let len = (val >> 20) & 0xfff;
    if len < 8 {
        return Some(8);
    }
    Some(len as u16)
}
