use crate::capabilities;

capabilities! {
    std {
        id: 0x05,
        name: "Message Signaled Interrupts",
        registers: [
            {
                name: "Message Control",
                offset: 0x02,
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
                size: Dword,
                fields: []
            },
            {
                name: "Message Upper Address",
                offset: 0x08,
                size: Dword,
                fields: []
            },
            {
                name: "Message Data",
                offset: 0x0c,
                size: Word,
                fields: []
            },
            {
                name: "Mask Bits",
                offset: 0x10,
                size: Dword,
                fields: []
            },
            {
                name: "Pending Bits",
                offset: 0x14,
                size: Dword,
                fields: []
            },
        ]
    }
}
