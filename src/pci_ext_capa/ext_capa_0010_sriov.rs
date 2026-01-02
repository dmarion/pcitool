use crate::capabilities;

capabilities! {
    {
        id: 0x0010,
        version: 1,
        is_extended: true,
        name: "Single Root I/O Virtualization",
        size: 64,
        registers: [
            {
                name: "Capabilities",
                offset: 0x04,
                id: CAPS,
                size: Dword,
                fields: [
                    { name: "VF migration capable", lsb: 0, bits: 1 },
                    { name: "ARI capable hierarchy preserved", lsb: 1, bits: 1 },
                    { name: "VF 10-bit Tag Requester Supported", lsb: 2, bits: 1 },
                    { name: "VF Migration Interrupt Message Number", lsb: 21, bits: 11 },
                ]
            },
            {
                name: "Control",
                offset: 0x08,
                id: CTRL,
                size: Word,
                fields: [
                    { name: "Enable VFs", lsb: 0, bits: 1 },
                    { name: "VF Migration Enable", lsb: 1, bits: 1 },
                    { name: "VF Migration Interrupt Enable", lsb: 2, bits: 1 },
                    { name: "VF MSE", lsb: 3, bits: 1 },
                    { name: "ARI Capable Hierarchy", lsb: 4, bits: 1 },
                    { name: "VF 10-bit Tag Requester Enable", lsb: 5, bits: 1 },
                ]
            },
            {
                name: "Status",
                offset: 0x0a,
                id: STAT,
                size: Word,
                fields: [
                    { name: "VF Migration Status", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "Number of VFs initially supported",
                offset: 0x0c,
                id: NUMBER_OF_VFS_INITIALLY_SUPPORTED,
                size: Word,
                fields: []
            },
            {
                name: "Maximum VFs supported",
                offset: 0x0e,
                id: MAXIMUM_VFS_SUPPORTED,
                size: Word,
                fields: []
            },
            {
                name: "Number of VFs that can be enabled",
                offset: 0x10,
                id: NUMBER_OF_VFS_THAT_CAN_BE_ENABLED,
                size: Word,
                fields: []
            },
            {
                name: "Function Dependency Link",
                offset: 0x12,
                id: FUNCTION_DEPENDENCY_LINK,
                size: Word,
                fields: []
            },
            {
                name: "Routing ID offset to the first VF",
                offset: 0x14,
                id: ROUTING_ID_OFF_TO_THE_FIRST_VF,
                size: Word,
                fields: []
            },
            {
                name: "Routing ID offset from one VF to the next one",
                offset: 0x16,
                id: ROUTING_ID_OFF_FROM_ONE_VF_TO_THE_NEXT_ONE,
                size: Word,
                fields: []
            },
            {
                name: "Device ID assigned to VFs",
                offset: 0x1a,
                id: DEVICE_ID_ASSIGNED_TO_VFS,
                size: Word,
                fields: []
            },
            {
                name: "Page size bitmap (supported)",
                offset: 0x1c,
                id: PAGE_SIZE_BITMAP_SUPPORTED,
                size: Dword,
                fields: []
            },
            {
                name: "Page size bitmap (system)",
                offset: 0x20,
                id: PAGE_SIZE_BITMAP_SYSTEM,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR0",
                offset: 0x24,
                id: VF_BAR0,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR1",
                offset: 0x28,
                id: VF_BAR1,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR2",
                offset: 0x2c,
                id: VF_BAR2,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR3",
                offset: 0x30,
                id: VF_BAR3,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR4",
                offset: 0x34,
                id: VF_BAR4,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR5",
                offset: 0x38,
                id: VF_BAR5,
                size: Dword,
                fields: []
            },
            {
                name: "VF Migration State Array Offset",
                offset: 0x3c,
                id: VF_MIGRATION_STATE_ARRAY_OFF,
                size: Dword,
                fields: []
            },
        ]
    }
}
