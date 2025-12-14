use crate::capabilities;

capabilities! {
    ext {
        id: 0x0022,
        version: 1,
        name: "Readiness Time Reporting",
        registers: [
            {
                name: "Readiness Time Reporting 1",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Reset Time", lsb: 0, bits: 12 },
                    { name: "DL_Up Time", lsb: 12, bits: 12 },
                    { name: "Valid", lsb: 30, bits: 1 },
                    { name: "Immediate Readiness", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "Readiness Time Reporting 2",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "FLR Time", lsb: 0, bits: 12 },
                    { name: "D3Hot to D0 Time", lsb: 12, bits: 12 },
                ]
            },
        ]
    }
}
