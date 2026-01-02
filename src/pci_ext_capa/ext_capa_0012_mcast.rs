use crate::capabilities;

capabilities! {
    {
        id: 0x0012,
        version: 1,
        is_extended: true,
        name: "Multicast",
        size: 48,
        registers: [
            {
                name: "Multicast Capability",
                offset: 0x04,
                id: MULTICAST_CAP,
                size: Word,
                fields: [
                    { name: "Max_Groups", lsb: 0, bits: 6 },
                    { name: "Window_Size_Exponent", lsb: 8, bits: 6 },
                    { name: "ECRC Regeneration Supported", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Multicast Control",
                offset: 0x06,
                id: MULTICAST_CTRL,
                size: Word,
                fields: [
                    { name: "MC_Enable", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "MC_Base_Address",
                offset: 0x08,
                id: MC_BASE_ADDR,
                size: Qword,
                fields: [
                    { name: "Index_Position", lsb: 0, bits: 6 },
                    { name: "MC_Base_Address", lsb: 12, bits: 52 },
                ]
            },
            {
                name: "MC_Receive",
                offset: 0x10,
                id: MC_RECEIVE,
                size: Qword,
                fields: []
            },
            {
                name: "MC_Block_All",
                offset: 0x18,
                id: MC_BLOCK_ALL,
                size: Qword,
                fields: []
            },
            {
                name: "MC_Block_Untranslated",
                offset: 0x20,
                id: MC_BLOCK_UNTRANSLATED,
                size: Qword,
                fields: []
            },
            {
                name: "MC_Overlay_BAR",
                offset: 0x28,
                id: MC_OVERLAY_BAR,
                size: Qword,
                fields: [
                    { name: "MC_Overlay_Size_Exponent", lsb: 0, bits: 6 },
                    { name: "MC_Overlay_BAR", lsb: 12, bits: 52 },
                ]
            },
        ]
    }
}
