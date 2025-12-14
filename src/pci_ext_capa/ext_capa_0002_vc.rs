use crate::capabilities;

capabilities! {
    ext {
        id: 0x0002,
        version: 1,
        name: "Virtual Channel",
        registers: [
            {
                name: "VC Port VC Capability 1",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Extended VC count", lsb: 0, bits: 3 },
                    { name: "Low priority extended VC count", lsb: 4, bits: 3 },
                    {
                        name: "Arbitration table size",
                        lsb: 10,
                        bits: 2,
                        enum_values: [
                            (0x0, "32 phases"),
                            (0x1, "64 phases"),
                            (0x2, "128 phases"),
                        ]
                    },
                ]
            },
            {
                name: "VC Port VC Capability 2",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "32-phase arbitration", lsb: 1, bits: 1 },
                    { name: "64-phase arbitration", lsb: 2, bits: 1 },
                    { name: "128-phase arbitration", lsb: 3, bits: 1 },
                    { name: "Arbitration table offset", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "VC Port Control",
                offset: 0x0c,
                size: Dword,
                fields: [
                    { name: "Load VC Arbitration Table", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "VC Port Status",
                offset: 0x0e,
                size: Word,
                fields: [
                    { name: "Table completion status", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "VC Resource Capability",
                offset: 0x10,
                size: Dword,
                fields: [
                    { name: "32-phase arbitration", lsb: 1, bits: 1 },
                    { name: "64-phase arbitration", lsb: 2, bits: 1 },
                    { name: "128-phase arbitration", lsb: 3, bits: 1 },
                    { name: "128-phase arbitration table", lsb: 4, bits: 1 },
                    { name: "256-phase arbitration", lsb: 5, bits: 1 },
                    { name: "Arbitration table offset", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "VC Resource Control",
                offset: 0x14,
                size: Dword,
                fields: [
                    { name: "Load VC Arbitration Table", lsb: 16, bits: 1 },
                    {
                        name: "Arbitration Select",
                        lsb: 17,
                        bits: 3,
                        enum_values: [
                            (0x0, "Hardware fixed priority"),
                            (0x1, "Round robin"),
                            (0x2, "Weighted round robin"),
                            (0x3, "Weighted round robin (VC resource)"),
                        ]
                    },
                    { name: "VC ID", lsb: 24, bits: 3 },
                    { name: "TC/VC Map Enable", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "VC Resource Status",
                offset: 0x1a,
                size: Word,
                fields: [
                    { name: "Table completion status", lsb: 0, bits: 1 },
                    { name: "Negotiation Pending", lsb: 1, bits: 1 },
                ]
            },
        ]
    }
}
