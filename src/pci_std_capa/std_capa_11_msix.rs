use crate::capabilities;

capabilities! {
    {
        id: 0x11,
        name: "MSI-X",
        size: 12,
        registers: [
            {
                name: "Message Control",
                offset: 0x02,
                id: MESSAGE_CTRL,
                size: Word,
                fields: [
                    { name: "Table Size (n-1)", lsb: 0, bits: 11 },
                    { name: "Function Mask", lsb: 14, bits: 1 },
                    { name: "MSI-X Enable", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Table",
                offset: 0x04,
                id: TABLE,
                size: Dword,
                fields: [
                    { name: "BAR Indicator", lsb: 0, bits: 3 },
                    { name: "Table Offset", lsb: 3, bits: 29 },
                ]
            },
            {
                name: "PBA",
                offset: 0x08,
                id: PBA,
                size: Dword,
                fields: [
                    { name: "BAR Indicator", lsb: 0, bits: 3 },
                    { name: "PBA Offset", lsb: 3, bits: 29 },
                ]
            },
        ]
    }
}
