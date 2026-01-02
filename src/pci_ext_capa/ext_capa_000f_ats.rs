use crate::capabilities;

capabilities! {
    {
        id: 0x000f,
        version: 1,
        is_extended: true,
        name: "Address Translation Service",
        size: 8,
        registers: [
            {
                name: "ATS Capability",
                offset: 0x04,
                id: ATS_CAP,
                size: Word,
                fields: [
                    { name: "Invalidate Queue Depth", lsb: 0, bits: 5 },
                    { name: "Page Aligned Request", lsb: 5, bits: 1 },
                ]
            },
            {
                name: "ATS Control",
                offset: 0x06,
                id: ATS_CTRL,
                size: Word,
                fields: [
                    { name: "Smallest Translation Unit", lsb: 0, bits: 5 },
                    { name: "ATS Enable", lsb: 15, bits: 1 },
                ]
            },
        ]
    }
}
