use crate::capabilities;

capabilities! {
    ext {
        id: 0x001d,
        version: 1,
        name: "Downstream Port Containment",
        registers: [
            {
                name: "DPC Capability",
                offset: 0x04,
                size: Word,
                fields: [
                    { name: "DPC Interrupt Message Number", lsb: 0, bits: 5 },
                    { name: "RP Extensions for DPC", lsb: 5, bits: 1 },
                    { name: "Poisoned TLP Egress Blocking Supported", lsb: 6, bits: 1 },
                    { name: "DPC Software Triggering Supported", lsb: 7, bits: 1 },
                    { name: "RP PIO Log Size[3:0]", lsb: 8, bits: 4 },
                    { name: "DL_Active ERR_COR Signaling Supported", lsb: 12, bits: 1 },
                    { name: "RP PIO Log Size[4]", lsb: 13, bits: 1 },
                ]
            },
            {
                name: "DPC Control",
                offset: 0x06,
                size: Word,
                fields: [
                    {
                        name: "DPC Trigger Enable",
                        lsb: 0,
                        bits: 2,
                        enum_values: [
                            (0x0, "Disabled"),
                            (0x1, "DPC on fatal"),
                            (0x2, "DPC on fatal/non-fatal"),
                        ]
                    },
                    { name: "DPC Completion Control", lsb: 2, bits: 1 },
                    { name: "DPC Interrupt Enable", lsb: 3, bits: 1 },
                    { name: "DPC ERR_COR Enable", lsb: 4, bits: 1 },
                    { name: "Poisoned TLP Egress Blocking Enable", lsb: 5, bits: 1 },
                    { name: "DPC Software Trigger", lsb: 6, bits: 1 },
                    { name: "DL_Active ERR_COR Enable", lsb: 7, bits: 1 },
                    { name: "DPC SIG_SFW Enable", lsb: 8, bits: 1 },
                ]
            },
            {
                name: "DPC Status",
                offset: 0x08,
                size: Word,
                fields: [
                    { name: "DPC Trigger Status", lsb: 0, bits: 1 },
                    {
                        name: "DPC Trigger Reason",
                        lsb: 1,
                        bits: 2,
                        enum_values: [
                            (0x0, "Uncorrectable Error"),
                            (0x1, "RP PIO Error"),
                            (0x2, "Software Trigger"),
                        ]
                    },
                    { name: "DPC Interrupt Status", lsb: 3, bits: 1 },
                    { name: "DPC RP Busy", lsb: 4, bits: 1 },
                    { name: "DPC Trigger Reason Extension", lsb: 5, bits: 2 },
                    { name: "RP PIO First Error Pointer", lsb: 8, bits: 5 },
                    { name: "DPC SIG_SFW Status", lsb: 13, bits: 1 },
                ]
            },
            {
                name: "DPC Error Source ID",
                offset: 0x0a,
                size: Word,
                fields: [
                    { name: "DPC Error Source ID", lsb: 0, bits: 16 },
                ]
            },
            {
                name: "RP PIO Status",
                offset: 0x0c,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO Mask",
                offset: 0x10,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO Severity",
                offset: 0x14,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO SysError",
                offset: 0x18,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO Exception",
                offset: 0x1c,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO Header Log 0",
                offset: 0x20,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO Header Log 1",
                offset: 0x24,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO Header Log 2",
                offset: 0x28,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO Header Log 3",
                offset: 0x2c,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO ImpSpec Log",
                offset: 0x30,
                size: Dword,
                fields: []
            },
            {
                name: "RP PIO TLP Prefix Log",
                offset: 0x34,
                size: Dword,
                fields: []
            },
        ]
    }
}
