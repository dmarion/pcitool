use crate::capabilities;

capabilities! {
    ext {
        id: 0x0010,
        version: 1,
        name: "Single Root I/O Virtualization",
        registers: [
            {
                name: "Capabilities",
                offset: 0x04,
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
                size: Word,
                fields: [
                    { name: "VF Migration Status", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "Number of VFs initially supported",
                offset: 0x0c,
                size: Word,
                fields: []
            },
            {
                name: "Maximum VFs supported",
                offset: 0x0e,
                size: Word,
                fields: []
            },
            {
                name: "Number of VFs that can be enabled",
                offset: 0x10,
                size: Word,
                fields: []
            },
            {
                name: "Function Dependency Link",
                offset: 0x12,
                size: Word,
                fields: []
            },
            {
                name: "Routing ID offset to the first VF",
                offset: 0x14,
                size: Word,
                fields: []
            },
            {
                name: "Routing ID offset from one VF to the next one",
                offset: 0x16,
                size: Word,
                fields: []
            },
            {
                name: "Device ID assigned to VFs",
                offset: 0x1a,
                size: Word,
                fields: []
            },
            {
                name: "Page size bitmap (supported)",
                offset: 0x1c,
                size: Dword,
                fields: []
            },
            {
                name: "Page size bitmap (system)",
                offset: 0x20,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR0",
                offset: 0x24,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR1",
                offset: 0x28,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR2",
                offset: 0x2c,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR3",
                offset: 0x30,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR4",
                offset: 0x34,
                size: Dword,
                fields: []
            },
            {
                name: "VF BAR5",
                offset: 0x38,
                size: Dword,
                fields: []
            },
            {
                name: "VF Migration State Array Offset",
                offset: 0x3c,
                size: Dword,
                fields: []
            },
        ]
    }
}
