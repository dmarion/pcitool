use crate::capabilities;

capabilities! {
    {
        id: 0x07,
        name: "Slot Numbering",
        size: 8,
        registers: [
            {
                name: "Slot Numbering",
                offset: 0x02,
                id: SLOT_NUMBERING,
                size: Byte,
                fields: []
            },
        ]
    }
}
