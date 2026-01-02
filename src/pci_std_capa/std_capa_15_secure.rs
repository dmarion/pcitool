use crate::capabilities;

capabilities! {
    {
        id: 0x0f,
        name: "Secure Device",
        size: 8,
        registers: [
            {
                name: "Secure Capabilities",
                offset: 0x02,
                id: SECURE_CAPS,
                size: Word,
                fields: []
            }
        ]
    }
}
