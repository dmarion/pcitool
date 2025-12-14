use crate::capabilities;

capabilities! {
    ext {
        id: 0x0028,
        version: 1,
        name: "Hierarchy ID",
        registers: [
            {
                name: "Hierarchy ID Status",
                offset: 0x04,
                size: Dword,
                fields: [
                    { name: "Message Requester ID", lsb: 0, bits: 16 },
                    { name: "Hierarchy ID Valid", lsb: 28, bits: 1 },
                    { name: "Hierarchy ID Writeable", lsb: 29, bits: 1 },
                ]
            },
            {
                name: "Hierarchy ID Data",
                offset: 0x08,
                size: Dword,
                fields: [
                    { name: "System GUID Authority ID", lsb: 0, bits: 8 },
                    { name: "Hierarchy ID", lsb: 16, bits: 16 },
                ]
            },
            { name: "Hierarchy ID GUID 1", offset: 0x0c, size: Dword, fields: [] },
            { name: "Hierarchy ID GUID 2", offset: 0x10, size: Dword, fields: [] },
            { name: "Hierarchy ID GUID 3", offset: 0x14, size: Dword, fields: [] },
            { name: "Hierarchy ID GUID 4", offset: 0x18, size: Dword, fields: [] },
            { name: "Hierarchy ID GUID 5", offset: 0x1c, size: Dword, fields: [] },
        ]
    }
}
