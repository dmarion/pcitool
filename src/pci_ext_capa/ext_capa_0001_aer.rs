use crate::capabilities;
use crate::pci_device::PciCapa;
use crate::tree::{TreeColor, TreeLine, TreeNode, TreeSpan};

capabilities! {
    {
        id: 0x0001,
        version: 3,
        is_extended: true,
        name: "Advanced Error Reporting",
        get_size: get_size_v3,
        summary: summary,
        registers: [
            {
                name: "Uncorrectable Error Status",
                offset: 0x04,
                id: AER_V3_UNCORRECTABLE_ERROR_STAT,
                size: Dword,
                fields: [
                    { name: "Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Completer Abort", lsb: 15, bits: 1 },
                    { name: "Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Malformed TLP", lsb: 18, bits: 1 },
                    { name: "ECRC Error", lsb: 19, bits: 1 },
                    { name: "Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "ACS Violation", lsb: 21, bits: 1 },
                    { name: "Internal Error", lsb: 22, bits: 1 },
                    { name: "MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                    { name: "DMWr Request Egress Blocked", lsb: 27, bits: 1 },
                    { name: "IDE Check Failed", lsb: 28, bits: 1 },
                    { name: "Misrouted IDE TLP", lsb: 29, bits: 1 },
                    { name: "PCRC Check Failed", lsb: 30, bits: 1 },
                    { name: "TLP Translation Egress Blocked", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "Uncorrectable Error Mask",
                offset: 0x08,
                id: AER_V3_UNCORRECTABLE_ERROR_MASK,
                size: Dword,
                fields: [
                    { name: "Mask Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Mask Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Mask Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Mask Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Mask Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Mask Completer Abort", lsb: 15, bits: 1 },
                    { name: "Mask Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Mask Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Mask Malformed TLP", lsb: 18, bits: 1 },
                    { name: "Mask ECRC Error", lsb: 19, bits: 1 },
                    { name: "Mask Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "Mask ACS Violation", lsb: 21, bits: 1 },
                    { name: "Mask Internal Error", lsb: 22, bits: 1 },
                    { name: "Mask MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "Mask AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "Mask TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Mask Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                ]
            },
            {
                name: "Uncorrectable Error Severity",
                offset: 0x0c,
                id: AER_V3_UNCORRECTABLE_ERROR_SEVERITY,
                size: Dword,
                fields: [
                    { name: "Severity Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Severity Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Severity Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Severity Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Severity Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Severity Completer Abort", lsb: 15, bits: 1 },
                    { name: "Severity Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Severity Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Severity Malformed TLP", lsb: 18, bits: 1 },
                    { name: "Severity ECRC Error", lsb: 19, bits: 1 },
                    { name: "Severity Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "Severity ACS Violation", lsb: 21, bits: 1 },
                    { name: "Severity Internal Error", lsb: 22, bits: 1 },
                    { name: "Severity MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "Severity AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "Severity TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Severity Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                ]
            },
            {
                name: "Correctable Error Status",
                offset: 0x10,
                id: AER_V3_CORRECTABLE_ERROR_STAT,
                size: Dword,
                fields: [
                    { name: "Receiver Error", lsb: 0, bits: 1 },
                    { name: "Bad TLP", lsb: 6, bits: 1 },
                    { name: "Bad DLLP", lsb: 7, bits: 1 },
                    { name: "Replay Num Rollover", lsb: 8, bits: 1 },
                    { name: "Replay Timer Timeout", lsb: 12, bits: 1 },
                    { name: "Advisory Non-Fatal Error", lsb: 13, bits: 1 },
                    { name: "Corrected Internal Error", lsb: 14, bits: 1 },
                    { name: "Header Log Overflow", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Correctable Error Mask",
                offset: 0x14,
                id: AER_V3_CORRECTABLE_ERROR_MASK,
                size: Dword,
                fields: [
                    { name: "Mask Receiver Error", lsb: 0, bits: 1 },
                    { name: "Mask Bad TLP", lsb: 6, bits: 1 },
                    { name: "Mask Bad DLLP", lsb: 7, bits: 1 },
                    { name: "Mask Replay Num Rollover", lsb: 8, bits: 1 },
                    { name: "Mask Replay Timer Timeout", lsb: 12, bits: 1 },
                    { name: "Mask Advisory Non-Fatal Error", lsb: 13, bits: 1 },
                    { name: "Mask Corrected Internal Error", lsb: 14, bits: 1 },
                    { name: "Mask Header Log Overflow", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Advanced Capabilities",
                offset: 0x18,
                id: AER_V3_ADVANCED_CAPS,
                size: Dword,
                fields: [
                    { name: "First Error Pointer", lsb: 0, bits: 5 },
                    { name: "ECRC Generation Capable", lsb: 5, bits: 1 },
                    { name: "ECRC Generation Enable", lsb: 6, bits: 1 },
                    { name: "ECRC Check Capable", lsb: 7, bits: 1 },
                    { name: "ECRC Check Enable", lsb: 8, bits: 1 },
                    { name: "Multiple Header Recording Capable", lsb: 9, bits: 1 },
                    { name: "Multiple Header Recording Enable", lsb: 10, bits: 1 },
                    { name: "TLP Prefix Log Present", lsb: 11, bits: 1 },
                    { name: "Completion Timeout Prefix/Header Log Capable", lsb: 12, bits: 1 },
                    { name: "Header Log Size", lsb: 13, bits: 5 },
                    { name: "Logged TLP was Flit Mode", lsb: 18, bits: 1 },
                    { name: "Logged TLP Size", lsb: 19, bits: 5 },
                    { name: "RC ECS Handling Capable", lsb: 24, bits: 1 },
                ]
            },
            { name: "Header Log 0", offset: 0x1c, id: AER_V3_HEADER_LOG0, size: Dword, fields: [] },
            { name: "Header Log 1", offset: 0x20, id: AER_V3_HEADER_LOG1, size: Dword, fields: [] },
            { name: "Header Log 2", offset: 0x24, id: AER_V3_HEADER_LOG2, size: Dword, fields: [] },
            { name: "Header Log 3", offset: 0x28, id: AER_V3_HEADER_LOG3, size: Dword, fields: [] },
            {
                name: "Root Error Command",
                offset: 0x2c,
                id: AER_V3_ROOT_ERROR_CMD,
                size: Dword,
                fields: [
                    { name: "Correctable Error Reporting Enable", lsb: 0, bits: 1 },
                    { name: "Non-Fatal Error Reporting Enable", lsb: 1, bits: 1 },
                    { name: "Fatal Error Reporting Enable", lsb: 2, bits: 1 },
                    { name: "ECS Legacy Handling by SFW Enable", lsb: 3, bits: 1 },
                    { name: "ECS SIG_SFW Handling by SFW Enable", lsb: 4, bits: 1 },
                    { name: "ECS SIG_OS Handling by SFW Enable", lsb: 5, bits: 1 },
                    { name: "SFW Handling Source ID", lsb: 16, bits: 16 },
                ]
            },
            {
                name: "Root Error Status",
                offset: 0x30,
                id: AER_V3_ROOT_ERROR_STAT,
                size: Dword,
                fields: [
                    { name: "ERR_COR Received", lsb: 0, bits: 1 },
                    { name: "Multiple ERR_COR Received", lsb: 1, bits: 1 },
                    { name: "ERR_FATAL/NONFATAL Received", lsb: 2, bits: 1 },
                    { name: "Multiple ERR_FATAL/NONFATAL Received", lsb: 3, bits: 1 },
                    { name: "First Uncorrectable Fatal", lsb: 4, bits: 1 },
                    { name: "Non-Fatal Error Received", lsb: 5, bits: 1 },
                    { name: "Fatal Error Received", lsb: 6, bits: 1 },
                    { name: "ERR_COR Subclass", lsb: 7, bits: 2 },
                    { name: "SFW Handling ERR_COR Received", lsb: 9, bits: 1 },
                    { name: "Multiple SFW Handling ERR_COR Received", lsb: 10, bits: 1 },
                    { name: "Advanced Error Interrupt Message Number", lsb: 27, bits: 5 },
                ]
            },
            {
                name: "Error Source Identification",
                offset: 0x34,
                id: AER_V3_ERROR_SOURCE_IDENTIFICATION,
                size: Dword,
                fields: [
                    { name: "ERR_COR Source Identification", lsb: 0, bits: 16 },
                    { name: "ERR_FATAL/NONFATAL Source Identification", lsb: 16, bits: 16 },
                ]
            },
            { name: "Header Log 4 / TLP Prefix Log 0", offset: 0x38, id: AER_V3_HEADER_LOG_4_TLP_PREFIX_LOG0, size: Dword, fields: [] },
            { name: "Header Log 5 / TLP Prefix Log 1", offset: 0x3c, id: AER_V3_HEADER_LOG_5_TLP_PREFIX_LOG1, size: Dword, fields: [] },
            { name: "Header Log 6 / TLP Prefix Log 2", offset: 0x40, id: AER_V3_HEADER_LOG_6_TLP_PREFIX_LOG2, size: Dword, fields: [] },
            { name: "Header Log 7 / TLP Prefix Log 3", offset: 0x44, id: AER_V3_HEADER_LOG_7_TLP_PREFIX_LOG3, size: Dword, fields: [] },
            { name: "Header Log 8", offset: 0x48, id: AER_V3_HEADER_LOG8, size: Dword, fields: [] },
            { name: "Header Log 9", offset: 0x4c, id: AER_V3_HEADER_LOG9, size: Dword, fields: [] },
            { name: "Header Log 10", offset: 0x50, id: AER_V3_HEADER_LOG10, size: Dword, fields: [] },
            { name: "Header Log 11", offset: 0x54, id: AER_V3_HEADER_LOG11, size: Dword, fields: [] },
            { name: "Header Log 12", offset: 0x58, id: AER_V3_HEADER_LOG12, size: Dword, fields: [] },
            { name: "Header Log 13", offset: 0x5c, id: AER_V3_HEADER_LOG13, size: Dword, fields: [] },
        ]
    },
    {
        id: 0x0001,
        version: 2,
        is_extended: true,
        name: "Advanced Error Reporting",
        get_size: get_size_v2,
        summary: summary,
        registers: [
            {
                name: "Uncorrectable Error Status",
                offset: 0x04,
                id: AER_V2_UNCORRECTABLE_ERROR_STAT,
                size: Dword,
                fields: [
                    { name: "Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Completer Abort", lsb: 15, bits: 1 },
                    { name: "Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Malformed TLP", lsb: 18, bits: 1 },
                    { name: "ECRC Error", lsb: 19, bits: 1 },
                    { name: "Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "ACS Violation", lsb: 21, bits: 1 },
                    { name: "Internal Error", lsb: 22, bits: 1 },
                    { name: "MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                    { name: "DMWr Request Egress Blocked", lsb: 27, bits: 1 },
                    { name: "IDE Check Failed", lsb: 28, bits: 1 },
                    { name: "Misrouted IDE TLP", lsb: 29, bits: 1 },
                    { name: "PCRC Check Failed", lsb: 30, bits: 1 },
                    { name: "TLP Translation Egress Blocked", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "Uncorrectable Error Mask",
                offset: 0x08,
                id: AER_V2_UNCORRECTABLE_ERROR_MASK,
                size: Dword,
                fields: [
                    { name: "Mask Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Mask Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Mask Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Mask Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Mask Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Mask Completer Abort", lsb: 15, bits: 1 },
                    { name: "Mask Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Mask Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Mask Malformed TLP", lsb: 18, bits: 1 },
                    { name: "Mask ECRC Error", lsb: 19, bits: 1 },
                    { name: "Mask Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "Mask ACS Violation", lsb: 21, bits: 1 },
                    { name: "Mask Internal Error", lsb: 22, bits: 1 },
                    { name: "Mask MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "Mask AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "Mask TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Mask Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                ]
            },
            {
                name: "Uncorrectable Error Severity",
                offset: 0x0c,
                id: AER_V2_UNCORRECTABLE_ERROR_SEVERITY,
                size: Dword,
                fields: [
                    { name: "Severity Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Severity Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Severity Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Severity Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Severity Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Severity Completer Abort", lsb: 15, bits: 1 },
                    { name: "Severity Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Severity Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Severity Malformed TLP", lsb: 18, bits: 1 },
                    { name: "Severity ECRC Error", lsb: 19, bits: 1 },
                    { name: "Severity Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "Severity ACS Violation", lsb: 21, bits: 1 },
                    { name: "Severity Internal Error", lsb: 22, bits: 1 },
                    { name: "Severity MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "Severity AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "Severity TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Severity Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                ]
            },
            {
                name: "Correctable Error Status",
                offset: 0x10,
                id: AER_V2_CORRECTABLE_ERROR_STAT,
                size: Dword,
                fields: [
                    { name: "Receiver Error", lsb: 0, bits: 1 },
                    { name: "Bad TLP", lsb: 6, bits: 1 },
                    { name: "Bad DLLP", lsb: 7, bits: 1 },
                    { name: "Replay Num Rollover", lsb: 8, bits: 1 },
                    { name: "Replay Timer Timeout", lsb: 12, bits: 1 },
                    { name: "Advisory Non-Fatal Error", lsb: 13, bits: 1 },
                    { name: "Corrected Internal Error", lsb: 14, bits: 1 },
                    { name: "Header Log Overflow", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Correctable Error Mask",
                offset: 0x14,
                id: AER_V2_CORRECTABLE_ERROR_MASK,
                size: Dword,
                fields: [
                    { name: "Mask Receiver Error", lsb: 0, bits: 1 },
                    { name: "Mask Bad TLP", lsb: 6, bits: 1 },
                    { name: "Mask Bad DLLP", lsb: 7, bits: 1 },
                    { name: "Mask Replay Num Rollover", lsb: 8, bits: 1 },
                    { name: "Mask Replay Timer Timeout", lsb: 12, bits: 1 },
                    { name: "Mask Advisory Non-Fatal Error", lsb: 13, bits: 1 },
                    { name: "Mask Corrected Internal Error", lsb: 14, bits: 1 },
                    { name: "Mask Header Log Overflow", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Advanced Capabilities",
                offset: 0x18,
                id: AER_V2_ADVANCED_CAPS,
                size: Dword,
                fields: [
                    { name: "First Error Pointer", lsb: 0, bits: 5 },
                    { name: "ECRC Generation Capable", lsb: 5, bits: 1 },
                    { name: "ECRC Generation Enable", lsb: 6, bits: 1 },
                    { name: "ECRC Check Capable", lsb: 7, bits: 1 },
                    { name: "ECRC Check Enable", lsb: 8, bits: 1 },
                    { name: "Multiple Header Recording Capable", lsb: 9, bits: 1 },
                    { name: "Multiple Header Recording Enable", lsb: 10, bits: 1 },
                    { name: "TLP Prefix Log Present", lsb: 11, bits: 1 },
                    { name: "Completion Timeout Prefix/Header Log Capable", lsb: 12, bits: 1 },
                    { name: "Header Log Size", lsb: 13, bits: 5 },
                    { name: "Logged TLP was Flit Mode", lsb: 18, bits: 1 },
                    { name: "Logged TLP Size", lsb: 19, bits: 5 },
                    { name: "RC ECS Handling Capable", lsb: 24, bits: 1 },
                ]
            },
            { name: "Header Log 0", offset: 0x1c, id: AER_V2_HEADER_LOG0, size: Dword, fields: [] },
            { name: "Header Log 1", offset: 0x20, id: AER_V2_HEADER_LOG1, size: Dword, fields: [] },
            { name: "Header Log 2", offset: 0x24, id: AER_V2_HEADER_LOG2, size: Dword, fields: [] },
            { name: "Header Log 3", offset: 0x28, id: AER_V2_HEADER_LOG3, size: Dword, fields: [] },
            {
                name: "Root Error Command",
                offset: 0x2c,
                id: AER_V2_ROOT_ERROR_CMD,
                size: Dword,
                fields: [
                    { name: "Correctable Error Reporting Enable", lsb: 0, bits: 1 },
                    { name: "Non-Fatal Error Reporting Enable", lsb: 1, bits: 1 },
                    { name: "Fatal Error Reporting Enable", lsb: 2, bits: 1 },
                    { name: "ECS Legacy Handling by SFW Enable", lsb: 3, bits: 1 },
                    { name: "ECS SIG_SFW Handling by SFW Enable", lsb: 4, bits: 1 },
                    { name: "ECS SIG_OS Handling by SFW Enable", lsb: 5, bits: 1 },
                    { name: "SFW Handling Source ID", lsb: 16, bits: 16 },
                ]
            },
            {
                name: "Root Error Status",
                offset: 0x30,
                id: AER_V2_ROOT_ERROR_STAT,
                size: Dword,
                fields: [
                    { name: "ERR_COR Received", lsb: 0, bits: 1 },
                    { name: "Multiple ERR_COR Received", lsb: 1, bits: 1 },
                    { name: "ERR_FATAL/NONFATAL Received", lsb: 2, bits: 1 },
                    { name: "Multiple ERR_FATAL/NONFATAL Received", lsb: 3, bits: 1 },
                    { name: "First Uncorrectable Fatal", lsb: 4, bits: 1 },
                    { name: "Non-Fatal Error Received", lsb: 5, bits: 1 },
                    { name: "Fatal Error Received", lsb: 6, bits: 1 },
                    { name: "ERR_COR Subclass", lsb: 7, bits: 2 },
                    { name: "SFW Handling ERR_COR Received", lsb: 9, bits: 1 },
                    { name: "Multiple SFW Handling ERR_COR Received", lsb: 10, bits: 1 },
                    { name: "Advanced Error Interrupt Message Number", lsb: 27, bits: 5 },
                ]
            },
            {
                name: "Error Source Identification",
                offset: 0x34,
                id: AER_V2_ERROR_SOURCE_IDENTIFICATION,
                size: Dword,
                fields: [
                    { name: "ERR_COR Source Identification", lsb: 0, bits: 16 },
                    { name: "ERR_FATAL/NONFATAL Source Identification", lsb: 16, bits: 16 },
                ]
            },
            { name: "TLP Prefix Log 0", offset: 0x38, id: AER_V2_TLP_PREFIX_LOG0, size: Dword, fields: [] },
            { name: "TLP Prefix Log 1", offset: 0x3c, id: AER_V2_TLP_PREFIX_LOG1, size: Dword, fields: [] },
            { name: "TLP Prefix Log 2", offset: 0x40, id: AER_V2_TLP_PREFIX_LOG2, size: Dword, fields: [] },
            { name: "TLP Prefix Log 3", offset: 0x44, id: AER_V2_TLP_PREFIX_LOG3, size: Dword, fields: [] },
        ]
    },
    {
        id: 0x0001,
        version: 1,
        is_extended: true,
        name: "Advanced Error Reporting",
        get_size: get_size_v1,
        summary: summary,
        registers: [
            {
                name: "Uncorrectable Error Status",
                offset: 0x04,
                id: AER_V1_UNCORRECTABLE_ERROR_STAT,
                size: Dword,
                fields: [
                    { name: "Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Completer Abort", lsb: 15, bits: 1 },
                    { name: "Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Malformed TLP", lsb: 18, bits: 1 },
                    { name: "ECRC Error", lsb: 19, bits: 1 },
                    { name: "Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "ACS Violation", lsb: 21, bits: 1 },
                    { name: "Internal Error", lsb: 22, bits: 1 },
                    { name: "MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                    { name: "DMWr Request Egress Blocked", lsb: 27, bits: 1 },
                    { name: "IDE Check Failed", lsb: 28, bits: 1 },
                    { name: "Misrouted IDE TLP", lsb: 29, bits: 1 },
                    { name: "PCRC Check Failed", lsb: 30, bits: 1 },
                    { name: "TLP Translation Egress Blocked", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "Uncorrectable Error Mask",
                offset: 0x08,
                id: AER_V1_UNCORRECTABLE_ERROR_MASK,
                size: Dword,
                fields: [
                    { name: "Mask Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Mask Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Mask Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Mask Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Mask Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Mask Completer Abort", lsb: 15, bits: 1 },
                    { name: "Mask Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Mask Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Mask Malformed TLP", lsb: 18, bits: 1 },
                    { name: "Mask ECRC Error", lsb: 19, bits: 1 },
                    { name: "Mask Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "Mask ACS Violation", lsb: 21, bits: 1 },
                    { name: "Mask Internal Error", lsb: 22, bits: 1 },
                    { name: "Mask MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "Mask AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "Mask TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Mask Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                ]
            },
            {
                name: "Uncorrectable Error Severity",
                offset: 0x0c,
                id: AER_V1_UNCORRECTABLE_ERROR_SEVERITY,
                size: Dword,
                fields: [
                    { name: "Severity Data Link Protocol Error", lsb: 4, bits: 1 },
                    { name: "Severity Surprise Down Error", lsb: 5, bits: 1 },
                    { name: "Severity Poisoned TLP Received", lsb: 12, bits: 1 },
                    { name: "Severity Flow Control Protocol Error", lsb: 13, bits: 1 },
                    { name: "Severity Completion Timeout", lsb: 14, bits: 1 },
                    { name: "Severity Completer Abort", lsb: 15, bits: 1 },
                    { name: "Severity Unexpected Completion", lsb: 16, bits: 1 },
                    { name: "Severity Receiver Overflow", lsb: 17, bits: 1 },
                    { name: "Severity Malformed TLP", lsb: 18, bits: 1 },
                    { name: "Severity ECRC Error", lsb: 19, bits: 1 },
                    { name: "Severity Unsupported Request Error", lsb: 20, bits: 1 },
                    { name: "Severity ACS Violation", lsb: 21, bits: 1 },
                    { name: "Severity Internal Error", lsb: 22, bits: 1 },
                    { name: "Severity MC Blocked TLP", lsb: 23, bits: 1 },
                    { name: "Severity AtomicOp Egress Blocked", lsb: 24, bits: 1 },
                    { name: "Severity TLP Prefix Blocked", lsb: 25, bits: 1 },
                    { name: "Severity Poisoned TLP Egress Blocked", lsb: 26, bits: 1 },
                ]
            },
            {
                name: "Correctable Error Status",
                offset: 0x10,
                id: AER_V1_CORRECTABLE_ERROR_STAT,
                size: Dword,
                fields: [
                    { name: "Receiver Error", lsb: 0, bits: 1 },
                    { name: "Bad TLP", lsb: 6, bits: 1 },
                    { name: "Bad DLLP", lsb: 7, bits: 1 },
                    { name: "Replay Num Rollover", lsb: 8, bits: 1 },
                    { name: "Replay Timer Timeout", lsb: 12, bits: 1 },
                    { name: "Advisory Non-Fatal Error", lsb: 13, bits: 1 },
                    { name: "Corrected Internal Error", lsb: 14, bits: 1 },
                    { name: "Header Log Overflow", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Correctable Error Mask",
                offset: 0x14,
                id: AER_V1_CORRECTABLE_ERROR_MASK,
                size: Dword,
                fields: [
                    { name: "Mask Receiver Error", lsb: 0, bits: 1 },
                    { name: "Mask Bad TLP", lsb: 6, bits: 1 },
                    { name: "Mask Bad DLLP", lsb: 7, bits: 1 },
                    { name: "Mask Replay Num Rollover", lsb: 8, bits: 1 },
                    { name: "Mask Replay Timer Timeout", lsb: 12, bits: 1 },
                    { name: "Mask Advisory Non-Fatal Error", lsb: 13, bits: 1 },
                    { name: "Mask Corrected Internal Error", lsb: 14, bits: 1 },
                    { name: "Mask Header Log Overflow", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Advanced Capabilities",
                offset: 0x18,
                id: AER_V1_ADVANCED_CAPS,
                size: Dword,
                fields: [
                    { name: "First Error Pointer", lsb: 0, bits: 5 },
                    { name: "ECRC Generation Capable", lsb: 5, bits: 1 },
                    { name: "ECRC Generation Enable", lsb: 6, bits: 1 },
                    { name: "ECRC Check Capable", lsb: 7, bits: 1 },
                    { name: "ECRC Check Enable", lsb: 8, bits: 1 },
                    { name: "Multiple Header Recording Capable", lsb: 9, bits: 1 },
                    { name: "Multiple Header Recording Enable", lsb: 10, bits: 1 },
                    { name: "TLP Prefix Log Present", lsb: 11, bits: 1 },
                    { name: "Completion Timeout Prefix/Header Log Capable", lsb: 12, bits: 1 },
                ]
            },
        ]
    }
}

fn get_size_v3(cap: &PciCapa) -> Option<u16> {
    aer_size_with_header_log_size(cap, AER_V3_ADVANCED_CAPS)
}

fn get_size_v2(cap: &PciCapa) -> Option<u16> {
    aer_size_with_header_log_size(cap, AER_V2_ADVANCED_CAPS)
}

fn get_size_v1(cap: &PciCapa) -> Option<u16> {
    aer_size_legacy(cap, AER_V1_ADVANCED_CAPS)
}

fn aer_size_with_header_log_size(cap: &PciCapa, adv_off: u16) -> Option<u16> {
    let adv = cap.read_u32(u64::from(adv_off)).ok()?;
    let header_dw = aer_header_log_dw(adv);
    Some(aer_total_len(cap, header_dw))
}

fn aer_size_legacy(cap: &PciCapa, adv_off: u16) -> Option<u16> {
    let adv = cap.read_u32(u64::from(adv_off)).ok()?;
    let tlp_prefix = (adv >> 11) & 0x1 != 0;
    let header_dw = if tlp_prefix { 8 } else { 4 };
    Some(aer_total_len(cap, header_dw))
}

fn aer_header_log_dw(adv: u32) -> u16 {
    let mut dw = ((adv >> 13) & 0x1f) as u16;
    if dw == 0 {
        let tlp_prefix = (adv >> 11) & 0x1 != 0;
        dw = if tlp_prefix { 8 } else { 4 };
    }
    if dw < 4 {
        dw = 4;
    }
    dw
}

fn aer_total_len(cap: &PciCapa, header_dw: u16) -> u16 {
    let base = if aer_has_root_regs(cap) { 0x38 } else { 0x2c };
    let extra_dw = header_dw.saturating_sub(4);
    base + extra_dw * 4
}

fn aer_has_root_regs(cap: &PciCapa) -> bool {
    let Some(pcie_off) = cap.pcie_cap_offset() else {
        return true;
    };
    let Ok(pcie_cap) = cap.read_cfg_u16(pcie_off + 0x02) else {
        return true;
    };
    let port_type = (pcie_cap >> 4) & 0x0f;
    is_root_port_type(port_type)
}

fn is_root_port_type(port_type: u16) -> bool {
    matches!(port_type, 0x4 | 0x9 | 0xa)
}

fn summary(cap: &PciCapa) -> Option<Vec<TreeNode>> {
    let mut nodes = Vec::new();

    if let Some(node) = summarize_errors(
        cap,
        "Uncorrectable Errors",
        0x04,
        0x08,
        &[
            (4, "DLProtocol"),
            (5, "SurpriseDown"),
            (12, "PoisonedTLP"),
            (13, "FCProtocol"),
            (14, "CompletionTimeout"),
            (15, "CompleterAbort"),
            (16, "UnexpectedCompletion"),
            (17, "ReceiverOverflow"),
            (18, "MalformedTLP"),
            (19, "ECRC"),
            (20, "UnsupReq"),
            (21, "ACSViolation"),
            (22, "InternalError"),
            (23, "MCBlockedTLP"),
            (24, "AtomicOpEgressBlocked"),
            (25, "TLPPrefixBlocked"),
            (26, "PoisonedTLPEgressBlocked"),
            (27, "DMWRRequestEgressBlocked"),
            (28, "IDECheckFailed"),
            (29, "MisroutedIDETLP"),
            (30, "PCRCCheckFailed"),
            (31, "TLPTranslationEgressBlocked"),
        ],
    ) {
        nodes.push(node);
    }
    if let Some(node) = summarize_errors(
        cap,
        "Correctable Errors",
        0x10,
        0x14,
        &[
            (0, "Receiver"),
            (6, "BadTLP"),
            (7, "BadDLLP"),
            (8, "ReplayRollover"),
            (12, "ReplayTimeout"),
            (13, "AdvisoryNonFatal"),
            (14, "CorrectedInternal"),
            (15, "HeaderOverflow"),
        ],
    ) {
        nodes.push(node);
    }
    if nodes.is_empty() { None } else { Some(nodes) }
}

fn summarize_errors(
    cap: &PciCapa,
    label: &str,
    status_off: u16,
    mask_off: u16,
    table: &[(u8, &str)],
) -> Option<TreeNode> {
    let status = cap.read_u32(status_off as u64).ok()?;
    let mask = cap.read_u32(mask_off as u64).unwrap_or(0);
    let active = status & !mask;
    if active == 0 {
        return Some(TreeNode::with_value(
            TreeLine::from(label.to_string()),
            TreeLine::from(vec![TreeSpan::styled("None", TreeColor::Green)]),
        ));
    }
    let mut spans = Vec::new();
    let mut first = true;
    for (bit, name) in table {
        if active & (1 << bit) != 0 {
            if !first {
                spans.push(TreeSpan::raw(", "));
            }
            spans.push(TreeSpan::styled((*name).to_string(), TreeColor::Red));
            first = false;
        }
    }
    Some(TreeNode::with_value(
        TreeLine::from(label.to_string()),
        TreeLine::from(spans),
    ))
}
