use crate::capabilities;

capabilities! {
    ext {
        id: 0x0009,
        version: 1,
        name: "Virtual Channel (Secondary)",
        registers: [
            {
                name: "Port VC Capability 1",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Extended VC Count", lsb: 0, bits: 3 },
                    { name: "Port Arbitration Capability", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Port VC Capability 2",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "VC Arbitration Capability", lsb: 0, bits: 8 },
                    { name: "VC Arbitration Table Offset", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "Port VC Control",
                offset: 0x0c,
                size: Word,
                fields: [
                    { name: "VC Arbitration Select", lsb: 0, bits: 3 },
                ]
            },
            {
                name: "Port VC Status",
                offset: 0x0e,
                size: Word,
                fields: [
                    { name: "VC Arbitration Table Status", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
