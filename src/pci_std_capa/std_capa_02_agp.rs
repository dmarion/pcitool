use crate::capabilities;

capabilities! {
    {
        id: 0x02,
        name: "AGP",
        size: 12,
        registers: [
            {
                name: "AGP Status",
                offset: 0x04,
                id: AGP_STAT,
                size: Dword,
                fields: []
            },
            {
                name: "AGP Command",
                offset: 0x08,
                id: AGP_CMD,
                size: Dword,
                fields: []
            },
        ]
    }
}
