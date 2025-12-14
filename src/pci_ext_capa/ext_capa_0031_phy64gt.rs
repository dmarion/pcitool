use crate::capabilities;

const TX_PRESETS: &[(u64, &str)] = &[
    (0x0, "P0"),
    (0x1, "P1"),
    (0x2, "P2"),
    (0x3, "P3"),
    (0x4, "P4"),
    (0x5, "P5"),
    (0x6, "P6"),
    (0x7, "P7"),
    (0x8, "P8"),
    (0x9, "P9"),
    (0xa, "P10"),
];

capabilities! {
    ext {
        id: 0x0031,
        version: 1,
        name: "Physical Layer 64.0 GT/s",
        registers: [
            {
                name: "Capability",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "No Equalization Needed supported", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "Control",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "No Equalization Needed disable", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "Status",
                offset: 0x0c,
                size: Dword,
                fields: [
                    { name: "Equalization 64 GT/s complete", lsb: 0, bits: 1 },
                    { name: "Equalization Phase 1 successful", lsb: 1, bits: 1 },
                    { name: "Equalization Phase 2 successful", lsb: 2, bits: 1 },
                    { name: "Equalization Phase 3 successful", lsb: 3, bits: 1 },
                    { name: "Link Equalization Request", lsb: 4, bits: 1 },
                    { name: "Modified TS received", lsb: 5, bits: 1 },
                    { name: "Received Enhanced Link Behavior Control", lsb: 6, bits: 2 },
                    { name: "Transmitter Precoding On", lsb: 8, bits: 1 },
                    { name: "Transmitter Precoding Request", lsb: 9, bits: 1 },
                    { name: "No Equalization Needed Received", lsb: 10, bits: 1 },
                ]
            },
            {
                name: "Lane 0 Equalization Control",
                offset: 0x10,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 1 Equalization Control",
                offset: 0x11,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 2 Equalization Control",
                offset: 0x12,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 3 Equalization Control",
                offset: 0x13,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 4 Equalization Control",
                offset: 0x14,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 5 Equalization Control",
                offset: 0x15,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 6 Equalization Control",
                offset: 0x16,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 7 Equalization Control",
                offset: 0x17,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
        ]
    }
}
