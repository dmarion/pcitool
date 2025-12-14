use crate::capabilities;

capabilities! {
    ext {
        id: 0x0015,
        version: 1,
        name: "Resizable BAR",
        registers: [
            {
                name: "Capability",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Supported BAR sizes", lsb: 4, bits: 28 },
                ]
            },
            {
                name: "Control",
                offset: 0x08,
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
