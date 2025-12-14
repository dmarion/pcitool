use crate::capabilities;

capabilities! {
    ext {
        id: 0x0029,
        version: 1,
        name: "Native PCIe Enclosure Management",
        registers: [
            {
                name: "NPEM Capability",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "NPEM Capable", lsb: 0, bits: 1 },
                    { name: "NPEM Reset Capable", lsb: 1, bits: 1 },
                    { name: "NPEM OK Capable", lsb: 2, bits: 1 },
                    { name: "NPEM Locate Capable", lsb: 3, bits: 1 },
                    { name: "NPEM Fail Capable", lsb: 4, bits: 1 },
                    { name: "NPEM Rebuild Capable", lsb: 5, bits: 1 },
                    { name: "NPEM PFA Capable", lsb: 6, bits: 1 },
                    { name: "NPEM Hot Spare Capable", lsb: 7, bits: 1 },
                    { name: "NPEM In A Critical Array Capable", lsb: 8, bits: 1 },
                    { name: "NPEM In A Failed Array Capable", lsb: 9, bits: 1 },
                    { name: "NPEM Invalid Device Type Capable", lsb: 10, bits: 1 },
                    { name: "NPEM Disabled Capable", lsb: 11, bits: 1 },
                    { name: "Enclosure-specific Capabilities", lsb: 12, bits: 12 },
                ]
            },
            {
                name: "NPEM Control",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "NPEM Enable", lsb: 0, bits: 1 },
                    { name: "NPEM Initiate Reset", lsb: 1, bits: 1 },
                    { name: "NPEM OK Control", lsb: 2, bits: 1 },
                    { name: "NPEM Locate Control", lsb: 3, bits: 1 },
                    { name: "NPEM Fail Control", lsb: 4, bits: 1 },
                    { name: "NPEM Rebuild Control", lsb: 5, bits: 1 },
                    { name: "NPEM PFA Control", lsb: 6, bits: 1 },
                    { name: "NPEM Hot Spare Control", lsb: 7, bits: 1 },
                    { name: "NPEM In A Critical Array Control", lsb: 8, bits: 1 },
                    { name: "NPEM In A Failed Array Control", lsb: 9, bits: 1 },
                    { name: "NPEM Invalid Device Type Control", lsb: 10, bits: 1 },
                    { name: "NPEM Disabled Control", lsb: 11, bits: 1 },
                    { name: "Enclosure-specific Indications", lsb: 12, bits: 12 },
                ]
            },
            {
                name: "NPEM Status",
                offset: 0x0c,
                size: Dword,
                fields: [
                    { name: "NPEM Command Completed", lsb: 0, bits: 1 },
                ]
            },
        ]
    }
}
