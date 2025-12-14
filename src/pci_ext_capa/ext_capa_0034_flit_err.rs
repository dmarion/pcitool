use crate::capabilities;

capabilities! {
    ext {
        id: 0x0034,
        version: 1,
        name: "Flit Error Injection",
        registers: [
            {
                name: "Flit Error Injection Capability",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Error Injection Control 1",
                offset: 0x08,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Error Injection Control 2",
                offset: 0x0c,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Error Injection Status",
                offset: 0x10,
                size: Dword,
                fields: []
            },
        ]
    }
}
