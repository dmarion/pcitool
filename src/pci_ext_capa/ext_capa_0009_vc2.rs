use crate::capabilities;
use crate::pci_ext_capa::ext_capa_0002_vc::get_size as get_vc_size;

capabilities! {
    {
        id: 0x0009,
        version: 1,
        is_extended: true,
        name: "Virtual Channel (Secondary)",
        get_size: get_vc_size,
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
