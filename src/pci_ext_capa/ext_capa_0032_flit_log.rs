use crate::capabilities;

capabilities! {
    {
        id: 0x0032,
        version: 1,
        is_extended: true,
        name: "Flit Logging",
        size: 16,
        registers: [
            {
                name: "Flit Logging Capability",
                offset: 0x04,
                id: FLIT_LOGGING_CAP,
                size: Dword,
                fields: [
                    { name: "Flit Log FIFO Depth", lsb: 0, bits: 8 },
                ]
            },
            {
                name: "Flit Logging Status",
                offset: 0x08,
                id: FLIT_LOGGING_STAT,
                size: Dword,
                fields: [
                    { name: "Flit Log Valid", lsb: 0, bits: 1 },
                    { name: "Flit Log Overflow", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "Flit Logging Control",
                offset: 0x0c,
                id: FLIT_LOGGING_CTRL,
                size: Dword,
                fields: [
                    { name: "Flit Log Enable", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
