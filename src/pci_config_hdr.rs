use anyhow::Result;

#[derive(Debug)]
pub struct PciConfigHdr {
    pub vendor_id: u16,
    pub device_id: u16,
    pub capabilities_list: bool,
    pub prog_if: u8,
    pub subclass: u8,
    pub class_code: u8,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
    pub capabilities_pointer: u8,
}

impl PciConfigHdr {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let vendor_id = u16::from_le_bytes([bytes[0], bytes[1]]);
        let device_id = u16::from_le_bytes([bytes[2], bytes[3]]);
        let status_word = u16::from_le_bytes([bytes[6], bytes[7]]);
        let capabilities_pointer = bytes[0x34];
        let prog_if = bytes[0x09];
        let subclass = bytes[0x0a];
        let class_code = bytes[0x0b];
        let subsystem_vendor_id = u16::from_le_bytes([bytes[0x2c], bytes[0x2d]]);
        let subsystem_id = u16::from_le_bytes([bytes[0x2e], bytes[0x2f]]);

        Ok(Self {
            vendor_id,
            device_id,
            capabilities_list: status_word & 0x0010 != 0,
            prog_if,
            subclass,
            class_code,
            subsystem_vendor_id,
            subsystem_id,
            capabilities_pointer,
        })
    }
}
