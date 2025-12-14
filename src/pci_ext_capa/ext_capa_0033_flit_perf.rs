use crate::capabilities;

capabilities! {
    ext {
        id: 0x0033,
        version: 1,
        name: "Flit Performance Measurement",
        registers: [
            {
                name: "Flit Performance Measurement Capability",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Performance Measurement Control",
                offset: 0x08,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Performance Measurement Status",
                offset: 0x0c,
                size: Dword,
                fields: []
            },
        ]
    }
}
