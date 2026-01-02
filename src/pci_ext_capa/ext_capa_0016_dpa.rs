use crate::capabilities;

capabilities! {
    {
        id: 0x0016,
        version: 1,
        is_extended: true,
        name: "Dynamic Power Allocation",
        size: 16,
        registers: [
            {
                name: "DPA Capability",
                offset: 0x04,
                id: DPA_CAP,
                size: Dword,
                fields: [
                    { name: "Substate_Max", lsb: 0, bits: 5 },
                    { name: "Transition_Latency_Unit", lsb: 8, bits: 2 },
                    { name: "PWR_Allocation_Scale", lsb: 12, bits: 2 },
                ]
            },
            {
                name: "DPA Latency Indicator",
                offset: 0x08,
                id: DPA_LATENCY_INDICATOR,
                size: Dword,
                fields: []
            },
            {
                name: "DPA Status",
                offset: 0x0c,
                id: DPA_STAT,
                size: Word,
                fields: [
                    { name: "Substate_Status", lsb: 0, bits: 5 },
                ]
            },
            {
                name: "DPA Control",
                offset: 0x0e,
                id: DPA_CTRL,
                size: Word,
                fields: [
                    { name: "Substate_Control", lsb: 0, bits: 5 },
                    { name: "DPA_Enable", lsb: 8, bits: 1 },
                ]
            },
        ]
    }
}
