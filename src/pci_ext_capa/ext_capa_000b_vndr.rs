use crate::capabilities;

capabilities! {
    ext {
        id: 0x000b,
        version: 1,
        name: "Vendor Specific",
        registers: [
            {
                name: "Vendor-Specific Header",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Vendor-Specific ID", lsb: 0, bits: 16 },
                    { name: "Vendor-Specific Revision", lsb: 16, bits: 4 },
                    { name: "Vendor-Specific Length", lsb: 20, bits: 12 },
                ]
            },
        ]
    }
}
