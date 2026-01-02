use crate::capabilities;

capabilities! {
    {
        id: 0x0037,
        version: 1,
        is_extended: true,
        name: "NOP Flit",
        size: 20,
        registers: [
            {
                name: "NOP Flit Capabilities",
                offset: 0x04,
                id: NOP_FLIT_CAPS,
                size: Dword,
                fields: []
            },
            {
                name: "NOP Flit Control 1",
                offset: 0x08,
                id: NOP_FLIT_CTRL1,
                size: Dword,
                fields: []
            },
            {
                name: "NOP Flit Control 2",
                offset: 0x0c,
                id: NOP_FLIT_CTRL2,
                size: Dword,
                fields: []
            },
            {
                name: "NOP Flit Status",
                offset: 0x10,
                id: NOP_FLIT_STAT,
                size: Dword,
                fields: []
            },
        ]
    }
}
