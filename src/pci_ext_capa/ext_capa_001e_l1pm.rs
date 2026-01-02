use crate::capabilities;

capabilities! {
    {
        id: 0x001e,
        version: 1,
        is_extended: true,
        name: "L1 PM Substates",
        size: 20,
        registers: [
            {
                name: "L1 PM Substate Capability",
                offset: 0x04,
                id: L1_PM_SUBSTATE_CAP,
                size: Dword,
                fields: [
                    { name: "PCI-PM L1.2 Supported", lsb: 0, bits: 1 },
                    { name: "PCI-PM L1.1 Supported", lsb: 1, bits: 1 },
                    { name: "ASPM L1.2 Supported", lsb: 2, bits: 1 },
                    { name: "ASPM L1.1 Supported", lsb: 3, bits: 1 },
                    { name: "L1 PM Substates supported", lsb: 4, bits: 1 },
                    { name: "Link Activation Supported", lsb: 5, bits: 1 },
                    { name: "Port Common_Mode_Restore_Time", lsb: 8, bits: 8 },
                    {
                        name: "Port T_POWER_ON Scale",
                        lsb: 16,
                        bits: 2,
                        enum_values: [
                            (0x0, "2us"),
                            (0x1, "10us"),
                            (0x2, "100us"),
                        ]
                    },
                    { name: "Port T_POWER_ON Value", lsb: 19, bits: 5 },
                ]
            },
            {
                name: "L1 PM Substate Control 1",
                offset: 0x08,
                id: L1_PM_SUBSTATE_CTRL1,
                size: Dword,
                fields: [
                    { name: "PCI-PM L1.2 Enable", lsb: 0, bits: 1 },
                    { name: "PCI-PM L1.1 Enable", lsb: 1, bits: 1 },
                    { name: "ASPM L1.2 Enable", lsb: 2, bits: 1 },
                    { name: "ASPM L1.1 Enable", lsb: 3, bits: 1 },
                    { name: "Link Activation Interrupt Enable", lsb: 4, bits: 1 },
                    { name: "Link Activation Control", lsb: 5, bits: 1 },
                    { name: "Common_Mode_Restore_Time", lsb: 8, bits: 8 },
                    { name: "LTR_L1.2_THRESHOLD_Value", lsb: 16, bits: 10 },
                    {
                        name: "LTR_L1.2_THRESHOLD_Scale",
                        lsb: 29,
                        bits: 3,
                        enum_values: [
                            (0x0, "1ns"),
                            (0x1, "32ns"),
                            (0x2, "1024ns"),
                            (0x3, "32768ns"),
                            (0x4, "1048576ns"),
                            (0x5, "33554432ns"),
                        ]
                    },
                ]
            },
            {
                name: "L1 PM Substate Control 2",
                offset: 0x0c,
                id: L1_PM_SUBSTATE_CTRL2,
                size: Dword,
                fields: [
                    {
                        name: "T_POWER_ON Scale",
                        lsb: 0,
                        bits: 2,
                        enum_values: [
                            (0x0, "2us"),
                            (0x1, "10us"),
                            (0x2, "100us"),
                        ]
                    },
                    { name: "T_POWER_ON Value", lsb: 3, bits: 5 },
                ]
            },
            {
                name: "L1 PM Substate Status",
                offset: 0x10,
                id: L1_PM_SUBSTATE_STAT,
                size: Dword,
                fields: [
                    { name: "Link Activation Status", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
