use crate::capabilities;

capabilities! {
    {
        id: 0x08,
        name: "HyperTransport",
        size: 8,
        registers: [
            {
                name: "Command",
                offset: 0x02,
                id: CMD,
                size: Word,
                fields: [
                    {
                        name: "Type",
                        lsb: 0,
                        bits: 4,
                        enum_values: [
                            (0x0, "Slave/Primary"),
                            (0x2, "Host/Secondary"),
                            (0x4, "Interrupt"),
                            (0x5, "Revision ID"),
                            (0x6, "Unit ID Clumping"),
                            (0x7, "Extended Configuration"),
                            (0x8, "MSI Mapping"),
                            (0x9, "Direct Route"),
                            (0xa, "VC Mapping"),
                            (0xb, "Retry Mode"),
                            (0xc, "X86 Descriptors"),
                            (0xd, "Generic"),
                        ]
                    },
                    { name: "Slave Primary", lsb: 4, bits: 1 },
                    { name: "Device Number", lsb: 5, bits: 5 },
                    { name: "Link Number", lsb: 10, bits: 2 },
                    { name: "Virtual Channel", lsb: 12, bits: 1 },
                    { name: "Extended CTL", lsb: 13, bits: 1 },
                    { name: "Error Handling", lsb: 14, bits: 1 },
                    { name: "Protocol Error", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Link Control",
                offset: 0x04,
                id: LINK_CTRL,
                size: Word,
                fields: [
                    {
                        name: "Max Link Width In",
                        lsb: 0,
                        bits: 4,
                        enum_values: [
                            (0x0, "8-bit"),
                            (0x1, "16-bit"),
                            (0x3, "32-bit"),
                            (0x4, "2-bit"),
                            (0x5, "4-bit"),
                            (0x7, "Disconnected"),
                        ]
                    },
                    {
                        name: "Max Link Width Out",
                        lsb: 4,
                        bits: 4,
                        enum_values: [
                            (0x0, "8-bit"),
                            (0x1, "16-bit"),
                            (0x3, "32-bit"),
                            (0x4, "2-bit"),
                            (0x5, "4-bit"),
                            (0x7, "Disconnected"),
                        ]
                    },
                    {
                        name: "Link Width In",
                        lsb: 8,
                        bits: 4,
                        enum_values: [
                            (0x0, "8-bit"),
                            (0x1, "16-bit"),
                            (0x3, "32-bit"),
                            (0x4, "2-bit"),
                            (0x5, "4-bit"),
                            (0x7, "Disconnected"),
                        ]
                    },
                    {
                        name: "Link Width Out",
                        lsb: 12,
                        bits: 4,
                        enum_values: [
                            (0x0, "8-bit"),
                            (0x1, "16-bit"),
                            (0x3, "32-bit"),
                            (0x4, "2-bit"),
                            (0x5, "4-bit"),
                            (0x7, "Disconnected"),
                        ]
                    }
                ]
            },
        ]
    }
}
