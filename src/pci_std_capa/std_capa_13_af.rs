use crate::capabilities;

capabilities! {
    std {
        id: 0x13,
        name: "Advanced Features",
        registers: [
            {
                name: "AF Capabilities",
                offset: 0x03,
                size: Byte,
                fields: [
                    { name: "Transactions Pending Supported", lsb: 0, bits: 1 },
                    { name: "Function Level Reset Supported", lsb: 1, bits: 1 },
                ]
            },
            {
                name: "AF Control",
                offset: 0x04,
                size: Byte,
                fields: [
                    { name: "Initiate Function Level Reset", lsb: 0, bits: 1 },
                ]
            },
            {
                name: "AF Status",
                offset: 0x05,
                size: Byte,
                fields: [
                    { name: "Transactions Pending", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
