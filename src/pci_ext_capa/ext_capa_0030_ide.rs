use crate::capabilities;
use crate::pci_device::PciCapa;

fn get_size(capa: &PciCapa) -> Option<u16> {
    let caps = capa.read_u32(u64::from(IDE_CAP)).ok()?;
    let link_supported = (caps & 0x1) != 0;
    let selective_supported = (caps & 0x2) != 0;
    let link_tc = ((caps >> 13) & 0x7) as u32;
    let link_streams = if link_supported { link_tc + 1 } else { 0 };

    let selective_streams = if selective_supported {
        ((caps >> 16) & 0xff) as u32 + 1
    } else {
        0
    };

    let mut offset = 0x0c + link_streams * 0x08;
    for _ in 0..selective_streams {
        let stream_caps = capa.read_u32(offset as u64).ok()?;
        let addr_blocks = (stream_caps & 0x0f) as u32;
        offset += 0x14 + addr_blocks * 0x0c;
    }

    u16::try_from(offset).ok()
}

capabilities! {
    {
        id: 0x0030,
        version: 1,
        is_extended: true,
        name: "Integrity and Data Encryption",
        get_size: get_size,
        registers: [
            {
                name: "IDE Capability",
                offset: 0x04,
                id: IDE_CAP,
                size: Dword,
                fields: [
                    { name: "Link IDE Supported", lsb: 0, bits: 1 },
                    { name: "Selective IDE Supported", lsb: 1, bits: 1 },
                    { name: "Flow Through IDE Supported", lsb: 2, bits: 1 },
                    { name: "Partial Header Encryption Supported", lsb: 3, bits: 1 },
                    { name: "Agile IDE Supported", lsb: 4, bits: 1 },
                    { name: "Max Number of Link IDE Streams", lsb: 8, bits: 3 },
                    { name: "Max Number of Selective IDE Streams", lsb: 11, bits: 8 },
                ]
            },
            {
                name: "IDE Control",
                offset: 0x08,
                id: IDE_CTRL,
                size: Dword,
                fields: [
                    { name: "Flow Through IDE Enable", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
