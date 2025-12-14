use crate::capabilities;

capabilities! {
    ext {
        id: 0x0004,
        version: 1,
        name: "Power Budgeting",
        registers: [
            {
                name: "Data Select Register",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "Control",
                offset: 0x06,
                size: Word,
                fields: [
                    { name: "Reserved", lsb: 0, bits: 16 },
                ]
            },
            {
                name: "Data Register",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "Base Power", lsb: 0, bits: 8 },
                    { name: "Data Scale", lsb: 8, bits: 2 },
                    { name: "PM Sub State", lsb: 10, bits: 3 },
                    { name: "PM State", lsb: 13, bits: 2 },
                    { name: "Type", lsb: 15, bits: 3 },
                    { name: "Power Rail", lsb: 18, bits: 3 },
                ]
            },
            {
                name: "Capability",
                offset: 0x0c,
                size: Dword,
                fields: [
                    { name: "Included in system budget", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
