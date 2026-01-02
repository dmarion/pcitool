use crate::capabilities;

capabilities! {
    {
        id: 0x0033,
        version: 1,
        is_extended: true,
        name: "Flit Performance Measurement",
        size: 16,
        registers: [
            {
                name: "Flit Performance Measurement Capability",
                offset: 0x04,
                id: FLIT_PERFORMANCE_MEASUREMENT_CAP,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Performance Measurement Control",
                offset: 0x08,
                id: FLIT_PERFORMANCE_MEASUREMENT_CTRL,
                size: Dword,
                fields: []
            },
            {
                name: "Flit Performance Measurement Status",
                offset: 0x0c,
                id: FLIT_PERFORMANCE_MEASUREMENT_STAT,
                size: Dword,
                fields: []
            },
        ]
    }
}
