use crate::capabilities;

capabilities! {
    {
        id: 0x0004,
        version: 1,
        is_extended: true,
        name: "Power Budgeting",
        size: 16,
        registers: [
            {
                name: "Data Select Register",
                offset: 0x04,
                id: DATA_SELECT_REG,
                size: Dword,
                fields: []
            },
            {
                name: "Control",
                offset: 0x06,
                id: CTRL,
                size: Word,
                fields: [
                    { name: "Reserved", lsb: 0, bits: 16 },
                ]
            },
            {
                name: "Data Register",
                offset: 0x08,
                id: DATA_REG,
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
                id: CAP,
                size: Dword,
                fields: [
                    { name: "Included in system budget", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
