use crate::capabilities;

capabilities! {
    std {
        id: 0x0f,
        name: "Secure Device",
        registers: [
            {
                name: "Secure Capabilities",
                offset: 0x02,
                size: Word,
                fields: []
            }
        ]
    }
}
