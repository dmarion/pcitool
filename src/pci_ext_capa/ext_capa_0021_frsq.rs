use crate::capabilities;

capabilities! {
    ext {
        id: 0x0021,
        version: 1,
        name: "FRS Queueing",
        registers: [
            {
                name: "FRS Queueing Capability",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "FRS Queue Max Depth", lsb: 0, bits: 12 },
                    { name: "FRS Interrupt Message Number", lsb: 16, bits: 5 },
                ]
            },
            {
                name: "FRS Queueing Status",
                offset: 0x08,
                size: Word,
                fields: [
                    { name: "FRS Message Received", lsb: 0, bits: 1 },
                    { name: "FRS Message Overflow", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "FRS Queueing Control",
                offset: 0x0a,
                size: Word,
                fields: [
                    { name: "FRS Interrupt Enable", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "FRS Message Queue",
                offset: 0x0c,
                size: Dword,
                fields: []
            },
        ]
    }
}
