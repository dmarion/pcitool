use crate::capabilities;

capabilities! {
    std {
        id: 0x03,
        name: "Vital Product Data",
        registers: [
            {
                name: "VPD Address",
                offset: 0x02,
                size: Word,
                fields: [
                    { name: "VPD Address", lsb: 0, bits: 15 },
                    {
                        name: "F",
                        lsb: 15,
                        bits: 1,
                        enum_values: [
                            (0x0, "Read"),
                            (0x1, "Write/Done"),
                        ]
                    },
                ]
            },
            {
                name: "VPD Data",
                offset: 0x04,
                size: Dword,
                fields: []
            }
        ]
    }
}
