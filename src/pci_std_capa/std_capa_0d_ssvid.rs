use crate::capabilities;

capabilities! {
    {
        id: 0x0d,
        name: "Subsystem Vendor ID",
        size: 8,
        registers: [
            {
                name: "Subsystem Vendor ID",
                offset: 0x04,
                id: SUBSYSTEM_VENDOR_ID,
                size: Word,
                fields: []
            },
            {
                name: "Subsystem ID",
                offset: 0x06,
                id: SUBSYSTEM_ID,
                size: Word,
                fields: []
            },
        ]
    }
}
