use crate::pci_device::PciCapa;
use crate::tree::{TreeLine, TreeNode};
use linkme::distributed_slice;

pub type StdCapEntry = (u8, u8, u8);
pub type ExtCapEntry = (u16, u8, u16, u16);

#[derive(Clone, Copy)]
pub enum RegisterSize {
    Byte,
    Word,
    Dword,
    Qword,
}

#[derive(Clone, Copy)]
pub struct FieldDesc {
    pub name: &'static str,
    pub lsb: u8,
    pub bits: u8,
    pub enum_values: &'static [(u64, &'static str)],
}

#[derive(Clone, Copy)]
pub struct RegisterDesc {
    pub name: &'static str,
    pub offset: u16,
    pub size: RegisterSize,
    pub fields: &'static [FieldDesc],
}

#[derive(Clone, Copy)]
pub enum CapaSize {
    Fixed(u16),
    Dynamic(fn(&PciCapa) -> Option<u16>),
}

pub struct CapabilityDesc {
    pub id: u16,
    pub version: Option<u8>,
    pub is_extended: bool,
    pub name: &'static str,
    pub size: CapaSize,
    pub registers: &'static [RegisterDesc],
    pub summary: Option<fn(&PciCapa) -> Option<Vec<TreeNode>>>,
}

#[distributed_slice]
pub static EXT_CAP_REGISTRY: [&'static CapabilityDesc] = [..];

#[distributed_slice]
pub static STD_CAP_REGISTRY: [&'static CapabilityDesc] = [..];

#[macro_export]
macro_rules! capabilities {
    ( $( $entry:tt ),* $(,)? ) => {
        $( $crate::capabilities!(@entry $entry); )*
    };

    (@entry {
        id: $id:expr,
        version: $ver:expr,
        is_extended: true,
        name: $name:expr,
        $(size: $size:expr,)?
        $(get_size: $get_size:expr,)?
        $(summary: $summary:expr,)?
        registers: [
            $(
                {
                    name: $reg_name:expr,
                    offset: $reg_off:expr,
                    id: $reg_id:ident,
                    size: $reg_size:ident,
                    fields: [
                        $(
                            {
                                name: $fname:expr,
                                lsb: $flsb:expr,
                                bits: $fbits:expr
                                $(, enum_values: $enums:tt )?
                            }
                        ),* $(,)?
                    ]
                }
            ),* $(,)?
        ]
    }) => {
        $(
            $crate::capabilities!(@reg_const $reg_off, $reg_id);
        )*
        $crate::capabilities!(@impl ext, $id, $ver, $name,
            size: { $($size)? },
            get_size: { $($get_size)? },
            summary: { $($summary)? },
            [
                $(
                    $crate::pci_capa::RegisterDesc {
                        name: $reg_name,
                        offset: $reg_id,
                        size: $crate::pci_capa::RegisterSize::$reg_size,
                        fields: &[
                            $(
                                $crate::capabilities!(
                                    @field $fname, $flsb, $fbits
                                    $(, $enums)?
                                )
                            ),*
                        ],
                    }
                ),*
            ]
        );
    };

    (@entry {
        id: $id:expr,
        $(is_extended: false,)?
        name: $name:expr,
        $(size: $size:expr,)?
        $(get_size: $get_size:expr,)?
        $(summary: $summary:expr,)?
        registers: [
            $(
                {
                    name: $reg_name:expr,
                    offset: $reg_off:expr,
                    id: $reg_id:ident,
                    size: $reg_size:ident,
                    fields: [
                        $(
                            {
                                name: $fname:expr,
                                lsb: $flsb:expr,
                                bits: $fbits:expr
                                $(, enum_values: $enums:tt )?
                            }
                        ),* $(,)?
                    ]
                }
            ),* $(,)?
        ]
    }) => {
        $(
            $crate::capabilities!(@reg_const $reg_off, $reg_id);
        )*
        $crate::capabilities!(@impl std, $id, $name,
            size: { $($size)? },
            get_size: { $($get_size)? },
            summary: { $($summary)? },
            [
                $(
                    $crate::pci_capa::RegisterDesc {
                        name: $reg_name,
                        offset: $reg_id,
                        size: $crate::pci_capa::RegisterSize::$reg_size,
                        fields: &[
                            $(
                                $crate::capabilities!(
                                    @field $fname, $flsb, $fbits
                                    $(, $enums)?
                                )
                            ),*
                        ],
                    }
                ),*
            ]
        );
    };

    (@reg_const $off:expr, $id:ident) => {
        const $id: u16 = $off;
    };

    (@field $name:expr, $lsb:expr, $bits:expr) => {
        $crate::pci_capa::FieldDesc {
            name: $name,
            lsb: $lsb,
            bits: $bits,
            enum_values: &[],
        }
    };

    (@field $name:expr, $lsb:expr, $bits:expr, [ $(($eval:expr, $ename:expr)),* $(,)? ]) => {
        $crate::pci_capa::FieldDesc {
            name: $name,
            lsb: $lsb,
            bits: $bits,
            enum_values: &[ $(($eval, $ename)),* ],
        }
    };

    (@field $name:expr, $lsb:expr, $bits:expr, $enums:expr) => {
        $crate::pci_capa::FieldDesc {
            name: $name,
            lsb: $lsb,
            bits: $bits,
            enum_values: $enums,
        }
    };

    // Standard Capabilities - Fixed Size
    (@impl std, $id:expr, $name:expr,
        size: { $size:expr },
        get_size: { },
        summary: { $summary:expr },
        [ $($regs:expr),* ]
    ) => {
        const _: () = {
            #[linkme::distributed_slice($crate::pci_capa::STD_CAP_REGISTRY)]
            static CAPABILITY_DESC: &'static $crate::pci_capa::CapabilityDesc = &$crate::pci_capa::CapabilityDesc {
                id: $id as u16,
                version: None,
                is_extended: false,
                name: $name,
                size: $crate::pci_capa::CapaSize::Fixed($size),
                registers: &[
                    $crate::pci_capa::RegisterDesc {
                        name: "Header",
                        offset: 0x00,
                        size: $crate::pci_capa::RegisterSize::Word,
                        fields: &[
                            $crate::capabilities!(@field "Capability ID", 0, 8),
                            $crate::capabilities!(@field "Next Capability Pointer", 8, 8),
                        ],
                    },
                    $($regs),*
                ],
                summary: Some($summary),
            };
        };
    };

    (@impl std, $id:expr, $name:expr,
        size: { $size:expr },
        get_size: { },
        summary: { },
        [ $($regs:expr),* ]
    ) => {
        const _: () = {
            #[linkme::distributed_slice($crate::pci_capa::STD_CAP_REGISTRY)]
            static CAPABILITY_DESC: &'static $crate::pci_capa::CapabilityDesc = &$crate::pci_capa::CapabilityDesc {
                id: $id as u16,
                version: None,
                is_extended: false,
                name: $name,
                size: $crate::pci_capa::CapaSize::Fixed($size),
                registers: &[
                    $crate::pci_capa::RegisterDesc {
                        name: "Header",
                        offset: 0x00,
                        size: $crate::pci_capa::RegisterSize::Word,
                        fields: &[
                            $crate::capabilities!(@field "Capability ID", 0, 8),
                            $crate::capabilities!(@field "Next Capability Pointer", 8, 8),
                        ],
                    },
                    $($regs),*
                ],
                summary: None,
            };
        };
    };

    // Standard Capabilities - Dynamic Size
    (@impl std, $id:expr, $name:expr,
        size: { },
        get_size: { $get_size:expr },
        summary: { $summary:expr },
        [ $($regs:expr),* ]
    ) => {
        const _: () = {
            #[linkme::distributed_slice($crate::pci_capa::STD_CAP_REGISTRY)]
            static CAPABILITY_DESC: &'static $crate::pci_capa::CapabilityDesc = &$crate::pci_capa::CapabilityDesc {
                id: $id as u16,
                version: None,
                is_extended: false,
                name: $name,
                size: $crate::pci_capa::CapaSize::Dynamic($get_size),
                registers: &[
                    $crate::pci_capa::RegisterDesc {
                        name: "Header",
                        offset: 0x00,
                        size: $crate::pci_capa::RegisterSize::Word,
                        fields: &[
                            $crate::capabilities!(@field "Capability ID", 0, 8),
                            $crate::capabilities!(@field "Next Capability Pointer", 8, 8),
                        ],
                    },
                    $($regs),*
                ],
                summary: Some($summary),
            };
        };
    };

    (@impl std, $id:expr, $name:expr,
        size: { },
        get_size: { $get_size:expr },
        summary: { },
        [ $($regs:expr),* ]
    ) => {
        const _: () = {
            #[linkme::distributed_slice($crate::pci_capa::STD_CAP_REGISTRY)]
            static CAPABILITY_DESC: &'static $crate::pci_capa::CapabilityDesc = &$crate::pci_capa::CapabilityDesc {
                id: $id as u16,
                version: None,
                is_extended: false,
                name: $name,
                size: $crate::pci_capa::CapaSize::Dynamic($get_size),
                registers: &[
                    $crate::pci_capa::RegisterDesc {
                        name: "Header",
                        offset: 0x00,
                        size: $crate::pci_capa::RegisterSize::Word,
                        fields: &[
                            $crate::capabilities!(@field "Capability ID", 0, 8),
                            $crate::capabilities!(@field "Next Capability Pointer", 8, 8),
                        ],
                    },
                    $($regs),*
                ],
                summary: None,
            };
        };
    };

    // Extended Capabilities - Fixed Size
    (@impl ext, $id:expr, $ver:expr, $name:expr,
        size: { $size:expr },
        get_size: { },
        summary: { $summary:expr },
        [ $($regs:expr),* ]
    ) => {
        const _: () = {
            #[linkme::distributed_slice($crate::pci_capa::EXT_CAP_REGISTRY)]
            static CAPABILITY_DESC: &'static $crate::pci_capa::CapabilityDesc = &$crate::pci_capa::CapabilityDesc {
                id: $id,
                version: Some($ver),
                is_extended: true,
                name: $name,
                size: $crate::pci_capa::CapaSize::Fixed($size),
                registers: &[
                    $crate::pci_capa::RegisterDesc {
                        name: "Header",
                        offset: 0x00,
                        size: $crate::pci_capa::RegisterSize::Dword,
                        fields: &[
                            $crate::capabilities!(@field "Extended Capability ID", 0, 16),
                            $crate::capabilities!(@field "Capability Version", 16, 4),
                            $crate::capabilities!(@field "Next Capability Offset", 20, 12),
                        ],
                    },
                    $($regs),*
                ],
                summary: Some($summary),
            };
        };
    };

    (@impl ext, $id:expr, $ver:expr, $name:expr,
        size: { $size:expr },
        get_size: { },
        summary: { },
        [ $($regs:expr),* ]
    ) => {
        const _: () = {
            #[linkme::distributed_slice($crate::pci_capa::EXT_CAP_REGISTRY)]
            static CAPABILITY_DESC: &'static $crate::pci_capa::CapabilityDesc = &$crate::pci_capa::CapabilityDesc {
                id: $id,
                version: Some($ver),
                is_extended: true,
                name: $name,
                size: $crate::pci_capa::CapaSize::Fixed($size),
                registers: &[
                    $crate::pci_capa::RegisterDesc {
                        name: "Header",
                        offset: 0x00,
                        size: $crate::pci_capa::RegisterSize::Dword,
                        fields: &[
                            $crate::capabilities!(@field "Extended Capability ID", 0, 16),
                            $crate::capabilities!(@field "Capability Version", 16, 4),
                            $crate::capabilities!(@field "Next Capability Offset", 20, 12),
                        ],
                    },
                    $($regs),*
                ],
                summary: None,
            };
        };
    };

    // Extended Capabilities - Dynamic Size
    (@impl ext, $id:expr, $ver:expr, $name:expr,
        size: { },
        get_size: { $get_size:expr },
        summary: { $summary:expr },
        [ $($regs:expr),* ]
    ) => {
        const _: () = {
            #[linkme::distributed_slice($crate::pci_capa::EXT_CAP_REGISTRY)]
            static CAPABILITY_DESC: &'static $crate::pci_capa::CapabilityDesc = &$crate::pci_capa::CapabilityDesc {
                id: $id,
                version: Some($ver),
                is_extended: true,
                name: $name,
                size: $crate::pci_capa::CapaSize::Dynamic($get_size),
                registers: &[
                    $crate::pci_capa::RegisterDesc {
                        name: "Header",
                        offset: 0x00,
                        size: $crate::pci_capa::RegisterSize::Dword,
                        fields: &[
                            $crate::capabilities!(@field "Extended Capability ID", 0, 16),
                            $crate::capabilities!(@field "Capability Version", 16, 4),
                            $crate::capabilities!(@field "Next Capability Offset", 20, 12),
                        ],
                    },
                    $($regs),*
                ],
                summary: Some($summary),
            };
        };
    };

    (@impl ext, $id:expr, $ver:expr, $name:expr,
        size: { },
        get_size: { $get_size:expr },
        summary: { },
        [ $($regs:expr),* ]
    ) => {
        const _: () = {
            #[linkme::distributed_slice($crate::pci_capa::EXT_CAP_REGISTRY)]
            static CAPABILITY_DESC: &'static $crate::pci_capa::CapabilityDesc = &$crate::pci_capa::CapabilityDesc {
                id: $id,
                version: Some($ver),
                is_extended: true,
                name: $name,
                size: $crate::pci_capa::CapaSize::Dynamic($get_size),
                registers: &[
                    $crate::pci_capa::RegisterDesc {
                        name: "Header",
                        offset: 0x00,
                        size: $crate::pci_capa::RegisterSize::Dword,
                        fields: &[
                            $crate::capabilities!(@field "Extended Capability ID", 0, 16),
                            $crate::capabilities!(@field "Capability Version", 16, 4),
                            $crate::capabilities!(@field "Next Capability Offset", 20, 12),
                        ],
                    },
                    $($regs),*
                ],
                summary: None,
            };
        };
    };
}

pub fn print_registers(
    regs: &[RegisterDesc],
    mut fetch: impl FnMut(u16, RegisterSize) -> Option<u64>,
    cap_size: u16,
    offset_width: usize,
    out: &mut Vec<TreeNode>,
) {
    for reg in regs {
        let reg_len = match reg.size {
            RegisterSize::Byte => 1,
            RegisterSize::Word => 2,
            RegisterSize::Dword => 4,
            RegisterSize::Qword => 8,
        };
        let reg_end = reg.offset as u32 + reg_len;
        if reg_end > cap_size as u32 {
            continue;
        }

        if let Some(raw) = fetch(reg.offset, reg.size) {
            let value_str = format_reg_value(raw, reg.size);
            let mut reg_node = TreeNode::with_value_collapsed(
                TreeLine::from(format!(
                    "  0x{:0width$x} {}",
                    reg.offset,
                    reg.name,
                    width = offset_width
                )),
                TreeLine::from(format!("{}", value_str)),
            );

            for field in reg.fields {
                let mask = if field.bits >= 64 {
                    u64::MAX
                } else {
                    (1u128 << field.bits) as u64 - 1
                };
                let val = (raw >> field.lsb) & mask;
                let mut field_node = if field.bits == 1 {
                    TreeNode::with_value(
                        TreeLine::from(format!("     {}", field.name)),
                        TreeLine::from(if val != 0 { "on" } else { "off" }),
                    )
                } else {
                    let mut val_str = if val > 9 {
                        let hex_digits = (field.bits + 3) / 4;
                        format!("0x{:0width$x} ({})", val, val, width = hex_digits as usize)
                    } else {
                        val.to_string()
                    };

                    if let Some((_, name)) = field.enum_values.iter().find(|(v, _)| *v == val) {
                        val_str = format!("{} ({})", val_str, name);
                    }

                    TreeNode::with_value(
                        TreeLine::from(format!("     {}", field.name)),
                        TreeLine::from(val_str),
                    )
                };
                field_node.align_with_parent = true;
                reg_node.add_child(field_node);
            }
            out.push(reg_node);
        }
    }
}

pub fn format_reg_value(raw: u64, size: RegisterSize) -> String {
    match size {
        RegisterSize::Byte => format!("0x{:02x}", raw as u8),
        RegisterSize::Word => format!("0x{:04x}", raw as u16),
        RegisterSize::Dword => format!("0x{:08x}", raw),
        RegisterSize::Qword => format!("0x{:016x}", raw),
    }
}
