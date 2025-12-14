use crate::capabilities;

capabilities! {
    std {
        id: 0x12,
        name: "SATA Data Index Configuration",
        registers: [
            {
                name: "SATA Capability",
                offset: 0x02,
                size: Word,
                fields: [
                    { name: "Revision", lsb: 0, bits: 4 },
                    {
                        name: "BAR Offset",
                        lsb: 4,
                        bits: 4,
                        enum_values: [
                            (0x0, "00h"),
                            (0x1, "04h"),
                            (0x2, "08h"),
                            (0x3, "0Ch"),
                            (0x4, "10h"),
                            (0x5, "14h"),
                            (0x6, "18h"),
                            (0x7, "1Ch"),
                            (0x8, "20h"),
                            (0x9, "24h"),
                            (0xa, "Reserved"),
                            (0xf, "Index/Data pairs in config space"),
                        ]
                    },
                    {
                        name: "BAR Location",
                        lsb: 8,
                        bits: 4,
                        enum_values: [
                            (0x0, "BAR0"),
                            (0x1, "BAR1"),
                            (0x2, "BAR2"),
                            (0x3, "BAR3"),
                            (0x4, "BAR4"),
                            (0x5, "BAR5"),
                            (0xf, "Absolute address"),
                        ]
                    },
                ]
            },
        ]
    }
}
