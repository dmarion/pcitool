use crate::capabilities;

capabilities! {
    {
        id: 0x001f,
        version: 1,
        is_extended: true,
        name: "Precision Time Measurement",
        size: 12,
        registers: [
            {
                name: "PTM Capability",
                offset: 0x04,
                id: PTM_CAP,
                size: Dword,
                fields: [
                    { name: "Requester capable", lsb: 0, bits: 1 },
                    { name: "Responder capable", lsb: 1, bits: 1 },
                    { name: "Root capable", lsb: 2, bits: 1 },
                    { name: "ePTM Capable", lsb: 3, bits: 1 },
                    { name: "PTM Propagation Delay Adaptation Capable", lsb: 4, bits: 1 },
                    { name: "Clock granularity", lsb: 8, bits: 8 },
                ]
            },
            {
                name: "PTM Control",
                offset: 0x08,
                id: PTM_CTRL,
                size: Dword,
                fields: [
                    { name: "PTM enable", lsb: 0, bits: 1 },
                    { name: "Root select", lsb: 1, bits: 1 },
                    { name: "Effective Granularity", lsb: 8, bits: 8 },
                ]
            },
        ]
    }
}
