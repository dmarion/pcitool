use crate::capabilities;
use crate::pci_device::PciCapa;

fn get_size(capa: &PciCapa) -> Option<u16> {
    let caps = capa.read_u32(u64::from(TPH_CAPS)).ok()?;
    let st_loc = (caps >> 9) & 0x3;
    if st_loc == 0x1 {
        let entries = ((caps >> 16) & 0x7ff) + 1;
        return u16::try_from(0x0c + entries * 2).ok();
    }
    Some(0x0c)
}

capabilities! {
    {
        id: 0x0017,
        version: 1,
        is_extended: true,
        name: "Transaction Processing Hints",
        get_size: get_size,
        registers: [
            {
                name: "TPH Capabilities",
                offset: 0x04,
                id: TPH_CAPS,
                size: Dword,
                fields: [
                    { name: "No ST Mode Supported", lsb: 0, bits: 1 },
                    { name: "Interrupt Vector Mode Supported", lsb: 1, bits: 1 },
                    { name: "Device Specific Mode Supported", lsb: 2, bits: 1 },
                    { name: "Extended TPH Requester Supported", lsb: 8, bits: 1 },
                    {
                        name: "ST Table Location",
                        lsb: 9,
                        bits: 2,
                        enum_values: [
                            (0x0, "None"),
                            (0x1, "TPH Requestor Capability"),
                            (0x2, "MSI-X Table"),
                        ]
                    },
                    { name: "ST Table Size", lsb: 16, bits: 11 },
                ]
            },
            {
                name: "TPH Control",
                offset: 0x08,
                id: TPH_CTRL,
                size: Dword,
                fields: [
                    {
                        name: "ST Mode Select",
                        lsb: 0,
                        bits: 3,
                        enum_values: [
                            (0x0, "No ST Mode"),
                            (0x1, "Interrupt Vector Mode"),
                            (0x2, "Device Specific Mode"),
                        ]
                    },
                    { name: "TPH Requester Enable", lsb: 8, bits: 2 },
                ]
            },
        ]
    }
}
