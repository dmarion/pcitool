use crate::capabilities;

capabilities! {
    std {
        id: 0x14,
        name: "Enhanced Allocation",
        registers: [
            {
                name: "EA Capabilities",
                offset: 0x02,
                size: Word,
                fields: [
                    { name: "Number of Entries", lsb: 0, bits: 6 },
                ]
            },
        ]
    }
}
