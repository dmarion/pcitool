use crate::capabilities;

capabilities! {
    ext {
        id: 0x002c,
        version: 1,
        name: "Scalable Fabric Interface",
        registers: [
            {
                name: "SFI Capability",
                offset: 0x04,
                size: Word,
                fields: [
                    { name: "SFI OOB PD Supported", lsb: 0, bits: 1 },
                    { name: "Enhanced SFI Supported", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "SFI Control",
                offset: 0x06,
                size: Word,
                fields: [
                    { name: "SFI PD State Mask", lsb: 0, bits: 1 },
                    { name: "SFI DLL State Mask", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "SFI Status",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "SFI PD State", lsb: 0, bits: 1 },
                    { name: "SFI OOB PD State", lsb: 1, bits: 1 },
                ]
            },
            { name: "SFI CAM Address", offset: 0x0c, size: Dword, fields: [] },
            { name: "SFI CAM Data", offset: 0x10, size: Dword, fields: [] },
        ]
    }
}
