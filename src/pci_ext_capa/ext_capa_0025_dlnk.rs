use crate::capabilities;

const L0P_EXIT_LATENCY: &[(u64, &str)] = &[
    (0x0, "Less than 1 us"),
    (0x1, "Less than 2 us"),
    (0x2, "Less than 4 us"),
    (0x3, "Less than 8 us"),
    (0x4, "Less than 16 us"),
    (0x5, "Less than 32 us"),
    (0x6, "Less than 64 us"),
    (0x7, "More than 64 us"),
];

const TLPS_PER_HALF_FLIT: &[(u64, &str)] = &[
    (0x0, "8 TLPs/Half Flit"),
    (0x1, "4 TLPs/Half Flit (>=128GT/s) else 8"),
    (0x2, "Reserved (treat as 4/8)"),
    (0x3, "Reserved (treat as 4/8)"),
];

capabilities! {
    {
        id: 0x0025,
        version: 1,
        is_extended: true,
        name: "Data Link Feature",
        size: 16,
        registers: [
            {
                name: "Data Link Feature Capabilities",
                offset: 0x04,
                id: DATA_LINK_FEATURE_CAPS,
                size: Dword,
                fields: [
                    { name: "Local Scaled Flow Control Supported", lsb: 0, bits: 1 },
                    { name: "Local Immediate Readiness", lsb: 1, bits: 1 },
                    { name: "Local Extended VC Count", lsb: 2, bits: 3 },
                    { name: "Local L0p Exit Latency", lsb: 5, bits: 3, enum_values: L0P_EXIT_LATENCY },
                    { name: "Local TLPs per Half Flit", lsb: 8, bits: 2, enum_values: TLPS_PER_HALF_FLIT },
                    { name: "Data Link Feature Exchange is Enabled", lsb: 31, bits: 1 },
                ]
            },
            {
                name: "Data Link Feature Status",
                offset: 0x08,
                id: DATA_LINK_FEATURE_STAT,
                size: Dword,
                fields: [
                    { name: "Remote Scaled Flow Control Supported", lsb: 0, bits: 1 },
                    { name: "Remote Immediate Readiness", lsb: 1, bits: 1 },
                    { name: "Remote Extended VC Count", lsb: 2, bits: 3 },
                    { name: "Remote L0p Exit Latency", lsb: 5, bits: 3, enum_values: L0P_EXIT_LATENCY },
                    { name: "Remote TLPs per Half Flit", lsb: 8, bits: 2, enum_values: TLPS_PER_HALF_FLIT },
                    { name: "Remote Data Link Feature Supported Valid", lsb: 31, bits: 1 },
                ]
            },
        ]
    }
}
