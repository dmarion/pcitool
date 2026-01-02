use crate::capabilities;

capabilities! {
    {
        id: 0x15,
        name: "Flattening Portal Bridge",
        size: 8,
        registers: [
            {
                name: "Control",
                offset: 0x02,
                id: CTRL,
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
                id: STAT,
                size: Word,
                fields: [
                    { name: "Portal Ready", lsb: 0, bits: 1 },
                    { name: "Portal Error", lsb: 1, bits: 1 },
                ]
            },
        ]
    }
}
