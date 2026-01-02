use crate::capabilities;
use crate::pci_device::PciCapa;

fn get_size(capa: &PciCapa) -> Option<u16> {
    let caps = capa.read_u32(u64::from(MRBL_CAPS)).ok()?;
    let len = (caps & 0x0fff) as u16;
    (len > 0).then_some(len)
}

capabilities! {
    {
        id: 0x0036,
        version: 1,
        is_extended: true,
        name: "MMIO Register Block Locator",
        get_size: get_size,
        registers: [
            {
                name: "MRBL Capabilities",
                offset: 0x04,
                id: MRBL_CAPS,
                size: Dword,
                fields: []
            },
        ]
    }
}
