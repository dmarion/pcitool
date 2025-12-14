use crate::capabilities;

capabilities! {
    std {
        id: 0x0d,
        name: "Subsystem Vendor ID",
        registers: [
            {
                name: "Subsystem Vendor ID",
                offset: 0x04,
                size: Word,
                fields: []
            },
            {
                name: "Subsystem ID",
                offset: 0x06,
                size: Word,
                fields: []
            },
        ]
    }
}
