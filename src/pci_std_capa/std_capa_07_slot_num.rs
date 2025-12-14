use crate::capabilities;

capabilities! {
    std {
        id: 0x07,
        name: "Slot Numbering",
        registers: [
            {
                name: "Slot Numbering",
                offset: 0x02,
                size: Byte,
                fields: []
            },
        ]
    }
}
