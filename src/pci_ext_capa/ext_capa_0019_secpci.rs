use crate::capabilities;
use crate::pci_device::PciCapa;
use crate::tree::{TreeColor, TreeLine, TreeNode, TreeSpan};

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

const RX_HINTS: &[(u64, &str)] = &[
    (0x0, "-11 dB"),
    (0x1, "-10 dB"),
    (0x2, "-9 dB"),
    (0x3, "-8 dB"),
    (0x4, "-7 dB"),
    (0x5, "-6 dB"),
    (0x6, "-5 dB"),
    (0x7, "-4 dB"),
];

capabilities! {
    {
        id: 0x0019,
        version: 1,
        is_extended: true,
        name: "Secondary PCI Express",
        get_size: get_size,
        summary: summary,
        registers: [
            {
                name: "Link Control 3",
                offset: 0x04,
                id: LINK_CTRL3,
                size: Word,
                fields: [
                    { name: "Perform Link Equalization", lsb: 0, bits: 1 },
                    { name: "Link Equalization Request Interrupt Enable", lsb: 1, bits: 1 },
                    {
                        name: "Enable Lower SKP OS Generation Vector",
                        lsb: 9,
                        bits: 7,
                        enum_values: [
                            (0x01, "2.5 GT/s"),
                            (0x02, "5.0 GT/s"),
                            (0x04, "8.0 GT/s"),
                            (0x08, "16.0 GT/s"),
                            (0x10, "32.0 GT/s"),
                            (0x20, "64.0 GT/s"),
                            (0x40, "128.0 GT/s"),
                        ]
                    },
                ]
            },
            {
                name: "Lane Error Status",
                offset: 0x08,
                id: LANE_ERROR_STAT,
                size: Dword,
                fields: []
            },
            {
                name: "Lane 0 Equalization Control",
                offset: 0x0c,
                id: LANE_0_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 1 Equalization Control",
                offset: 0x0e,
                id: LANE_1_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 2 Equalization Control",
                offset: 0x10,
                id: LANE_2_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 3 Equalization Control",
                offset: 0x12,
                id: LANE_3_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 4 Equalization Control",
                offset: 0x14,
                id: LANE_4_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 5 Equalization Control",
                offset: 0x16,
                id: LANE_5_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 6 Equalization Control",
                offset: 0x18,
                id: LANE_6_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 7 Equalization Control",
                offset: 0x1a,
                id: LANE_7_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 8 Equalization Control",
                offset: 0x1c,
                id: LANE_8_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 9 Equalization Control",
                offset: 0x1e,
                id: LANE_9_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 10 Equalization Control",
                offset: 0x20,
                id: LANE_10_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 11 Equalization Control",
                offset: 0x22,
                id: LANE_11_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 12 Equalization Control",
                offset: 0x24,
                id: LANE_12_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 13 Equalization Control",
                offset: 0x26,
                id: LANE_13_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 14 Equalization Control",
                offset: 0x28,
                id: LANE_14_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
            {
                name: "Lane 15 Equalization Control",
                offset: 0x2a,
                id: LANE_15_EQ_CTRL,
                size: Word,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Downstream Receiver Preset Hint", lsb: 4, bits: 3, enum_values: RX_HINTS },
                    { name: "Upstream Transmitter Preset", lsb: 8, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Receiver Preset Hint", lsb: 12, bits: 3, enum_values: RX_HINTS },
                ]
            },
        ]
    }
}

fn get_size(cap: &PciCapa) -> Option<u16> {
    let max_width = cap.max_link_width()?;
    Some(0x0c + max_width * 2)
}

fn summary(cap: &PciCapa) -> Option<Vec<TreeNode>> {
    let status = cap.read_u32(u64::from(LANE_ERROR_STAT)).unwrap_or(0);
    let node = if status == 0 {
        TreeNode::with_value(
            TreeLine::from("Lane Errors"),
            TreeLine::from(vec![TreeSpan::styled("None", TreeColor::Green)]),
        )
    } else {
        let mut lanes = Vec::new();
        for i in 0..32 {
            if (status & (1 << i)) != 0 {
                lanes.push(i.to_string());
            }
        }
        TreeNode::with_value(
            TreeLine::from("Lane Errors"),
            TreeLine::from(vec![TreeSpan::styled(
                format!("Lane(s) {}", lanes.join(", ")),
                TreeColor::Red,
            )]),
        )
    };
    Some(vec![node])
}
