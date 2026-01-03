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

capabilities! {
    {
        id: 0x002A,
        version: 1,
        is_extended: true,
        name: "Physical Layer 32.0 GT/s",
        get_size: get_size,
        summary: summary,
        registers: [
            {
                name: "Capability",
                offset: 0x04,
                id: CAP,
                size: Dword,
                fields: [
                    { name: "Equalization bypass to highest rate supported", lsb: 0, bits: 1 },
                    { name: "No Equalization Needed supported", lsb: 1, bits: 1 },
                    { name: "Modified TS Usage Mode 0 supported", lsb: 8, bits: 1 },
                    { name: "Modified TS Usage Mode 1 supported", lsb: 9, bits: 1 },
                    { name: "Modified TS Usage Mode 2 supported", lsb: 10, bits: 1 },
                ]
            },
            {
                name: "Control",
                offset: 0x08,
                id: CTRL,
                size: Dword,
                fields: [
                    { name: "Equalization bypass to highest rate disable", lsb: 0, bits: 1 },
                    { name: "No Equalization Needed disable", lsb: 1, bits: 1 },
                    { name: "Modified TS Usage Mode selected", lsb: 8, bits: 3 },
                ]
            },
            {
                name: "Status",
                offset: 0x0c,
                id: STAT,
                size: Dword,
                fields: [
                    { name: "Equalization 32 GT/s complete", lsb: 0, bits: 1 },
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
                name: "RX ModTS1",
                offset: 0x10,
                id: RX_MODTS1,
                size: Dword,
                fields: [
                    { name: "Usage Mode", lsb: 0, bits: 3 },
                    { name: "Information 1", lsb: 3, bits: 13 },
                    { name: "Vendor ID", lsb: 16, bits: 16 },
                ]
            },
            {
                name: "RX ModTS2",
                offset: 0x14,
                id: RX_MODTS2,
                size: Dword,
                fields: [
                    { name: "Information 2", lsb: 0, bits: 24 },
                    { name: "Alt Protocol Neg Status", lsb: 24, bits: 2 },
                ]
            },
            {
                name: "TX ModTS1",
                offset: 0x18,
                id: TX_MODTS1,
                size: Dword,
                fields: [
                    { name: "Usage Mode", lsb: 0, bits: 3 },
                    { name: "Information 1", lsb: 3, bits: 13 },
                    { name: "Vendor ID", lsb: 16, bits: 16 },
                ]
            },
            {
                name: "TX ModTS2",
                offset: 0x1c,
                id: TX_MODTS2,
                size: Dword,
                fields: [
                    { name: "Information 2", lsb: 0, bits: 24 },
                    { name: "Alt Protocol Neg Status", lsb: 24, bits: 2 },
                ]
            },
            {
                name: "Lane 0 Equalization Control",
                offset: 0x20,
                id: LANE_0_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 1 Equalization Control",
                offset: 0x21,
                id: LANE_1_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 2 Equalization Control",
                offset: 0x22,
                id: LANE_2_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 3 Equalization Control",
                offset: 0x23,
                id: LANE_3_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 4 Equalization Control",
                offset: 0x24,
                id: LANE_4_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 5 Equalization Control",
                offset: 0x25,
                id: LANE_5_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 6 Equalization Control",
                offset: 0x26,
                id: LANE_6_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 7 Equalization Control",
                offset: 0x27,
                id: LANE_7_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 8 Equalization Control",
                offset: 0x28,
                id: LANE_8_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 9 Equalization Control",
                offset: 0x29,
                id: LANE_9_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 10 Equalization Control",
                offset: 0x2a,
                id: LANE_10_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 11 Equalization Control",
                offset: 0x2b,
                id: LANE_11_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 12 Equalization Control",
                offset: 0x2c,
                id: LANE_12_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 13 Equalization Control",
                offset: 0x2d,
                id: LANE_13_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 14 Equalization Control",
                offset: 0x2e,
                id: LANE_14_EQ_CTRL,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 15 Equalization Control",
                offset: 0x2f,
                id: LANE_15_EQ_CTRL,
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
    let lanes = cap.max_link_width()?.min(32);
    Some(0x20 + lanes)
}

fn summary(cap: &PciCapa) -> Option<Vec<TreeNode>> {
    let status = cap.read_u32(u64::from(STAT)).ok()?;
    let eq_complete = status & 0x1 != 0;
    let phase1 = status & 0x2 != 0;
    let phase2 = status & 0x4 != 0;
    let phase3 = status & 0x8 != 0;

    let mut spans = Vec::new();
    let fields = [
        ("Equalization Complete", eq_complete),
        ("Phase1", phase1),
        ("Phase2", phase2),
        ("Phase3", phase3),
    ];

    for (idx, (label, val)) in fields.iter().enumerate() {
        if idx > 0 {
            spans.push(TreeSpan::raw(" "));
        }
        let color = if *val {
            TreeColor::Green
        } else {
            TreeColor::Red
        };
        if let Some(rest) = label.strip_prefix("Equalization ") {
            spans.push(TreeSpan::raw("Equalization "));
            let text = format!("{}{}", rest, if *val { "+" } else { "-" });
            spans.push(TreeSpan::styled(text, color));
        } else {
            let text = format!("{}{}", label, if *val { "+" } else { "-" });
            spans.push(TreeSpan::styled(text, color));
        }
    }

    Some(vec![TreeNode::with_value(
        TreeLine::from("Physical Layer 32.0 GT/s"),
        TreeLine::from(spans),
    )])
}
