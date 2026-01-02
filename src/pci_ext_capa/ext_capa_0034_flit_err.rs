use crate::capabilities;

capabilities! {
    {
        id: 0x0034,
        version: 1,
        is_extended: true,
        name: "Flit Error Injection",
        size: 20,
        registers: [
            {
                name: "Flit Error Injection Capability",
                offset: 0x04,
                id: FLIT_ERROR_INJECTION_CAP,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Error Injection Control 1",
                offset: 0x08,
                id: FLIT_ERROR_INJECTION_CTRL1,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Error Injection Control 2",
                offset: 0x0c,
                id: FLIT_ERROR_INJECTION_CTRL2,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Error Injection Status",
                offset: 0x10,
                id: FLIT_ERROR_INJECTION_STAT,
                size: Dword,
                fields: []
            },
        ]
    }
}
