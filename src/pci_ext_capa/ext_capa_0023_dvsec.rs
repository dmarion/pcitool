use crate::capabilities;

capabilities! {
    ext {
        id: 0x0023,
        version: 1,
        name: "Designated Vendor-Specific",
        registers: [
            {
                name: "Designated Vendor-Specific Header 1",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Vendor ID", lsb: 0, bits: 16 },
                    { name: "DVSEC Revision", lsb: 16, bits: 4 },
                    { name: "DVSEC Length", lsb: 20, bits: 12 },
                ]
            },
            {
                name: "Designated Vendor-Specific Header 2",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "Designated Vendor-Specific ID", lsb: 0, bits: 16 },
                ]
            },
        ]
    }
}
