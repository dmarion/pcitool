use crate::capabilities;

capabilities! {
    ext {
        id: 0x002b,
        version: 1,
        name: "Alternate Protocol",
        registers: [
            {
                name: "Alternate Protocol Capabilities",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Alternate Protocol Count", lsb: 0, bits: 8 },
                    { name: "Alternate Protocol Selective Enable Supported", lsb: 8, bits: 1 },
                ]
            },
            {
                name: "Alternate Protocol Control",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "Alternate Protocol Index Select", lsb: 0, bits: 8 },
                ]
            },
            { name: "Alternate Protocol Data 1", offset: 0x0c, size: Dword, fields: [] },
            { name: "Alternate Protocol Data 2", offset: 0x10, size: Dword, fields: [] },
            { name: "Alternate Protocol Selective Enable Mask", offset: 0x14, size: Dword, fields: [] },
        ]
    }
}
