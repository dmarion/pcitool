use crate::capabilities;

capabilities! {
    {
        id: 0x04,
        name: "Slot Identification",
        size: 8,
        registers: [
            {
                name: "Slot Identifier",
                offset: 0x02,
                id: SLOT_IDENTIFIER,
                size: Byte,
                fields: []
            },
            {
                name: "Chassis ID",
                offset: 0x03,
                id: CHASSIS_ID,
                size: Byte,
                fields: []
            },
        ]
    }
}
