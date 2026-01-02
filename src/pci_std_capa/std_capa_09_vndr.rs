use crate::capabilities;
use crate::pci_device::PciCapa;

capabilities! {
    {
        id: 0x09,
        name: "Vendor Specific",
        get_size: get_size,
        registers: [
            {
                name: "Capability Length",
                offset: 0x02,
                id: CAP_LENGTH,
                size: Byte,
                fields: []
            }
        ]
    }
}

fn get_size(cap: &PciCapa) -> Option<u16> {
    let len = cap.read_u8(u64::from(CAP_LENGTH)).ok()? as u16;
    if len < 3 {
        return None;
    }
    Some(len)
}
