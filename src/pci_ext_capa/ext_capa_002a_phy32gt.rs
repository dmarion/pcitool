use crate::capabilities;
use crate::pci_capa::{RegisterSize, read_raw};
use crate::tree::PciNode;
use ratatui::prelude::{Color, Span, Style};
use ratatui::text::Line;

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
        id: 0x002A,
        version: 1,
        name: "Physical Layer 32.0 GT/s",
        summary: summary,
        registers: [
            {
                name: "Capability",
                offset: 0x04,
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
                size: Dword,
                fields: [
                    { name: "Information 2", lsb: 0, bits: 24 },
                    { name: "Alt Protocol Neg Status", lsb: 24, bits: 2 },
                ]
            },
            {
                name: "TX ModTS1",
                offset: 0x18,
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
                size: Dword,
                fields: [
                    { name: "Information 2", lsb: 0, bits: 24 },
                    { name: "Alt Protocol Neg Status", lsb: 24, bits: 2 },
                ]
            },
            {
                name: "Lane 0 Equalization Control",
                offset: 0x20,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 1 Equalization Control",
                offset: 0x21,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 2 Equalization Control",
                offset: 0x22,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 3 Equalization Control",
                offset: 0x23,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 4 Equalization Control",
                offset: 0x24,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 5 Equalization Control",
                offset: 0x25,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 6 Equalization Control",
                offset: 0x26,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 7 Equalization Control",
                offset: 0x27,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 8 Equalization Control",
                offset: 0x28,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 9 Equalization Control",
                offset: 0x29,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 10 Equalization Control",
                offset: 0x2a,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 11 Equalization Control",
                offset: 0x2b,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 12 Equalization Control",
                offset: 0x2c,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 13 Equalization Control",
                offset: 0x2d,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 14 Equalization Control",
                offset: 0x2e,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
            {
                name: "Lane 15 Equalization Control",
                offset: 0x2f,
                size: Byte,
                fields: [
                    { name: "Downstream Transmitter Preset", lsb: 0, bits: 4, enum_values: TX_PRESETS },
                    { name: "Upstream Transmitter Preset", lsb: 4, bits: 4, enum_values: TX_PRESETS },
                ]
            },
        ]
    }
}

fn summary(_offset: u16, _version: u8, bytes: &[u8]) -> Option<Vec<PciNode>> {
    let status = read_raw(bytes, 0x0c, RegisterSize::Dword)?;
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
            spans.push(Span::raw(" "));
        }
        spans.push(Span::raw(*label));
        if *val {
            spans.push(Span::styled("+", Style::default().fg(Color::LightGreen)));
        } else {
            spans.push(Span::styled("-", Style::default().fg(Color::LightRed)));
        }
    }

    Some(vec![PciNode::with_value(
        Line::from("Physical Layer 32.0 GT/s"),
        Line::from(spans),
    )])
}
