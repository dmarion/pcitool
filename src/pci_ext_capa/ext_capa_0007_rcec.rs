use crate::capabilities;

capabilities! {
    ext {
        id: 0x0007,
        version: 1,
        name: "Root Complex Event Collector Endpoint Association",
        registers: [
            {
                name: "Association Bitmap for RCiEPs",
                offset: 0x04,
                size: Dword,
                fields: []
            },
            {
                name: "RCEC Associated Bus Numbers",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "Next Bus Number", lsb: 0, bits: 8 },
                    { name: "Last Bus Number", lsb: 8, bits: 8 },
                ]
            },
        ]
    }
}
