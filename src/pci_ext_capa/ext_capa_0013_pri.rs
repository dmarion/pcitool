use crate::capabilities;

capabilities! {
    ext {
        id: 0x0013,
        version: 1,
        name: "Page Request Interface",
        registers: [
            {
                name: "PRI Control",
                offset: 0x04,
                size: Word,
                fields: [
                    { name: "Enable", lsb: 0, bits: 1 },
                    { name: "Reset", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "PRI Status",
                offset: 0x06,
                size: Word,
                fields: [
                    { name: "Response Failure", lsb: 0, bits: 1 },
                    { name: "Unexpected PRG index", lsb: 1, bits: 1 },
                    { name: "PRI Stopped", lsb: 8, bits: 1 },
                    { name: "PRG Response PASID Required", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "PRI Max Request Supported",
                offset: 0x08,
                size: Dword,
                fields: []
            },
            {
                name: "PRI Max Request Allowed",
                offset: 0x0c,
                size: Dword,
                fields: []
            },
        ]
    }
}
