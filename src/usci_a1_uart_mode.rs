#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - USCI A1 LIN Control"]
    pub uca1abctl: UCA1ABCTL,
    #[doc = "0x02 - USCI A1 IrDA Transmit Control"]
    pub uca1irtctl: UCA1IRTCTL,
    #[doc = "0x03 - USCI A1 IrDA Receive Control"]
    pub uca1irrctl: UCA1IRRCTL,
    #[doc = "0x04 - USCI A1 Control Register 0"]
    pub uca1ctl0: UCA1CTL0,
    #[doc = "0x05 - USCI A1 Control Register 1"]
    pub uca1ctl1: UCA1CTL1,
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    pub uca1br0: UCA1BR0,
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    pub uca1br1: UCA1BR1,
    #[doc = "0x08 - USCI A1 Modulation Control"]
    pub uca1mctl: UCA1MCTL,
    #[doc = "0x09 - USCI A1 Status Register"]
    pub uca1stat: UCA1STAT,
    #[doc = "0x0a - USCI A1 Receive Buffer"]
    pub uca1rxbuf: UCA1RXBUF,
    #[doc = "0x0b - USCI A1 Transmit Buffer"]
    pub uca1txbuf: UCA1TXBUF,
}
#[doc = "UCA1ABCTL (rw) register accessor: an alias for `Reg<UCA1ABCTL_SPEC>`"]
pub type UCA1ABCTL = crate::Reg<uca1abctl::UCA1ABCTL_SPEC>;
#[doc = "USCI A1 LIN Control"]
pub mod uca1abctl;
#[doc = "UCA1IRTCTL (rw) register accessor: an alias for `Reg<UCA1IRTCTL_SPEC>`"]
pub type UCA1IRTCTL = crate::Reg<uca1irtctl::UCA1IRTCTL_SPEC>;
#[doc = "USCI A1 IrDA Transmit Control"]
pub mod uca1irtctl;
#[doc = "UCA1IRRCTL (rw) register accessor: an alias for `Reg<UCA1IRRCTL_SPEC>`"]
pub type UCA1IRRCTL = crate::Reg<uca1irrctl::UCA1IRRCTL_SPEC>;
#[doc = "USCI A1 IrDA Receive Control"]
pub mod uca1irrctl;
#[doc = "UCA1CTL0 (rw) register accessor: an alias for `Reg<UCA1CTL0_SPEC>`"]
pub type UCA1CTL0 = crate::Reg<uca1ctl0::UCA1CTL0_SPEC>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0;
#[doc = "UCA1CTL1 (rw) register accessor: an alias for `Reg<UCA1CTL1_SPEC>`"]
pub type UCA1CTL1 = crate::Reg<uca1ctl1::UCA1CTL1_SPEC>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1;
#[doc = "UCA1BR0 (rw) register accessor: an alias for `Reg<UCA1BR0_SPEC>`"]
pub type UCA1BR0 = crate::Reg<uca1br0::UCA1BR0_SPEC>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0;
#[doc = "UCA1BR1 (rw) register accessor: an alias for `Reg<UCA1BR1_SPEC>`"]
pub type UCA1BR1 = crate::Reg<uca1br1::UCA1BR1_SPEC>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1;
#[doc = "UCA1MCTL (rw) register accessor: an alias for `Reg<UCA1MCTL_SPEC>`"]
pub type UCA1MCTL = crate::Reg<uca1mctl::UCA1MCTL_SPEC>;
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctl;
#[doc = "UCA1STAT (rw) register accessor: an alias for `Reg<UCA1STAT_SPEC>`"]
pub type UCA1STAT = crate::Reg<uca1stat::UCA1STAT_SPEC>;
#[doc = "USCI A1 Status Register"]
pub mod uca1stat;
#[doc = "UCA1RXBUF (rw) register accessor: an alias for `Reg<UCA1RXBUF_SPEC>`"]
pub type UCA1RXBUF = crate::Reg<uca1rxbuf::UCA1RXBUF_SPEC>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf;
#[doc = "UCA1TXBUF (rw) register accessor: an alias for `Reg<UCA1TXBUF_SPEC>`"]
pub type UCA1TXBUF = crate::Reg<uca1txbuf::UCA1TXBUF_SPEC>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf;
