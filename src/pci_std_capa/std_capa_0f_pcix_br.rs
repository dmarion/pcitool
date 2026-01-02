use crate::capabilities;

capabilities! {
    {
        id: 0x0f,
        name: "PCI-X Bridge",
        size: 8,
        registers: [
            {
                name: "Secondary Status",
                offset: 0x02,
                id: SECONDARY_STAT,
                size: Word,
                fields: []
            },
            {
                name: "Bridge Status",
                offset: 0x04,
                id: BRIDGE_STAT,
                size: Dword,
                fields: []
            },
        ]
    }
}
