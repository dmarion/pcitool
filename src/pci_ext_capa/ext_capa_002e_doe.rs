use crate::capabilities;

capabilities! {
    {
        id: 0x002e,
        version: 1,
        is_extended: true,
        name: "Data Object Exchange",
        size: 24,
        registers: [
            {
                name: "DOE Capabilities",
                offset: 0x04,
                id: DOE_CAPS,
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
                id: DOE_CTRL,
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
                id: DOE_STAT,
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
                id: DOE_WRITE_DATA_MAILBOX,
                size: Dword,
                fields: []
            },
            {
                name: "DOE Read Data Mailbox",
                offset: 0x14,
                id: DOE_READ_DATA_MAILBOX,
                size: Dword,
                fields: []
            },
        ]
    }
}
