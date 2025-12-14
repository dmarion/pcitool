use crate::capabilities;

capabilities! {
    ext {
        id: 0x0038,
        version: 1,
        name: "Shared I/O Virtualization",
        registers: [
            {
                name: "SIOV Capabilities",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "Total SDIs",
                offset: 0x08,
                size: Word,
                fields: []
            },
            {
                name: "SIOV Status",
                offset: 0x0b,
                size: Byte,
                fields: []
            },
            {
                name: "First SDI Offset",
                offset: 0x0c,
                size: Word,
                fields: []
            },
            {
                name: "SDI Stride",
                offset: 0x0e,
                size: Word,
                fields: []
            },
        ]
    }
}
