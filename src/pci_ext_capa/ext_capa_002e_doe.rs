use crate::capabilities;

capabilities! {
    ext {
        id: 0x002e,
        version: 1,
        name: "Data Object Exchange",
        registers: [
            {
                name: "DOE Capabilities",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "DOE Interrupt Support", lsb: 0, bits: 1 },
                    { name: "DOE Interrupt Message Number", lsb: 1, bits: 11 },
                    { name: "DOE Attention Mechanism Support", lsb: 12, bits: 1 },
                ]
            },
            {
                name: "DOE Control",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "DOE Abort", lsb: 0, bits: 1 },
                    { name: "DOE Interrupt Enable", lsb: 1, bits: 1 },
                    { name: "DOE Attention Not Needed", lsb: 2, bits: 1 },
                ]
            },
            {
                name: "DOE Status",
                offset: 0x0c,
                size: Dword,
                fields: [
                    { name: "DOE Busy", lsb: 0, bits: 1 },
                    { name: "DOE Interrupt Status", lsb: 1, bits: 1 },
                    { name: "DOE Error", lsb: 2, bits: 1 },
                    { name: "DOE Data Object Ready", lsb: 3, bits: 1 },
                ]
            },
            {
                name: "DOE Write Data Mailbox",
                offset: 0x10,
                size: Dword,
                fields: []
            },
            {
                name: "DOE Read Data Mailbox",
                offset: 0x14,
                size: Dword,
                fields: []
            },
        ]
    }
}
