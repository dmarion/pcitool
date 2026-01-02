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

fn fn_arb_entries(select: u8) -> Option<u32> {
    match select {
        1 => Some(32),
        2 => Some(64),
        3 => Some(128),
        4 => Some(128),
        5 => Some(256),
        _ => None,
    }
}

fn fn_arb_entry_bits(bits: u8) -> u32 {
    match bits {
        0 => 1,
        1 => 2,
        2 => 4,
        _ => 8,
    }
}

fn get_size(capa: &PciCapa) -> Option<u16> {
    let port_cap1 = capa.read_u32(u64::from(PORT_VC_CAP1)).ok()?;
    let ext_vc_count = (port_cap1 & 0x7) as u16;
    let entry_bits = fn_arb_entry_bits(((port_cap1 >> 10) & 0x3) as u8);

    let mut size = 0x10u32 + (ext_vc_count as u32 + 1) * 0x0c;

    let port_cap2 = capa.read_u32(u64::from(PORT_VC_CAP2)).ok()?;
    let vc_table_offset = ((port_cap2 >> 24) & 0xff) as u32;
    if vc_table_offset > 0 {
        let port_ctrl = capa.read_u16(u64::from(PORT_VC_CTRL)).ok()?;
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
        let fn_select = ((res_ctrl >> 17) & 0x7) as u8;
        if let Some(entries) = fn_arb_entries(fn_select) {
            let table_len = (entries * entry_bits) / 8;
            let end = table_offset * 16 + table_len;
            size = size.max(end);
        }
    }

    u16::try_from(size).ok()
}

capabilities! {
    {
        id: 0x0008,
        version: 1,
        is_extended: true,
        name: "Multi-Function Virtual Channel",
        get_size: get_size,
        registers: [
            {
                name: "Port VC Capability 1",
                offset: 0x04,
                id: PORT_VC_CAP1,
                size: Dword,
                fields: [
                    { name: "Extended VC Count", lsb: 0, bits: 3 },
                    { name: "Port Arbitration Capability", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "Port VC Capability 2",
                offset: 0x08,
                id: PORT_VC_CAP2,
                size: Dword,
                fields: [
                    { name: "VC Arbitration Capability", lsb: 0, bits: 8 },
                    { name: "VC Arbitration Table Offset", lsb: 24, bits: 8 },
                ]
            },
            {
                name: "Port VC Control",
                offset: 0x0c,
                id: PORT_VC_CTRL,
                size: Word,
                fields: [
                    { name: "VC Arbitration Select", lsb: 0, bits: 3 },
                ]
            },
            {
                name: "Port VC Status",
                offset: 0x0e,
                id: PORT_VC_STAT,
                size: Word,
                fields: [
                    { name: "VC Arbitration Table Status", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
