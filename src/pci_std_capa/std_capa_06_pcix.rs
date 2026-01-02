use crate::capabilities;

capabilities! {
    {
        id: 0x06,
        name: "PCI-X",
        size: 8,
        registers: [
            {
                name: "PCI-X Command",
                offset: 0x02,
                id: PCI_X_CMD,
                size: Word,
                fields: []
            },
            {
                name: "PCI-X Status",
                offset: 0x04,
                id: PCI_X_STAT,
                size: Dword,
                fields: []
            },
        ]
    }
}
