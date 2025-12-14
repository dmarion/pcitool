use crate::capabilities;

capabilities! {
    std {
        id: 0x09,
        name: "Vendor Specific",
        registers: [
            {
                name: "Capability Length",
                offset: 0x02,
                size: Byte,
                fields: []
            }
        ]
    }
}
