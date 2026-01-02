use crate::capabilities;
use crate::pci_device::PciCapa;

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
    {
        id: 0x0039,
        version: 1,
        is_extended: true,
        name: "Physical Layer 128.0 GT/s",
        get_size: get_size,
        registers: [
            {
                name: "Capability",
                offset: 0x04,
                id: CAP,
                size: Dword,
                fields: [
                    { name: "No Equalization Needed supported", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "Control",
                offset: 0x08,
                id: CTRL,
                size: Dword,
                fields: [
                    { name: "No Equalization Needed disable", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "Status",
                offset: 0x0c,
                id: STAT,
                size: Dword,
                fields: [
                    { name: "Equalization 128 GT/s complete", lsb: 0, bits: 1 },
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
                id: LANE_0_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 1 Equalization Control",
                offset: 0x11,
                id: LANE_1_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 2 Equalization Control",
                offset: 0x12,
                id: LANE_2_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 3 Equalization Control",
                offset: 0x13,
                id: LANE_3_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 4 Equalization Control",
                offset: 0x14,
                id: LANE_4_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 5 Equalization Control",
                offset: 0x15,
                id: LANE_5_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 6 Equalization Control",
                offset: 0x16,
                id: LANE_6_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 7 Equalization Control",
                offset: 0x17,
                id: LANE_7_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
        ]
    }
}

fn get_size(cap: &PciCapa) -> Option<u16> {
    let lanes = cap.max_link_width()?.min(16);
    Some(0x10 + lanes)
}
