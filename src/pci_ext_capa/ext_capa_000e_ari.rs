use crate::capabilities;

capabilities! {
    ext {
        id: 0x000e,
        version: 1,
        name: "Alternative Routing-ID Interpretation",
        registers: [
            {
                name: "ARI Capability",
                offset: 0x04,
                size: Word,
                fields: [
                    { name: "MFVC Function Groups Capability", lsb: 0, bits: 1 },
                    { name: "ACS Function Groups Capability", lsb: 1, bits: 1 },
                    { name: "Next Function Number", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "ARI Control",
                offset: 0x06,
                size: Word,
                fields: [
                    { name: "MFVC Function Groups Enable", lsb: 0, bits: 1 },
                    { name: "ACS Function Groups Enable", lsb: 1, bits: 1 },
                    { name: "Function Group", lsb: 4, bits: 3 },
                ]
            },
        ]
    }
}
