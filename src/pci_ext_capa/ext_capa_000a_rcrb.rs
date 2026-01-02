use crate::capabilities;

capabilities! {
    {
        id: 0x000a,
        version: 1,
        is_extended: true,
        name: "RCRB Header",
        size: 16,
        registers: [
            {
                name: "RCRB Vendor ID and Device ID",
                offset: 0x04,
                id: RCRB_VENDOR_ID_AND_DEVICE_ID,
                size: Dword,
                fields: [
                    { name: "Vendor ID", lsb: 0, bits: 16 },
                    { name: "Device ID", lsb: 16, bits: 16 },
                ]
            },
            {
                name: "RCRB Capabilities",
                offset: 0x08,
                id: RCRB_CAPS,
                size: Dword,
                fields: []
            },
            {
                name: "RCRB Control",
                offset: 0x0c,
                id: RCRB_CTRL,
                size: Dword,
                fields: []
            },
        ]
    }
}
