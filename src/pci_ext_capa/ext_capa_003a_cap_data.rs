use crate::capabilities;

capabilities! {
    ext {
        id: 0x003a,
        version: 1,
        name: "Captured Data",
        registers: [
            {
                name: "Captured Data Capabilities",
                offset: 0x04,
                size: Word,
                fields: []
            },
            {
                name: "Captured Data Control",
                offset: 0x06,
                size: Word,
                fields: []
            },
            {
                name: "Captured Data Register",
                offset: 0x08,
                size: Dword,
                fields: []
            },
        ]
    }
}
