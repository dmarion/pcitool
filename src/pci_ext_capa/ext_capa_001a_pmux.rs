use crate::capabilities;

capabilities! {
    {
        id: 0x001a,
        version: 1,
        is_extended: true,
        name: "Protocol Multiplexing",
        size: 16,
        registers: [
            {
                name: "PMUX Capability",
                offset: 0x04,
                id: PMUX_CAP,
                size: Dword,
                fields: []
            },
            {
                name: "PMUX Control",
                offset: 0x08,
                id: PMUX_CTRL,
                size: Dword,
                fields: []
            },
            {
                name: "PMUX Status",
                offset: 0x0c,
                id: PMUX_STAT,
                size: Dword,
                fields: []
            },
        ]
    }
}
