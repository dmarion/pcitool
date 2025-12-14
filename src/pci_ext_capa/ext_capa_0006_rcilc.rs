use crate::capabilities;

capabilities! {
    ext {
        id: 0x0006,
        version: 1,
        name: "Root Complex Internal Link Control",
        registers: [
            {
                name: "Root Complex Link Capabilities",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Max Link Speed", lsb: 0, bits: 4 },
                    { name: "Max Link Width", lsb: 4, bits: 6 },
                    { name: "ASPM Support", lsb: 10, bits: 2 },
                    { name: "L0s Exit Latency", lsb: 12, bits: 3 },
                    { name: "L1 Exit Latency", lsb: 15, bits: 3 },
                ]
            },
            {
                name: "Root Complex Link Control",
                offset: 0x08,
                size: Word,
                fields: [
                    { name: "ASPM Control", lsb: 0, bits: 2 },
                    { name: "Read Completion Boundary", lsb: 3, bits: 1 },
                    { name: "Common Clock Configuration", lsb: 6, bits: 1 },
                    { name: "Extended Synch", lsb: 7, bits: 1 },
                ]
            },
            {
                name: "Root Complex Link Status",
                offset: 0x0a,
                size: Word,
                fields: [
                    { name: "Current Link Speed", lsb: 0, bits: 4 },
                    { name: "Negotiated Link Width", lsb: 4, bits: 6 },
                    { name: "Link Training", lsb: 15, bits: 1 },
                ]
            },
        ]
    }
}
