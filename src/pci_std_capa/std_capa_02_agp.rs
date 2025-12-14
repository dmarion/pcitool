use crate::capabilities;

capabilities! {
    std {
        id: 0x02,
        name: "AGP",
        registers: [
            {
                name: "AGP Status",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "AGP Command",
                offset: 0x08,
                size: Dword,
                fields: []
            },
        ]
    }
}
