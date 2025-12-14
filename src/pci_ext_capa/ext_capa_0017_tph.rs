use crate::capabilities;

capabilities! {
    ext {
        id: 0x0017,
        version: 1,
        name: "Transaction Processing Hints",
        registers: [
            {
                name: "TPH Capabilities",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "No ST Mode Supported", lsb: 0, bits: 1 },
                    { name: "Interrupt Vector Mode Supported", lsb: 1, bits: 1 },
                    { name: "Device Specific Mode Supported", lsb: 2, bits: 1 },
                    { name: "Extended TPH Requester Supported", lsb: 8, bits: 1 },
                    {
                        name: "ST Table Location",
                        lsb: 9,
                        bits: 2,
                        enum_values: [
                            (0x0, "None"),
                            (0x1, "TPH Requestor Capability"),
                            (0x2, "MSI-X Table"),
                        ]
                    },
                    { name: "ST Table Size", lsb: 16, bits: 11 },
                ]
            },
            {
                name: "TPH Control",
                offset: 0x08,
                size: Dword,
                fields: [
                    {
                        name: "ST Mode Select",
                        lsb: 0,
                        bits: 3,
                        enum_values: [
                            (0x0, "No ST Mode"),
                            (0x1, "Interrupt Vector Mode"),
                            (0x2, "Device Specific Mode"),
                        ]
                    },
                    { name: "TPH Requester Enable", lsb: 8, bits: 2 },
                ]
            },
        ]
    }
}
