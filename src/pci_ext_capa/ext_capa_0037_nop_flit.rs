use crate::capabilities;

capabilities! {
    ext {
        id: 0x0037,
        version: 1,
        name: "NOP Flit",
        registers: [
            {
                name: "NOP Flit Capabilities",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "NOP Flit Control 1",
                offset: 0x08,
                size: Dword,
                fields: []
            },
            {
                name: "NOP Flit Control 2",
                offset: 0x0c,
                size: Dword,
                fields: []
            },
            {
                name: "NOP Flit Status",
                offset: 0x10,
                size: Dword,
                fields: []
            },
        ]
    }
}
