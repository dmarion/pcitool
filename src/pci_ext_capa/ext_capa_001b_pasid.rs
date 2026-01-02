use crate::capabilities;

capabilities! {
    {
        id: 0x001b,
        version: 1,
        is_extended: true,
        name: "Process Address Space ID",
        size: 12,
        registers: [
            {
                name: "PASID Capability",
                offset: 0x04,
                id: PASID_CAP,
                size: Word,
                fields: [
                    { name: "Execute Permissions Supported", lsb: 1, bits: 1 },
                    { name: "Privilege Mode Supported", lsb: 2, bits: 1 },
                    { name: "Translated Requests with PASID Supported", lsb: 3, bits: 1 },
                    { name: "Max PASID Width", lsb: 8, bits: 5 },
                ]
            },
            {
                name: "PASID Control",
                offset: 0x06,
                id: PASID_CTRL,
                size: Word,
                fields: [
                    { name: "PASID Enable", lsb: 0, bits: 1 },
                    { name: "Execute Permissions Enable", lsb: 1, bits: 1 },
                    { name: "Privilege Mode Enable", lsb: 2, bits: 1 },
                    { name: "Translated Requests with PASID Enable", lsb: 3, bits: 1 },
                ]
            },
        ]
    }
}
