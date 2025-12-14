use crate::capabilities;

capabilities! {
    ext {
        id: 0x0016,
        version: 1,
        name: "Dynamic Power Allocation",
        registers: [
            {
                name: "DPA Capability",
                offset: 0x04,
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
                size: Dword,
                fields: []
            },
            {
                name: "DPA Status",
                offset: 0x0c,
                size: Word,
                fields: [
                    { name: "Substate_Status", lsb: 0, bits: 5 },
                ]
            },
            {
                name: "DPA Control",
                offset: 0x0e,
                size: Word,
                fields: [
                    { name: "Substate_Control", lsb: 0, bits: 5 },
                    { name: "DPA_Enable", lsb: 8, bits: 1 },
                ]
            },
        ]
    }
}
