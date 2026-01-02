use crate::capabilities;

capabilities! {
    {
        id: 0x01,
        name: "Power Management",
        size: 8,
        registers: [
            {
                name: "Power Management Capabilities",
                offset: 0x02,
                id: POWER_MANAGEMENT_CAPS,
                size: Word,
                fields: [
                    { name: "Version", lsb: 0, bits: 3 },
                    { name: "PME clock required", lsb: 3, bits: 1 },
                    { name: "Device specific initialization", lsb: 5, bits: 1 },
                    {
                        name: "Auxiliary power support",
                        lsb: 6,
                        bits: 3,
                        enum_values: [
                            (0x0, "0mA"),
                            (0x1, "55mA"),
                            (0x2, "100mA"),
                            (0x3, "160mA"),
                            (0x4, "220mA"),
                            (0x5, "270mA"),
                            (0x6, "320mA"),
                            (0x7, "375mA"),
                        ]
                    },
                    { name: "D1 power state support", lsb: 9, bits: 1 },
                    { name: "D2 power state support", lsb: 10, bits: 1 },
                    { name: "PME Support", lsb: 11, bits: 5 },
                ]
            },
            {
                name: "Power Management Control/Status",
                offset: 0x04,
                id: POWER_MANAGEMENT_CTRL_STAT,
                size: Word,
                fields: [
                    {
                        name: "Power State",
                        lsb: 0,
                        bits: 2,
                        enum_values: [
                            (0x0, "D0"),
                            (0x1, "D1"),
                            (0x2, "D2"),
                            (0x3, "D3hot"),
                        ]
                    },
                    { name: "No soft reset", lsb: 3, bits: 1 },
                    { name: "PME Enable", lsb: 8, bits: 1 },
                    { name: "Data Select", lsb: 9, bits: 4 },
                    { name: "Data Scale", lsb: 13, bits: 2 },
                    { name: "PME Status", lsb: 15, bits: 1 },
                ]
            },
        ]
    }
}
