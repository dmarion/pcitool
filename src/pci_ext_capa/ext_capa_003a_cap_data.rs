use crate::capabilities;

capabilities! {
    {
        id: 0x003a,
        version: 1,
        is_extended: true,
        name: "Captured Data",
        size: 16,
        registers: [
            {
                name: "Captured Data Capabilities",
                offset: 0x04,
                id: CAPTURED_DATA_CAPS,
                size: Word,
                fields: []
            },
            {
                name: "Captured Data Control",
                offset: 0x06,
                id: CAPTURED_DATA_CTRL,
                size: Word,
                fields: []
            },
            {
                name: "Captured Data Register",
                offset: 0x08,
                id: CAPTURED_DATA_REG,
                size: Dword,
                fields: []
            },
        ]
    }
}
