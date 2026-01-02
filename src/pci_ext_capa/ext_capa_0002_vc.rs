use crate::capabilities;
use crate::pci_device::PciCapa;

fn vc_arb_entries(select: u8) -> Option<u32> {
    match select {
        1 => Some(32),
        2 => Some(64),
        3 => Some(128),
        _ => None,
    }
}

fn port_arb_entries(select: u8) -> Option<u32> {
    match select {
        1 => Some(32),
        2 => Some(64),
        3 => Some(128),
        4 => Some(128),
        5 => Some(256),
        _ => None,
    }
}

fn port_arb_entry_bits(bits: u8) -> u32 {
    match bits {
        0 => 1,
        1 => 2,
        2 => 4,
        _ => 8,
    }
}

pub(crate) fn get_size(capa: &PciCapa) -> Option<u16> {
    let port_cap1 = capa.read_u32(u64::from(VC_PORT_VC_CAP1)).ok()?;
    let ext_vc_count = (port_cap1 & 0x7) as u16;
    let entry_bits = port_arb_entry_bits(((port_cap1 >> 10) & 0x3) as u8);

    let mut size = 0x10u32 + (ext_vc_count as u32 + 1) * 0x0c;

    let port_cap2 = capa.read_u32(u64::from(VC_PORT_VC_CAP2)).ok()?;
    let vc_table_offset = ((port_cap2 >> 24) & 0xff) as u32;
    if vc_table_offset > 0 {
        let port_ctrl = capa.read_u16(u64::from(VC_PORT_CTRL)).ok()?;
        let vc_select = ((port_ctrl >> 1) & 0x7) as u8;
        if let Some(entries) = vc_arb_entries(vc_select) {
            let table_len = entries / 2;
            let end = vc_table_offset * 16 + table_len;
            size = size.max(end);
        }
    }

    for vc in 0..=ext_vc_count {
        let cap_off = 0x10u64 + vc as u64 * 0x0c;
        let res_cap = capa.read_u32(cap_off).ok()?;
        let table_offset = ((res_cap >> 24) & 0xff) as u32;
        if table_offset == 0 {
            continue;
        }
        let res_ctrl = capa.read_u32(cap_off + 0x04).ok()?;
        let port_select = ((res_ctrl >> 17) & 0x7) as u8;
        if let Some(entries) = port_arb_entries(port_select) {
            let table_len = (entries * entry_bits) / 8;
            let end = table_offset * 16 + table_len;
            size = size.max(end);
        }
    }

    u16::try_from(size).ok()
}

capabilities! {
    {
        id: 0x0002,
        version: 1,
        is_extended: true,
        name: "Virtual Channel",
        get_size: get_size,
        registers: [
            {
                name: "VC Port VC Capability 1",
                offset: 0x04,
                id: VC_PORT_VC_CAP1,
                size: Dword,
                fields: [
                    { name: "Extended VC count", lsb: 0, bits: 3 },
                    { name: "Low priority extended VC count", lsb: 4, bits: 3 },
                    {
                        name: "Arbitration table size",
                        lsb: 10,
                        bits: 2,
                        enum_values: [
                            (0x0, "32 phases"),
                            (0x1, "64 phases"),
                            (0x2, "128 phases"),
                        ]
                    },
                ]
            },
            {
                name: "VC Port VC Capability 2",
                offset: 0x08,
                id: VC_PORT_VC_CAP2,
                size: Dword,
                fields: [
                    { name: "32-phase arbitration", lsb: 1, bits: 1 },
                    { name: "64-phase arbitration", lsb: 2, bits: 1 },
                    { name: "128-phase arbitration", lsb: 3, bits: 1 },
                    { name: "Arbitration table offset", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "VC Port Control",
                offset: 0x0c,
                id: VC_PORT_CTRL,
                size: Dword,
                fields: [
                    { name: "Load VC Arbitration Table", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "VC Port Status",
                offset: 0x0e,
                id: VC_PORT_STAT,
                size: Word,
                fields: [
                    { name: "Table completion status", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "VC Resource Capability",
                offset: 0x10,
                id: VC_RESOURCE_CAP,
                size: Dword,
                fields: [
                    { name: "32-phase arbitration", lsb: 1, bits: 1 },
                    { name: "64-phase arbitration", lsb: 2, bits: 1 },
                    { name: "128-phase arbitration", lsb: 3, bits: 1 },
                    { name: "128-phase arbitration table", lsb: 4, bits: 1 },
                    { name: "256-phase arbitration", lsb: 5, bits: 1 },
                    { name: "Arbitration table offset", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "VC Resource Control",
                offset: 0x14,
                id: VC_RESOURCE_CTRL,
                size: Dword,
                fields: [
                    { name: "Load VC Arbitration Table", lsb: 16, bits: 1 },
                    {
                        name: "Arbitration Select",
                        lsb: 17,
                        bits: 3,
                        enum_values: [
                            (0x0, "Hardware fixed priority"),
                            (0x1, "Round robin"),
                            (0x2, "Weighted round robin"),
                            (0x3, "Weighted round robin (VC resource)"),
                        ]
                    },
                    { name: "VC ID", lsb: 24, bits: 3 },
                    { name: "TC/VC Map Enable", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "VC Resource Status",
                offset: 0x1a,
                id: VC_RESOURCE_STAT,
                size: Word,
                fields: [
                    { name: "Table completion status", lsb: 0, bits: 1 },
                    { name: "Negotiation Pending", lsb: 1, bits: 1 },
                ]
            },
        ]
    }
}
