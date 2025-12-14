use crate::capabilities;

capabilities! {
    ext {
        id: 0x0030,
        version: 1,
        name: "Integrity and Data Encryption",
        registers: [
            {
                name: "IDE Capability",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Link IDE Supported", lsb: 0, bits: 1 },
                    { name: "Selective IDE Supported", lsb: 1, bits: 1 },
                    { name: "Flow Through IDE Supported", lsb: 2, bits: 1 },
                    { name: "Partial Header Encryption Supported", lsb: 3, bits: 1 },
                    { name: "Agile IDE Supported", lsb: 4, bits: 1 },
                    { name: "Max Number of Link IDE Streams", lsb: 8, bits: 3 },
                    { name: "Max Number of Selective IDE Streams", lsb: 11, bits: 8 },
                ]
            },
            {
                name: "IDE Control",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "Flow Through IDE Enable", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
