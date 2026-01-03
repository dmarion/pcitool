use crate::capabilities;
use crate::pci_device::PciCapa;
use crate::tree::{TreeLine, TreeNode};

capabilities! {
    {
        id: 0x01,
        name: "Power Management",
        size: 8,
        summary: pm_summary,
        registers: [
            {
                name: "Power Management Capabilities",
                offset: 0x02,
                id: POWER_MANAGEMENT_CAPS,
                size: Word,
                fields: [
                    { name: "Version", lsb: 0, bits: 3 },
                    { name: "PME clock required", lsb: 3, bits: 1 },
                    { name: "Immediate readiness on return to D0", lsb: 4, bits: 1 },
                    { name: "Device specific initialization", lsb: 5, bits: 1 },
                    {
                        name: "Auxiliary power",
                        lsb: 6,
                        bits: 3,
                        enum_values: [
                            (0x0, "0mW"),
                            (0x1, "182mW"),
                            (0x2, "330mW"),
                            (0x3, "528mW"),
                            (0x4, "726mW"),
                            (0x5, "891mW"),
                            (0x6, "1056mW"),
                            (0x7, "1238mW"),
                        ]
                    },
                    { name: "D1 power state support", lsb: 9, bits: 1 },
                    { name: "D2 power state support", lsb: 10, bits: 1 },
                    { name: "PME Support", lsb: 11, bits: 5 },
                ]
            },
            {
                name: "Power Management Control/Status",
                offset: 0x04,
                id: POWER_MANAGEMENT_CTRL_STAT,
                size: Word,
                fields: [
                    {
                        name: "Power State",
                        lsb: 0,
                        bits: 2,
                        enum_values: [
                            (0x0, "D0"),
                            (0x1, "D1"),
                            (0x2, "D2"),
                            (0x3, "D3hot"),
                        ]
                    },
                    { name: "No soft reset", lsb: 3, bits: 1 },
                    { name: "PME Enable", lsb: 8, bits: 1 },
                    { name: "Data Select", lsb: 9, bits: 4 },
                    { name: "Data Scale", lsb: 13, bits: 2 },
                    { name: "PME Status", lsb: 15, bits: 1 },
                ]
            },
            {
                name: "Power Management Data",
                offset: 0x07,
                id: POWER_MANAGEMENT_DATA,
                size: Byte,
                fields: [
                    { name: "Data", lsb: 0, bits: 8 },
                ]
            },
        ]
    }
}

fn pm_summary(cap: &PciCapa) -> Option<Vec<TreeNode>> {
    let pmcsr = cap.read_u16(u64::from(POWER_MANAGEMENT_CTRL_STAT)).ok()?;
    let power_state = match pmcsr & 0x3 {
        0 => "D0",
        1 => "D1",
        2 => "D2",
        3 => "D3hot",
        _ => "Unknown",
    };

    let aux_ma = cap
        .read_u16(u64::from(POWER_MANAGEMENT_CAPS))
        .ok()
        .map(|pmc| (pmc >> 6) & 0x7)
        .and_then(|aux| match aux {
            0 => None,
            1 => Some(55),
            2 => Some(100),
            3 => Some(160),
            4 => Some(220),
            5 => Some(270),
            6 => Some(320),
            7 => Some(375),
            _ => None,
        });

    let summary = if let Some(aux_ma) = aux_ma {
        format!("{power_state} State, {aux_ma} mA Aux Power")
    } else {
        format!("{power_state} State")
    };

    let mut root =
        TreeNode::with_value_collapsed(TreeLine::from("Power Management"), TreeLine::from(summary));

    const DATA_ENTRIES: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];

    let mut consumed: [Option<String>; 4] = [None, None, None, None];
    let mut dissipated: [Option<String>; 4] = [None, None, None, None];
    let mut common_logic = None::<String>;

    let orig_select = (pmcsr >> 9) & 0x0f;
    let base = (pmcsr & !0x1e00) & !0x8000;
    let mut wrote = false;

    for select in DATA_ENTRIES {
        let write_val = base | (select << 9);
        if cap
            .write_u16(u64::from(POWER_MANAGEMENT_CTRL_STAT), write_val)
            .is_err()
        {
            break;
        }
        wrote = true;

        let sel_pmcsr = match cap.read_u16(u64::from(POWER_MANAGEMENT_CTRL_STAT)) {
            Ok(val) => val,
            Err(_) => break,
        };
        let data_scale = (sel_pmcsr >> 13) & 0x03;
        let data = match cap.read_u8(u64::from(POWER_MANAGEMENT_DATA)) {
            Ok(val) => val,
            Err(_) => continue,
        };
        let (scale, decimals) = match data_scale {
            1 => (0.1, 1),
            2 => (0.01, 2),
            3 => (0.001, 3),
            _ => (0.0, 0),
        };
        if scale > 0.0 {
            let value = format!("{:.*}W", decimals, data as f64 * scale);
            match select {
                0..=3 => consumed[*select as usize] = Some(value),
                4..=7 => dissipated[*select as usize - 4] = Some(value),
                8 => common_logic = Some(value),
                _ => {}
            }
        }
    }

    if wrote {
        let _ = cap.write_u16(
            u64::from(POWER_MANAGEMENT_CTRL_STAT),
            base | (orig_select << 9),
        );
    }

    let states = ["D0", "D1", "D2", "D3"];
    for (idx, state) in states.iter().enumerate() {
        let mut parts = Vec::new();
        if let Some(value) = consumed[idx].as_deref() {
            parts.push(format!("{value} consumed"));
        }
        if let Some(value) = dissipated[idx].as_deref() {
            parts.push(format!("{value} dissipated"));
        }
        if !parts.is_empty() {
            root.add_child(TreeNode::with_value(
                TreeLine::from(format!("{state} Power")),
                TreeLine::from(parts.join(", ")),
            ));
        }
    }

    if let Some(value) = common_logic.as_deref() {
        root.add_child(TreeNode::with_value(
            TreeLine::from("Common Logic Power"),
            TreeLine::from(format!("{value} consumed")),
        ));
    }

    Some(vec![root])
}
