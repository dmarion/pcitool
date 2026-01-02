use crate::capabilities;

capabilities! {
    {
        id: 0x0007,
        version: 1,
        is_extended: true,
        name: "Root Complex Event Collector Endpoint Association",
        size: 16,
        registers: [
            {
                name: "Association Bitmap for RCiEPs",
                offset: 0x04,
                id: ASSOCIATION_BITMAP_FOR_RCIEPS,
                size: Dword,
                fields: []
            },
            {
                name: "RCEC Associated Bus Numbers",
                offset: 0x08,
                id: RCEC_ASSOCIATED_BUS_NUMBERS,
                size: Dword,
                fields: [
                    { name: "Next Bus Number", lsb: 0, bits: 8 },
                    { name: "Last Bus Number", lsb: 8, bits: 8 },
                ]
            },
        ]
    }
}
