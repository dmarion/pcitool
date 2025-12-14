use crate::capabilities;

capabilities! {
    ext {
        id: 0x0032,
        version: 1,
        name: "Flit Logging",
        registers: [
            {
                name: "Flit Logging Capability",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Flit Log FIFO Depth", lsb: 0, bits: 8 },
                ]
            },
            {
                name: "Flit Logging Status",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "Flit Log Valid", lsb: 0, bits: 1 },
                    { name: "Flit Log Overflow", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "Flit Logging Control",
                offset: 0x0c,
                size: Dword,
                fields: [
                    { name: "Flit Log Enable", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
