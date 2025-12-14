use crate::capabilities;
use crate::pci_capa::{RegisterSize, read_raw};
use crate::tree::PciNode;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

capabilities! {
    std {
        id: 0x10,
        name: "PCI Express",
        summary: pcie_summary,
        registers: [
            {
                name: "Capabilities",
                offset: 0x02,
                size: Word,
                fields: [
                    { name: "PCIe Capability Version", lsb: 0, bits: 4 },
                    {
                        name: "Device/Port Type",
                        lsb: 4,
                        bits: 4,
                        enum_values: [
                            (0x0, "PCI Express Endpoint"),
                            (0x1, "Legacy PCI Express Endpoint"),
                            (0x4, "Root Port of PCI Express Root Complex"),
                            (0x5, "Upstream Port of PCI Express Switch"),
                            (0x6, "Downstream Port of PCI Express Switch"),
                            (0x7, "PCI Express to PCI/PCI-X Bridge"),
                            (0x8, "PCI/PCI-X to PCI Express Bridge"),
                            (0x9, "RCiEP"),
                            (0xa, "Root Complex Event Collector"),
                        ]
                    },
                    { name: "Slot Implemented", lsb: 8, bits: 1 },
                ]
            },
            {
                name: "Device Capabilities",
                offset: 0x04,
                size: Dword,
                fields: [
                    {
                        name: "Max Payload Size Supported",
                        lsb: 0,
                        bits: 3,
                        enum_values: [
                            (0x0, "128 bytes"),
                            (0x1, "256 bytes"),
                            (0x2, "512 bytes"),
                            (0x3, "1024 bytes"),
                            (0x4, "2048 bytes"),
                            (0x5, "4096 bytes"),
                        ]
                    },
                    { name: "Phantom Functions Supported", lsb: 3, bits: 2 },
                    { name: "Extended Tag Supported", lsb: 5, bits: 1 },
                    { name: "Attention Button Present", lsb: 6, bits: 1 },
                    { name: "Attention Indicator Present", lsb: 7, bits: 1 },
                    { name: "Power Indicator Present", lsb: 8, bits: 1 },
                    { name: "Role-Based Error Reporting", lsb: 15, bits: 1 },
                    { name: "Endpoint L0s Acceptable Latency", lsb: 12, bits: 3 },
                    { name: "Endpoint L1 Acceptable Latency", lsb: 15, bits: 3 },
                    { name: "Slot Power Limit Value", lsb: 18, bits: 8 },
                    {
                        name: "Slot Power Limit Scale",
                        lsb: 26,
                        bits: 2,
                        enum_values: [
                            (0x0, "1.0x"),
                            (0x1, "0.1x"),
                            (0x2, "0.01x"),
                            (0x3, "0.001x"),
                        ]
                    },
                    { name: "Function-Level Reset", lsb: 28, bits: 1 },
                    { name: "TEE-IO Supported", lsb: 30, bits: 1 },
                ]
            },
            {
                name: "Device Control",
                offset: 0x08,
                size: Word,
                fields: [
                    { name: "Correctable Error Reporting Enable", lsb: 0, bits: 1 },
                    { name: "Non-Fatal Error Reporting Enable", lsb: 1, bits: 1 },
                    { name: "Fatal Error Reporting Enable", lsb: 2, bits: 1 },
                    { name: "Unsupported Request Reporting Enable", lsb: 3, bits: 1 },
                    { name: "Enable Relaxed Ordering", lsb: 4, bits: 1 },
                    {
                        name: "Max Payload Size",
                        lsb: 5,
                        bits: 2,
                        enum_values: [
                            (0x0, "128 bytes"),
                            (0x1, "256 bytes"),
                            (0x2, "512 bytes"),
                            (0x3, "1024 bytes"),
                        ]
                    },
                    { name: "Extended Tag Field Enable", lsb: 8, bits: 1 },
                    { name: "Phantom Functions Enable", lsb: 9, bits: 1 },
                    { name: "Aux Power PM Enable", lsb: 10, bits: 1 },
                    { name: "Enable No Snoop", lsb: 11, bits: 1 },
                    {
                        name: "Max Read Request Size",
                        lsb: 12,
                        bits: 3,
                        enum_values: [
                            (0x0, "128 bytes"),
                            (0x1, "256 bytes"),
                            (0x2, "512 bytes"),
                            (0x3, "1024 bytes"),
                            (0x4, "2048 bytes"),
                            (0x5, "4096 bytes"),
                        ]
                    },
                    { name: "Function Level Reset/BCRE", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Device Status",
                offset: 0x0a,
                size: Word,
                fields: [
                    { name: "Correctable Error Detected", lsb: 0, bits: 1 },
                    { name: "Non-Fatal Error Detected", lsb: 1, bits: 1 },
                    { name: "Fatal Error Detected", lsb: 2, bits: 1 },
                    { name: "Unsupported Request Detected", lsb: 3, bits: 1 },
                    { name: "AUX Power Detected", lsb: 4, bits: 1 },
                    { name: "Transactions Pending", lsb: 5, bits: 1 },
                ]
            },
            {
                name: "Link Capabilities",
                offset: 0x0c,
                size: Dword,
                fields: [
                    {
                        name: "Maximum Link Speed",
                        lsb: 0,
                        bits: 4,
                        enum_values: [
                            (0x1, "2.5 GT/s"),
                            (0x2, "5.0 GT/s"),
                            (0x3, "8.0 GT/s"),
                            (0x4, "16.0 GT/s"),
                            (0x5, "32.0 GT/s"),
                            (0x6, "64.0 GT/s"),
                            (0x7, "128.0 GT/s"),
                        ]
                    },
                    { name: "Maximum Link Width", lsb: 4, bits: 6 },
                    {
                        name: "ASPM Support",
                        lsb: 10,
                        bits: 2,
                        enum_values: [
                            (0x0, "not supported"),
                            (0x1, "L0s"),
                            (0x2, "L1"),
                            (0x3, "L0s L1"),
                        ]
                    },
                    { name: "L0s Exit Latency", lsb: 12, bits: 3 },
                    { name: "L1 Exit Latency", lsb: 15, bits: 3 },
                    { name: "Clock Power Management", lsb: 18, bits: 1 },
                    { name: "Surprise Down Error Reporting", lsb: 19, bits: 1 },
                    { name: "Data Link Layer Active Reporting", lsb: 20, bits: 1 },
                    { name: "Link Bandwidth Notification Capability", lsb: 21, bits: 1 },
                    { name: "ASPM Optionality Compliance", lsb: 22, bits: 1 },
                    { name: "Port Number", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "Link Control",
                offset: 0x10,
                size: Word,
                fields: [
                    {
                        name: "ASPM Control",
                        lsb: 0,
                        bits: 2,
                        enum_values: [
                            (0x0, "disabled"),
                            (0x1, "L0s enabled"),
                            (0x2, "L1 enabled"),
                            (0x3, "L0s L1 enabled"),
                        ]
                    },
                    {
                        name: "Read Completion Boundary",
                        lsb: 3,
                        bits: 1,
                        enum_values: [
                            (0x0, "64 bytes"),
                            (0x1, "128 bytes"),
                        ]
                    },
                    { name: "Link Disable", lsb: 4, bits: 1 },
                    { name: "Retrain Link", lsb: 5, bits: 1 },
                    { name: "Common Clock Configuration", lsb: 6, bits: 1 },
                    { name: "Extended Synch", lsb: 7, bits: 1 },
                    { name: "Clock Power Management", lsb: 8, bits: 1 },
                    { name: "Hardware Autonomous Width Disable", lsb: 9, bits: 1 },
                    { name: "Bandwidth Mgmt Interrupt Enable", lsb: 10, bits: 1 },
                    { name: "Autonomous BW Mgmt Interrupt Enable", lsb: 11, bits: 1 },
                    { name: "FLIT Mode Disable", lsb: 13, bits: 1 },
                ]
            },
            {
                name: "Link Status",
                offset: 0x12,
                size: Word,
                fields: [
                    {
                        name: "Negotiated Link Speed",
                        lsb: 0,
                        bits: 4,
                        enum_values: [
                            (0x1, "2.5 GT/s"),
                            (0x2, "5.0 GT/s"),
                            (0x3, "8.0 GT/s"),
                            (0x4, "16.0 GT/s"),
                            (0x5, "32.0 GT/s"),
                            (0x6, "64.0 GT/s"),
                            (0x7, "128.0 GT/s"),
                        ]
                    },
                    { name: "Negotiated Link Width", lsb: 4, bits: 6 },
                    { name: "Training Error", lsb: 10, bits: 1 },
                    { name: "Link Training", lsb: 11, bits: 1 },
                    { name: "Slot Clock Configuration", lsb: 12, bits: 1 },
                    { name: "Data Link Layer Active", lsb: 13, bits: 1 },
                    { name: "Bandwidth Mgmt Status", lsb: 14, bits: 1 },
                    { name: "Autonomous Bandwidth Status", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Slot Capabilities",
                offset: 0x14,
                size: Dword,
                fields: [
                    { name: "Attention Button Present", lsb: 0, bits: 1 },
                    { name: "Power Controller Present", lsb: 1, bits: 1 },
                    { name: "MRL Sensor Present", lsb: 2, bits: 1 },
                    { name: "Attention Indicator Present", lsb: 3, bits: 1 },
                    { name: "Power Indicator Present", lsb: 4, bits: 1 },
                    { name: "Hot-Plug Surprise", lsb: 5, bits: 1 },
                    { name: "Hot-Plug Capable", lsb: 6, bits: 1 },
                    { name: "Slot Power Limit Value", lsb: 7, bits: 8 },
                    { name: "Slot Power Limit Scale", lsb: 15, bits: 2 },
                    { name: "Electromechanical Interlock Present", lsb: 17, bits: 1 },
                    { name: "No Command Completed Support", lsb: 18, bits: 1 },
                    { name: "Physical Slot Number", lsb: 19, bits: 13 },
                ]
            },
            {
                name: "Slot Control",
                offset: 0x18,
                size: Word,
                fields: [
                    { name: "Attention Button Pressed Enable", lsb: 0, bits: 1 },
                    { name: "Power Fault Detected Enable", lsb: 1, bits: 1 },
                    { name: "MRL Sensor Changed Enable", lsb: 2, bits: 1 },
                    { name: "Presence Detect Changed Enable", lsb: 3, bits: 1 },
                    { name: "Command Completed Interrupt Enable", lsb: 4, bits: 1 },
                    { name: "Hot-Plug Interrupt Enable", lsb: 5, bits: 1 },
                    {
                        name: "Attention Indicator Control",
                        lsb: 6,
                        bits: 2,
                        enum_values: [
                            (0x1, "On"),
                            (0x2, "Blink"),
                            (0x3, "Off"),
                        ]
                    },
                    {
                        name: "Power Indicator Control",
                        lsb: 8,
                        bits: 2,
                        enum_values: [
                            (0x1, "On"),
                            (0x2, "Blink"),
                            (0x3, "Off"),
                        ]
                    },
                    { name: "Power Controller Control", lsb: 10, bits: 1 },
                    { name: "Electromechanical Interlock Control", lsb: 11, bits: 1 },
                    { name: "Data Link Layer State Changed Enable", lsb: 12, bits: 1 },
                ]
            },
            {
                name: "Slot Status",
                offset: 0x1a,
                size: Word,
                fields: [
                    { name: "Attention Button Pressed", lsb: 0, bits: 1 },
                    { name: "Power Fault Detected", lsb: 1, bits: 1 },
                    { name: "MRL Sensor Changed", lsb: 2, bits: 1 },
                    { name: "Presence Detect Changed", lsb: 3, bits: 1 },
                    { name: "Command Completed", lsb: 4, bits: 1 },
                    {
                        name: "MRL Sensor State",
                        lsb: 5,
                        bits: 1,
                        enum_values: [
                            (0x0, "Closed"),
                            (0x1, "Open"),
                        ]
                    },
                    {
                        name: "Presence Detect State",
                        lsb: 6,
                        bits: 1,
                        enum_values: [
                            (0x0, "Empty"),
                            (0x1, "Present"),
                        ]
                    },
                    { name: "Electromechanical Interlock Status", lsb: 7, bits: 1 },
                    { name: "Data Link Layer State Changed", lsb: 8, bits: 1 },
                ]
            },
            {
                name: "Root Control",
                offset: 0x1c,
                size: Word,
                fields: [
                    { name: "System Error on Correctable Error", lsb: 0, bits: 1 },
                    { name: "System Error on Non-Fatal Error", lsb: 1, bits: 1 },
                    { name: "System Error on Fatal Error", lsb: 2, bits: 1 },
                    { name: "PME Interrupt Enable", lsb: 3, bits: 1 },
                    { name: "CRS Software Visibility", lsb: 4, bits: 1 },
                ]
            },
            {
                name: "Root Capabilities",
                offset: 0x1e,
                size: Word,
                fields: [
                    { name: "CRS Software Visibility", lsb: 0, bits: 1 }
                ]
            },
            {
                name: "Root Status",
                offset: 0x20,
                size: Dword,
                fields: [
                    { name: "PME Requester ID", lsb: 0, bits: 16 },
                    { name: "PME Status", lsb: 16, bits: 1 },
                    { name: "PME Pending", lsb: 17, bits: 1 },
                ]
            },
            {
                name: "Device Capabilities 2",
                offset: 0x24,
                size: Dword,
                fields: [
                    { name: "Completion Timeout Ranges", lsb: 0, bits: 4 },
                    { name: "Completion Timeout Disable Supported", lsb: 4, bits: 1 },
                    { name: "ARI Forwarding Supported", lsb: 5, bits: 1 },
                    { name: "AtomicOp Routing Supported", lsb: 6, bits: 1 },
                    { name: "32-bit AtomicOp Completer", lsb: 7, bits: 1 },
                    { name: "64-bit AtomicOp Completer", lsb: 8, bits: 1 },
                    { name: "128-bit CAS Completer", lsb: 9, bits: 1 },
                    { name: "No RO-enabled PR-PR Passing", lsb: 10, bits: 1 },
                    { name: "LTR Supported", lsb: 11, bits: 1 },
                    { name: "TPH Completer Supported", lsb: 12, bits: 2 },
                    { name: "LN System CLS Supported", lsb: 14, bits: 2 },
                    { name: "10 Bit Tag Completer", lsb: 16, bits: 1 },
                    { name: "10 Bit Tag Requester", lsb: 17, bits: 1 },
                    {
                        name: "OBFF Supported",
                        lsb: 18,
                        bits: 2,
                        enum_values: [
                            (0x0, "Not supported"),
                            (0x1, "Message"),
                            (0x2, "WAKE#"),
                            (0x3, "Both"),
                        ]
                    },
                    { name: "Extended Fmt Field Supported", lsb: 20, bits: 1 },
                    { name: "End-End TLP Prefix Supported", lsb: 21, bits: 1 },
                    { name: "Max End-End TLP Prefixes", lsb: 22, bits: 2 },
                    { name: "Emergency Power Reduction Supported", lsb: 24, bits: 2 },
                    { name: "Emergency Power Reduction Init Required", lsb: 26, bits: 1 },
                    { name: "FRS Supported", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "Device Control 2",
                offset: 0x28,
                size: Word,
                fields: [
                    { name: "Completion Timeout Value", lsb: 0, bits: 4 },
                    { name: "Completion Timeout Disable", lsb: 4, bits: 1 },
                    { name: "ARI Forwarding", lsb: 5, bits: 1 },
                    { name: "AtomicOp Requester Enable", lsb: 6, bits: 1 },
                    { name: "AtomicOp Egress Blocking", lsb: 7, bits: 1 },
                    { name: "IDO Request Enable", lsb: 8, bits: 1 },
                    { name: "IDO Completion Enable", lsb: 9, bits: 1 },
                    { name: "LTR Enable", lsb: 10, bits: 1 },
                    { name: "Emergency Power Reduction Request", lsb: 11, bits: 1 },
                    { name: "10 Bit Tag Requester Enable", lsb: 12, bits: 1 },
                    {
                        name: "OBFF Enable",
                        lsb: 13,
                        bits: 2,
                        enum_values: [
                            (0x0, "Disabled"),
                            (0x1, "Variation A"),
                            (0x2, "Variation B"),
                        ]
                    },
                    { name: "End-End TLP Prefix Blocking", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Device Status 2",
                offset: 0x2a,
                size: Word,
                fields: []
            },
            {
                name: "Link Capabilities 2",
                offset: 0x2c,
                size: Dword,
                fields: [
                    { name: "Supported Link Speeds Vector", lsb: 1, bits: 7 },
                    { name: "Crosslink Supported", lsb: 8, bits: 1 },
                    { name: "Lower SKP OS Generation Supported Speeds Vector", lsb: 9, bits: 7 },
                    { name: "Lower SKP OS Reception Supported Speeds Vector", lsb: 16, bits: 7 },
                    { name: "Retimer Presence Detect Supported", lsb: 23, bits: 1 },
                    { name: "Two Retimers Presence Detect Supported", lsb: 24, bits: 1 },
                    { name: "Optical Retimer Presence Detect Supported", lsb: 25, bits: 1 },
                    { name: "FRA Retimers Detect Supported", lsb: 26, bits: 1 },
                    { name: "Device Readiness Status", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "Link Control 2",
                offset: 0x30,
                size: Word,
                fields: [
                    { name: "Target Link Speed", lsb: 0, bits: 4 },
                    { name: "Enter Compliance", lsb: 4, bits: 1 },
                    { name: "Hardware Autonomous Speed Disable", lsb: 5, bits: 1 },
                    { name: "Selectable De-emphasis", lsb: 6, bits: 1 },
                    { name: "Transmit Margin", lsb: 7, bits: 3 },
                    { name: "Enter Modified Compliance", lsb: 10, bits: 1 },
                    { name: "Compliance SOS", lsb: 11, bits: 1 },
                    { name: "Compliance Preset/De-emphasis", lsb: 12, bits: 4 },
                ]
            },
            {
                name: "Link Status 2",
                offset: 0x32,
                size: Word,
                fields: [
                    { name: "Current De-emphasis Level", lsb: 0, bits: 1 },
                    { name: "Equalization Complete", lsb: 1, bits: 1 },
                    { name: "Equalization Phase 1 Successful", lsb: 2, bits: 1 },
                    { name: "Equalization Phase 2 Successful", lsb: 3, bits: 1 },
                    { name: "Equalization Phase 3 Successful", lsb: 4, bits: 1 },
                    { name: "Link Equalization Request", lsb: 5, bits: 1 },
                    { name: "Retimer Detected", lsb: 6, bits: 1 },
                    { name: "Two Retimers Detected", lsb: 7, bits: 1 },
                    { name: "Crosslink Resolution", lsb: 8, bits: 2 },
                    { name: "FLIT Mode Active", lsb: 10, bits: 1 },
                    { name: "Optical Retimer Presence Detected", lsb: 11, bits: 1 },
                    { name: "Component Presence", lsb: 12, bits: 3 },
                    { name: "DRS Message Received", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Slot Capabilities 2",
                offset: 0x34,
                size: Dword,
                fields: [
                    { name: "In-Band PD Disable Supported", lsb: 0, bits: 1 },
                    {
                        name: "SCap2 OOB PD Supported",
                        lsb: 1,
                        bits: 2,
                        enum_values: [
                            (0x0, "Not supported"),
                            (0x1, "Via I2C"),
                            (0x2, "Via other means"),
                        ]
                    },
                ]
            },
            {
                name: "Slot Control 2",
                offset: 0x38,
                size: Word,
                fields: []
            },
            {
                name: "Slot Status 2",
                offset: 0x3a,
                size: Word,
                fields: []
            },
        ]
    }
}

fn pcie_summary(_off: u8, bytes: &[u8], _config: &[u8]) -> Option<Vec<PciNode>> {
    let mut nodes = Vec::new();
    let mut link_cap_speed: Option<u8> = None;
    let mut link_cap_width: Option<u8> = None;

    if let Some(raw) = read_raw(bytes, 0x0a, RegisterSize::Word) {
        let status = raw as u16;
        let mut flags: Vec<(&'static str, bool)> = Vec::new();
        if status & (1 << 0) != 0 {
            flags.push(("CorrErr", true));
        }
        if status & (1 << 1) != 0 {
            flags.push(("NonFatalErr", true));
        }
        if status & (1 << 2) != 0 {
            flags.push(("FatalErr", true));
        }
        if status & (1 << 3) != 0 {
            flags.push(("UnsupReq", true));
        }
        if status & (1 << 4) != 0 {
            flags.push(("AuxPwr", false));
        }
        if status & (1 << 5) != 0 {
            flags.push(("TransPend", false));
        }
        let summary = if flags.is_empty() {
            Line::from("none")
        } else {
            let mut spans = Vec::new();
            for (idx, (label, is_error)) in flags.iter().enumerate() {
                if idx > 0 {
                    spans.push(Span::raw(", "));
                }
                if *is_error {
                    spans.push(Span::styled(*label, Style::default().fg(Color::LightRed)));
                } else {
                    spans.push(Span::raw(*label));
                }
            }
            Line::from(spans)
        };
        nodes.push(PciNode::with_value(Line::from("Device Status"), summary));
    }

    if let Some(raw) = read_raw(bytes, 0x0c, RegisterSize::Dword) {
        let speed = (raw & 0x0f) as u8;
        let width = ((raw >> 4) & 0x3f) as u8;
        let aspm = ((raw >> 10) & 0x03) as u8;
        link_cap_speed = Some(speed);
        link_cap_width = Some(width);
        nodes.push(PciNode::with_value(
            Line::from("Link Capabilities"),
            Line::from(format!(
                "{} GT/s x{}, ASPM {}",
                link_speed_name(speed),
                width,
                aspm_support_name(aspm)
            )),
        ));
    }

    if let Some(raw) = read_raw(bytes, 0x12, RegisterSize::Word) {
        let speed = (raw & 0x0f) as u8;
        let width = ((raw >> 4) & 0x3f) as u8;
        let speed_style = match link_cap_speed {
            Some(cap) if speed < cap => Style::default().fg(Color::LightYellow),
            Some(_) => Style::default().fg(Color::LightGreen),
            None => Style::default(),
        };
        let width_style = match link_cap_width {
            Some(cap) if width < cap => Style::default().fg(Color::LightYellow),
            Some(_) => Style::default().fg(Color::LightGreen),
            None => Style::default(),
        };
        let line = Line::from(vec![
            Span::styled(format!("{} GT/s", link_speed_name(speed)), speed_style),
            Span::raw(" "),
            Span::styled(format!("x{}", width), width_style),
        ]);
        nodes.push(PciNode::with_value(Line::from("Link Status"), line));
    }

    if nodes.is_empty() { None } else { Some(nodes) }
}

fn aspm_support_name(code: u8) -> &'static str {
    match code {
        0 => "not supported",
        1 => "L0s",
        2 => "L1",
        3 => "L0s L1",
        _ => "unknown",
    }
}

fn link_speed_name(code: u8) -> &'static str {
    match code {
        1 => "2.5",
        2 => "5.0",
        3 => "8.0",
        4 => "16.0",
        5 => "32.0",
        6 => "64.0",
        _ => "unknown",
    }
}
