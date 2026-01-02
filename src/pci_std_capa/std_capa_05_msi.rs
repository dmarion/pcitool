use crate::capabilities;
use crate::pci_device::PciCapa;

capabilities! {
    {
        id: 0x05,
        name: "Message Signaled Interrupts",
        get_size: get_size,
        registers: [
            {
                name: "Message Control",
                offset: 0x02,
                id: MESSAGE_CTRL,
                size: Word,
                fields: [
                    { name: "MSI Enable", lsb: 0, bits: 1 },
                    {
                        name: "Multiple Message Capable",
                        lsb: 1,
                        bits: 3,
                        enum_values: [
                            (0x0, "1"),
                            (0x1, "2"),
                            (0x2, "4"),
                            (0x3, "8"),
                            (0x4, "16"),
                            (0x5, "32"),
                        ]
                    },
                    {
                        name: "Multiple Message Enable",
                        lsb: 4,
                        bits: 3,
                        enum_values: [
                            (0x0, "1"),
                            (0x1, "2"),
                            (0x2, "4"),
                            (0x3, "8"),
                            (0x4, "16"),
                            (0x5, "32"),
                        ]
                    },
                    { name: "64-bit Address Capable", lsb: 7, bits: 1 },
                    { name: "Per-Vector Masking Capable", lsb: 8, bits: 1 },
                ]
            },
            {
                name: "Message Address",
                offset: 0x04,
                id: MESSAGE_ADDR,
                size: Dword,
                fields: []
            },
            {
                name: "Message Upper Address",
                offset: 0x08,
                id: MESSAGE_UPPER_ADDR,
                size: Dword,
                fields: []
            },
            {
                name: "Message Data",
                offset: 0x0c,
                id: MESSAGE_DATA,
                size: Word,
                fields: []
            },
            {
                name: "Mask Bits",
                offset: 0x10,
                id: MASK_BITS,
                size: Dword,
                fields: []
            },
            {
                name: "Pending Bits",
                offset: 0x14,
                id: PENDING_BITS,
                size: Dword,
                fields: []
            },
        ]
    }
}

fn get_size(cap: &PciCapa) -> Option<u16> {
    let ctrl = cap.read_u16(u64::from(MESSAGE_CTRL)).ok()?;
    let is_64bit = (ctrl >> 7) & 0x1 != 0;
    let has_mask = (ctrl >> 8) & 0x1 != 0;
    let mut len = 10;
    if is_64bit {
        len += 4;
    }
    if has_mask {
        len += 10;
    }
    Some(len)
}
