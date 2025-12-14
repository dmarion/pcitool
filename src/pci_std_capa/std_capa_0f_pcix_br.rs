use crate::capabilities;

capabilities! {
    std {
        id: 0x0f,
        name: "PCI-X Bridge",
        registers: [
            {
                name: "Secondary Status",
                offset: 0x02,
                size: Word,
                fields: []
            },
            {
                name: "Bridge Status",
                offset: 0x04,
                size: Dword,
                fields: []
            },
        ]
    }
}
