use crate::capabilities;

capabilities! {
    {
        id: 0x002f,
        version: 1,
        is_extended: true,
        name: "Device 3",
        size: 16,
        registers: [
            {
                name: "Device Capabilities 3",
                offset: 0x04,
                id: DEVICE_CAPS3,
                size: Dword,
                fields: [
                    { name: "DMWr Request Routing Supported", lsb: 0, bits: 1 },
                    { name: "14-Bit Tag Completer Supported", lsb: 1, bits: 1 },
                    { name: "14-Bit Tag Requester Supported", lsb: 2, bits: 1 },
                    { name: "L0p Supported", lsb: 3, bits: 1 },
                    { name: "Port L0p Exit Latency", lsb: 4, bits: 3 },
                    { name: "Retimer L0p Exit Latency", lsb: 7, bits: 3 },
                    { name: "UIO Mem RdWr Completer Supported", lsb: 10, bits: 1 },
                    { name: "UIO Mem RdWr Requester Supported", lsb: 11, bits: 1 },
                    { name: "OHC-E Support", lsb: 12, bits: 3 },
                ]
            },
            {
                name: "Device Control 3",
                offset: 0x08,
                id: DEVICE_CTRL3,
                size: Dword,
                fields: [
                    { name: "DMWr Requester Enable", lsb: 0, bits: 1 },
                    { name: "DMWr Egress Blocking", lsb: 1, bits: 1 },
                    { name: "14-Bit Tag Requester Enable", lsb: 2, bits: 1 },
                    { name: "L0p Enable", lsb: 3, bits: 1 },
                    { name: "Target Link Width", lsb: 4, bits: 4 },
                    { name: "UIO Mem RdWr Requester Enable", lsb: 8, bits: 1 },
                    { name: "UIO Request 256B Boundary Disable", lsb: 9, bits: 1 },
                ]
            },
            {
                name: "Device Status 3",
                offset: 0x0c,
                id: DEVICE_STAT3,
                size: Dword,
                fields: [
                    { name: "Remote L0p Supported", lsb: 0, bits: 3 },
                    { name: "Segment Captured", lsb: 3, bits: 1 },
                    { name: "Initial Link Width", lsb: 4, bits: 1 },
                ]
            },
        ]
    }
}
