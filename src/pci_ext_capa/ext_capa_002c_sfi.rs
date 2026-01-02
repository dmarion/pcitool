use crate::capabilities;

capabilities! {
    {
        id: 0x002c,
        version: 1,
        is_extended: true,
        name: "Scalable Fabric Interface",
        size: 16,
        registers: [
            {
                name: "SFI Capability",
                offset: 0x04,
                id: SFI_CAP,
                size: Word,
                fields: [
                    { name: "SFI OOB PD Supported", lsb: 0, bits: 1 },
                    { name: "Enhanced SFI Supported", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "SFI Control",
                offset: 0x06,
                id: SFI_CTRL,
                size: Word,
                fields: [
                    { name: "SFI PD State Mask", lsb: 0, bits: 1 },
                    { name: "SFI DLL State Mask", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "SFI Status",
                offset: 0x08,
                id: SFI_STAT,
                size: Dword,
                fields: [
                    { name: "SFI PD State", lsb: 0, bits: 1 },
                    { name: "SFI OOB PD State", lsb: 1, bits: 1 },
                ]
            },
            { name: "SFI CAM Address", offset: 0x0c, id: SFI_CAM_ADDR, size: Dword, fields: [] },
            { name: "SFI CAM Data", offset: 0x10, id: SFI_CAM_DATA, size: Dword, fields: [] },
        ]
    }
}
