use crate::capabilities;

capabilities! {
    ext {
        id: 0x0005,
        version: 1,
        name: "Root Complex Link Declaration",
        registers: [
            {
                name: "Element Self Description",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Element Type", lsb: 0, bits: 4 },
                    { name: "Number of Link Entries", lsb: 8, bits: 8 },
                    { name: "Component ID", lsb: 16, bits: 8 },
                    { name: "Port Number", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "Link Entry 1 Description",
                offset: 0x0c,
                size: Dword,
                fields: [
                    { name: "Link Valid", lsb: 0, bits: 1 },
                    { name: "Link Type", lsb: 1, bits: 1 },
                    { name: "Associate RCRB Header", lsb: 2, bits: 1 },
                    { name: "Target Component ID", lsb: 8, bits: 8 },
                    { name: "Target Port Number", lsb: 16, bits: 8 },
                ]
            },
            {
                name: "Link Entry 1 Address Low",
                offset: 0x10,
                size: Dword,
                fields: []
            },
            {
                name: "Link Entry 1 Address High",
                offset: 0x14,
                size: Dword,
                fields: []
            },
        ]
    }
}
