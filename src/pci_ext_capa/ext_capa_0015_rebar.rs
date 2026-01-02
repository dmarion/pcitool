use crate::capabilities;
use crate::pci_device::PciCapa;

capabilities! {
    {
        id: 0x0015,
        version: 1,
        is_extended: true,
        name: "Resizable BAR",
        get_size: get_size,
        registers: [
            {
                name: "Capability",
                offset: 0x04,
                id: CAP,
                size: Dword,
                fields: [
                    { name: "Supported BAR sizes", lsb: 4, bits: 28 },
                ]
            },
            {
                name: "Control",
                offset: 0x08,
                id: CTRL,
                size: Dword,
                fields: [
                    { name: "BAR Index", lsb: 0, bits: 3 },
                    { name: "Number of Resizable BARs", lsb: 5, bits: 3 },
                    { name: "BAR Size", lsb: 8, bits: 5 },
                ]
            },
        ]
    }
}

fn get_size(cap: &PciCapa) -> Option<u16> {
    let ctrl = cap.read_u32(u64::from(CTRL)).ok()?;
    let num_bars = ((ctrl >> 5) & 0x07) as u16 + 1;
    Some(4 + num_bars * 8)
}
