use crate::capabilities;

capabilities! {
    ext {
        id: 0x000d,
        version: 1,
        name: "Access Controls",
        registers: [
            {
                name: "ACS Capability",
                offset: 0x04,
                size: Word,
                fields: [
                    { name: "Source Validation", lsb: 0, bits: 1 },
                    { name: "Translation Blocking", lsb: 1, bits: 1 },
                    { name: "P2P Request Redirect", lsb: 2, bits: 1 },
                    { name: "P2P Completion Redirect", lsb: 3, bits: 1 },
                    { name: "Upstream Forwarding", lsb: 4, bits: 1 },
                    { name: "P2P Egress Control", lsb: 5, bits: 1 },
                    { name: "Direct Translated P2P", lsb: 6, bits: 1 },
                    { name: "ACS Enhanced Capability", lsb: 7, bits: 1 },
                    { name: "Egress Control Vector Size", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "ACS Control",
                offset: 0x06,
                size: Word,
                fields: [
                    { name: "Source Validation Enable", lsb: 0, bits: 1 },
                    { name: "Translation Blocking Enable", lsb: 1, bits: 1 },
                    { name: "P2P Request Redirect Enable", lsb: 2, bits: 1 },
                    { name: "P2P Completion Redirect Enable", lsb: 3, bits: 1 },
                    { name: "Upstream Forwarding Enable", lsb: 4, bits: 1 },
                    { name: "P2P Egress Control Enable", lsb: 5, bits: 1 },
                    { name: "Direct Translated P2P Enable", lsb: 6, bits: 1 },
                    { name: "ACS I/O Request Blocking Enable", lsb: 7, bits: 1 },
                    { name: "ACS DSP Memory Target Access Control", lsb: 8, bits: 1 },
                    { name: "ACS USP Memory Target Access Control", lsb: 9, bits: 1 },
                    { name: "ACS Unclaimed Request Redirect Control", lsb: 10, bits: 1 },
                ]
            },
            {
                name: "ACS Egress Control Vector",
                offset: 0x08,
                size: Dword,
                fields: []
            },
        ]
    }
}
