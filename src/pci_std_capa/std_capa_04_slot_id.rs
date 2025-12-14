use crate::capabilities;

capabilities! {
    std {
        id: 0x04,
        name: "Slot Identification",
        registers: [
            {
                name: "Slot Identifier",
                offset: 0x02,
                size: Byte,
                fields: []
            },
            {
                name: "Chassis ID",
                offset: 0x03,
                size: Byte,
                fields: []
            },
        ]
    }
}
