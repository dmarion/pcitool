use crate::capabilities;

capabilities! {
    ext {
        id: 0x000a,
        version: 1,
        name: "RCRB Header",
        registers: [
            {
                name: "RCRB Vendor ID and Device ID",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Vendor ID", lsb: 0, bits: 16 },
                    { name: "Device ID", lsb: 16, bits: 16 },
                ]
            },
            {
                name: "RCRB Capabilities",
                offset: 0x08,
                size: Dword,
                fields: []
            },
            {
                name: "RCRB Control",
                offset: 0x0c,
                size: Dword,
                fields: []
            },
        ]
    }
}
