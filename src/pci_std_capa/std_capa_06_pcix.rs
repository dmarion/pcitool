use crate::capabilities;

capabilities! {
    std {
        id: 0x06,
        name: "PCI-X",
        registers: [
            {
                name: "PCI-X Command",
                offset: 0x02,
                size: Word,
                fields: []
            },
            {
                name: "PCI-X Status",
                offset: 0x04,
                size: Dword,
                fields: []
            },
        ]
    }
}
