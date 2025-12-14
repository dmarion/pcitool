pub fn class_name(class: u8, subclass: u8, prog_if: u8) -> String {
    match class {
        0x00 => match subclass {
            0x00 => "Unclassified device".into(),
            0x01 => "VGA compatible unclassified device".into(),
            0x80 => "Other unclassified device".into(),
            other => format!("Unknown Unclassified subclass (0x{:02x})", other),
        },
        0x01 => match subclass {
            0x00 => "SCSI storage controller".into(),
            0x01 => "IDE interface".into(),
            0x02 => "Floppy disk controller".into(),
            0x03 => "IPI bus controller".into(),
            0x04 => "RAID bus controller".into(),
            0x05 => "ATA controller".into(),
            0x06 => {
                if prog_if == 0x01 {
                    "SATA controller (AHCI 1.0)".into()
                } else {
                    "SATA controller".into()
                }
            }
            0x07 => "Serial Attached SCSI controller".into(),
            0x08 => "Non-Volatile memory controller".into(),
            0x80 => "Other mass storage controller".into(),
            other => format!("Unknown Mass storage controller subclass (0x{:02x})", other),
        },
        0x02 => match subclass {
            0x00 => "Ethernet controller".into(),
            0x01 => "Token ring network controller".into(),
            0x02 => "FDDI network controller".into(),
            0x03 => "ATM network controller".into(),
            0x04 => "ISDN controller".into(),
            0x07 => "Infiniband controller".into(),
            0x08 => "Fabric controller".into(),
            0x80 => "Other network controller".into(),
            other => format!("Unknown Network controller subclass (0x{:02x})", other),
        },
        0x03 => match subclass {
            0x00 => "VGA compatible controller".into(),
            0x01 => "XGA controller".into(),
            0x02 => "3D controller".into(),
            0x80 => "Other display controller".into(),
            other => format!("Unknown Display controller subclass (0x{:02x})", other),
        },
        0x04 => match subclass {
            0x00 => "Multimedia video controller".into(),
            0x01 => "Multimedia audio controller".into(),
            0x02 => "Computer telephony device".into(),
            0x03 => "Audio device".into(),
            0x80 => "Other multimedia controller".into(),
            other => format!("Unknown Multimedia controller subclass (0x{:02x})", other),
        },
        0x05 => match subclass {
            0x00 => "RAM memory".into(),
            0x01 => "Flash memory".into(),
            0x80 => "Other memory controller".into(),
            other => format!("Unknown Memory controller subclass (0x{:02x})", other),
        },
        0x06 => match subclass {
            0x00 => "Host bridge".into(),
            0x01 => "ISA bridge".into(),
            0x02 => "EISA bridge".into(),
            0x03 => "MicroChannel bridge".into(),
            0x04 => "PCI bridge".into(),
            0x05 => "PCMCIA bridge".into(),
            0x06 => "NuBus bridge".into(),
            0x07 => "CardBus bridge".into(),
            0x08 => "RACEway bridge".into(),
            0x09 => "PCI-to-PCI bridge".into(),
            0x0a => "Infiniband to PCI host bridge".into(),
            0x80 => "Other bridge".into(),
            other => format!("Unknown Bridge subclass (0x{:02x})", other),
        },
        0x07 => match subclass {
            0x00 => "Serial controller".into(),
            0x01 => "Parallel controller".into(),
            0x02 => "Multiport serial controller".into(),
            0x03 => "Modem".into(),
            0x04 => "IEEE 488.1/2 controller".into(),
            0x05 => "Smart card controller".into(),
            0x80 => "Other communication controller".into(),
            other => format!(
                "Unknown Communication controller subclass (0x{:02x})",
                other
            ),
        },
        0x08 => match subclass {
            0x00 => "PIC".into(),
            0x01 => "DMA controller".into(),
            0x02 => "Timer".into(),
            0x03 => "RTC controller".into(),
            0x04 => "PCI hot-plug controller".into(),
            0x05 => "SD host controller".into(),
            0x06 => "IOMMU".into(),
            0x80 => "Other system peripheral".into(),
            other => format!("Unknown System peripheral subclass (0x{:02x})", other),
        },
        0x09 => match subclass {
            0x00 => "Keyboard controller".into(),
            0x01 => "Digitizer pen".into(),
            0x02 => "Mouse controller".into(),
            0x03 => "Scanner controller".into(),
            0x04 => "Gameport controller".into(),
            0x80 => "Other input device controller".into(),
            other => format!("Unknown Input device controller subclass (0x{:02x})", other),
        },
        0x0a => match subclass {
            0x00 => "Generic docking station".into(),
            0x80 => "Other docking station".into(),
            other => format!("Unknown Docking station subclass (0x{:02x})", other),
        },
        0x0b => match subclass {
            0x00 => "386".into(),
            0x01 => "486".into(),
            0x02 => "Pentium".into(),
            0x03 => "Pentium Pro".into(),
            0x10 => "Alpha".into(),
            0x20 => "PowerPC".into(),
            0x30 => "MIPS".into(),
            0x40 => "Co-processor".into(),
            0x80 => "Other processor".into(),
            other => format!("Unknown Processor subclass (0x{:02x})", other),
        },
        0x0c => match subclass {
            0x00 => "FireWire (IEEE 1394)".into(),
            0x01 => "ACCESS bus".into(),
            0x02 => "SSA".into(),
            0x03 => match prog_if {
                0x00 => "USB UHCI controller".into(),
                0x10 => "USB OHCI controller".into(),
                0x20 => "USB EHCI controller".into(),
                0x30 => "USB XHCI controller".into(),
                0x40 => "USB4 host interface".into(),
                0x80 => "USB controller".into(),
                0xfe => "USB device".into(),
                other => format!("Unknown USB prog-if (0x{:02x})", other),
            },
            0x04 => "Fibre Channel".into(),
            0x05 => "SMBus".into(),
            0x06 => "InfiniBand".into(),
            0x07 => match prog_if {
                0x00 => "IPMI SMIC interface".into(),
                0x01 => "IPMI KCS interface".into(),
                0x02 => "IPMI BT interface".into(),
                other => format!("Unknown IPMI interface (0x{:02x})", other),
            },
            0x08 => "SERCOS interface".into(),
            0x09 => "CANbus".into(),
            other => format!("Unknown Serial bus controller subclass (0x{:02x})", other),
        },
        0x0d => match subclass {
            0x00 => "IRDA controller".into(),
            0x01 => "Consumer IR controller".into(),
            0x10 => "RF controller".into(),
            0x11 => "Bluetooth".into(),
            0x12 => "Broadband".into(),
            0x20 => "Ethernet 802.1a controller".into(),
            0x21 => "Ethernet 802.1b controller".into(),
            0x80 => "Other Wireless Controller".into(),
            other => format!("Unknown Wireless Controller subclass (0x{:02x})", other),
        },
        0x0e => "Intelligent I/O controller".into(),
        0x0f => match subclass {
            0x01 => "Satellite TV controller".into(),
            0x02 => "Satellite audio controller".into(),
            0x03 => "Satellite voice controller".into(),
            0x04 => "Satellite data controller".into(),
            0x80 => "Other satellite communication controller".into(),
            other => format!(
                "Unknown Satellite communication controller subclass (0x{:02x})",
                other
            ),
        },
        0x10 => match subclass {
            0x00 => "Network encryption controller".into(),
            0x10 => "Entertainment encryption controller".into(),
            0x80 => "Other encryption/decryption controller".into(),
            other => format!(
                "Unknown Encryption/decryption controller subclass (0x{:02x})",
                other
            ),
        },
        0x11 => match subclass {
            0x00 => "DPIO module".into(),
            0x01 => "Performance counters".into(),
            0x10 => "Communication synchronizer".into(),
            0x20 => "Management card".into(),
            0x80 => "Other data acquisition/signal processing controller".into(),
            other => format!(
                "Unknown Data acquisition/signal processing subclass (0x{:02x})",
                other
            ),
        },
        0x12 => "Processing accelerator".into(),
        0x13 => "Non-Essential Instrumentation".into(),
        other => format!(
            "Unknown class 0x{:02x}{:02x}{:02x}",
            other, subclass, prog_if
        ),
    }
}
