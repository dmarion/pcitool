use crate::capabilities;

capabilities! {
    ext {
        id: 0x001a,
        version: 1,
        name: "Protocol Multiplexing",
        registers: [
            {
                name: "PMUX Capability",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "PMUX Control",
                offset: 0x08,
                size: Dword,
                fields: []
            },
            {
                name: "PMUX Status",
                offset: 0x0c,
                size: Dword,
                fields: []
            },
        ]
    }
}
