use crate::capabilities;

capabilities! {
    ext {
        id: 0x0035,
        version: 1,
        name: "Streamlined Virtual Channel",
        registers: [
            {
                name: "SVC Port Capability 1",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "SVC Port Control",
                offset: 0x0c,
                size: Word,
                fields: []
            },
            {
                name: "SVC Port Status",
                offset: 0x10,
                size: Word,
                fields: []
            },
        ]
    }
}
