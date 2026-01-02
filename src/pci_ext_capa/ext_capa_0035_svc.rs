use crate::capabilities;
use crate::pci_device::PciCapa;

fn get_size(capa: &PciCapa) -> Option<u16> {
    let cap1 = capa.read_u32(u64::from(SVC_PORT_CAP1)).ok()?;
    let ext_vc_count = (cap1 & 0x7) as u16;
    let size = 0x14u32 + (ext_vc_count as u32 + 1) * 0x0c;
    u16::try_from(size).ok()
}

capabilities! {
    {
        id: 0x0035,
        version: 1,
        is_extended: true,
        name: "Streamlined Virtual Channel",
        get_size: get_size,
        registers: [
            {
                name: "SVC Port Capability 1",
                offset: 0x04,
                id: SVC_PORT_CAP1,
                size: Dword,
                fields: []
            },
            {
                name: "SVC Port Control",
                offset: 0x0c,
                id: SVC_PORT_CTRL,
                size: Word,
                fields: []
            },
            {
                name: "SVC Port Status",
                offset: 0x10,
                id: SVC_PORT_STAT,
                size: Word,
                fields: []
            },
        ]
    }
}
