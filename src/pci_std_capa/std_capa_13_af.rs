use crate::capabilities;

capabilities! {
    {
        id: 0x13,
        name: "Advanced Features",
        size: 8,
        registers: [
            {
                name: "AF Capabilities",
                offset: 0x03,
                id: AF_CAPS,
                size: Byte,
                fields: [
                    { name: "Transactions Pending Supported", lsb: 0, bits: 1 },
                    { name: "Function Level Reset Supported", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "AF Control",
                offset: 0x04,
                id: AF_CTRL,
                size: Byte,
                fields: [
                    { name: "Initiate Function Level Reset", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "AF Status",
                offset: 0x05,
                id: AF_STAT,
                size: Byte,
                fields: [
                    { name: "Transactions Pending", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
