use crate::capabilities;

capabilities! {
    ext {
        id: 0x0018,
        version: 1,
        name: "Latency Tolerance Reporting",
        registers: [
            {
                name: "Max Snoop Latency",
                offset: 0x04,
                size: Word,
                fields: [
                    { name: "Snoop Latency Value", lsb: 0, bits: 10 },
                    {
                        name: "Snoop Latency Scale",
                        lsb: 10,
                        bits: 3,
                        enum_values: [
                            (0x0, "1ns"),
                            (0x1, "32ns"),
                            (0x2, "1024ns"),
                            (0x3, "32768ns"),
                            (0x4, "1048576ns"),
                            (0x5, "33554432ns"),
                        ]
                    },
                ]
            },
            {
                name: "Max No Snoop Latency",
                offset: 0x06,
                size: Word,
                fields: [
                    { name: "No Snoop Latency Value", lsb: 0, bits: 10 },
                    {
                        name: "No Snoop Latency Scale",
                        lsb: 10,
                        bits: 3,
                        enum_values: [
                            (0x0, "1ns"),
                            (0x1, "32ns"),
                            (0x2, "1024ns"),
                            (0x3, "32768ns"),
                            (0x4, "1048576ns"),
                            (0x5, "33554432ns"),
                        ]
                    },
                ]
            },
        ]
    }
}
