use crate::capabilities;

const RECEIVER_NUMBERS: &[(u64, &str)] = &[
    (0x0, "Broadcast"),
    (0x1, "Rx(A) - Downstream Port"),
    (0x2, "Rx(B) - Retimer 1 Up"),
    (0x3, "Rx(C) - Retimer 1 Down"),
    (0x4, "Rx(D) - Retimer 2 Up"),
    (0x5, "Rx(E) - Retimer 2 Down"),
    (0x6, "Rx(F) - Upstream Port"),
];

const MARGIN_TYPES: &[(u64, &str)] = &[
    (0x0, "Timing Margin"),
    (0x1, "Voltage Margin"),
    (0x2, "Step Margin"),
    (0x3, "Report Capabilities"),
    (0x4, "Set Margin Offset"),
    (0x5, "Set Sample Count"),
    (0x6, "Go to Normal Settings"),
    (0x7, "No Command"),
];

const USAGE_MODELS: &[(u64, &str)] = &[
    (0x0, "Lane Margining at Receiver"),
    (0x1, "Sideband communication"),
];

capabilities! {
    {
        id: 0x0027,
        version: 1,
        is_extended: true,
        name: "Lane Margining at Receiver",
        size: 72,
        registers: [
            {
                name: "Port Capabilities",
                offset: 0x04,
                id: PORT_CAPS,
                size: Word,
                fields: [
                    { name: "Margining uses Driver Software", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "Port Status",
                offset: 0x06,
                id: PORT_STAT,
                size: Word,
                fields: [
                    { name: "Margining Ready", lsb: 0, bits: 1 },
                    { name: "Margining Software Ready", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "Lane 0 Control",
                offset: 0x08,
                id: LANE_0_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 0 Status",
                offset: 0x0a,
                id: LANE_0_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 1 Control",
                offset: 0x0c,
                id: LANE_1_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 1 Status",
                offset: 0x0e,
                id: LANE_1_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 2 Control",
                offset: 0x10,
                id: LANE_2_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 2 Status",
                offset: 0x12,
                id: LANE_2_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 3 Control",
                offset: 0x14,
                id: LANE_3_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 3 Status",
                offset: 0x16,
                id: LANE_3_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 4 Control",
                offset: 0x18,
                id: LANE_4_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 4 Status",
                offset: 0x1a,
                id: LANE_4_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 5 Control",
                offset: 0x1c,
                id: LANE_5_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 5 Status",
                offset: 0x1e,
                id: LANE_5_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 6 Control",
                offset: 0x20,
                id: LANE_6_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 6 Status",
                offset: 0x22,
                id: LANE_6_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 7 Control",
                offset: 0x24,
                id: LANE_7_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 7 Status",
                offset: 0x26,
                id: LANE_7_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 8 Control",
                offset: 0x28,
                id: LANE_8_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 8 Status",
                offset: 0x2a,
                id: LANE_8_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 9 Control",
                offset: 0x2c,
                id: LANE_9_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 9 Status",
                offset: 0x2e,
                id: LANE_9_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 10 Control",
                offset: 0x30,
                id: LANE_10_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 10 Status",
                offset: 0x32,
                id: LANE_10_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 11 Control",
                offset: 0x34,
                id: LANE_11_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 11 Status",
                offset: 0x36,
                id: LANE_11_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 12 Control",
                offset: 0x38,
                id: LANE_12_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 12 Status",
                offset: 0x3a,
                id: LANE_12_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 13 Control",
                offset: 0x3c,
                id: LANE_13_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 13 Status",
                offset: 0x3e,
                id: LANE_13_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 14 Control",
                offset: 0x40,
                id: LANE_14_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 14 Status",
                offset: 0x42,
                id: LANE_14_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 15 Control",
                offset: 0x44,
                id: LANE_15_CTRL,
                size: Word,
                fields: [
                    { name: "Receiver Number", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Lane 15 Status",
                offset: 0x46,
                id: LANE_15_STAT,
                size: Word,
                fields: [
                    { name: "Receiver Number Status", lsb: 0, bits: 3, enum_values: RECEIVER_NUMBERS },
                    { name: "Margin Type Status", lsb: 3, bits: 3, enum_values: MARGIN_TYPES },
                    { name: "Usage Model Status", lsb: 6, bits: 1, enum_values: USAGE_MODELS },
                    { name: "Margin Payload Status", lsb: 8, bits: 8 },
                ]
            },
        ]
    }
}
