use crate::capabilities;

capabilities! {
    std {
        id: 0x15,
        name: "Flattening Portal Bridge",
        registers: [
            {
                name: "Control",
                offset: 0x02,
                size: Word,
                fields: [
                    { name: "Enable", lsb: 0, bits: 1 },
                    { name: "Multicast Forwarding", lsb: 1, bits: 1 },
                    { name: "Reserved", lsb: 2, bits: 6 },
                ]
            },
            {
                name: "Status",
                offset: 0x04,
                size: Word,
                fields: [
                    { name: "Portal Ready", lsb: 0, bits: 1 },
                    { name: "Portal Error", lsb: 1, bits: 1 },
                ]
            },
        ]
    }
}
