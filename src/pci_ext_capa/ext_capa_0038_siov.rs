use crate::capabilities;

capabilities! {
    {
        id: 0x0038,
        version: 1,
        is_extended: true,
        name: "Shared I/O Virtualization",
        size: 16,
        registers: [
            {
                name: "SIOV Capabilities",
                offset: 0x04,
                id: SIOV_CAPS,
                size: Dword,
                fields: []
            },
            {
                name: "Total SDIs",
                offset: 0x08,
                id: TOTAL_SDIS,
                size: Word,
                fields: []
            },
            {
                name: "SIOV Status",
                offset: 0x0b,
                id: SIOV_STAT,
                size: Byte,
                fields: []
            },
            {
                name: "First SDI Offset",
                offset: 0x0c,
                id: FIRST_SDI_OFF,
                size: Word,
                fields: []
            },
            {
                name: "SDI Stride",
                offset: 0x0e,
                id: SDI_STRIDE,
                size: Word,
                fields: []
            },
        ]
    }
}
