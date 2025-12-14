use crate::capabilities;

capabilities! {
    ext {
        id: 0x0036,
        version: 1,
        name: "MMIO Register Block Locator",
        registers: [
            {
                name: "MRBL Capabilities",
                offset: 0x04,
                size: Dword,
                fields: []
            },
        ]
    }
}
