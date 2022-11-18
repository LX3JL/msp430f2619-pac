#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 0"]
    pub uca1ctl0_spi: UCA1CTL0_SPI,
    #[doc = "0x01 - USCI A1 Control Register 1"]
    pub uca1ctl1_spi: UCA1CTL1_SPI,
    #[doc = "0x02 - USCI A1 Baud Rate 0"]
    pub uca1br0_spi: UCA1BR0_SPI,
    #[doc = "0x03 - USCI A1 Baud Rate 1"]
    pub uca1br1_spi: UCA1BR1_SPI,
    #[doc = "0x04 - USCI A1 Modulation Control"]
    pub uca1mctl_spi: UCA1MCTL_SPI,
    #[doc = "0x05 - USCI A1 Status Register"]
    pub uca1stat_spi: UCA1STAT_SPI,
    #[doc = "0x06 - USCI A1 Receive Buffer"]
    pub uca1rxbuf_spi: UCA1RXBUF_SPI,
    #[doc = "0x07 - USCI A1 Transmit Buffer"]
    pub uca1txbuf_spi: UCA1TXBUF_SPI,
}
#[doc = "UCA1CTL0_SPI (rw) register accessor: an alias for `Reg<UCA1CTL0_SPI_SPEC>`"]
pub type UCA1CTL0_SPI = crate::Reg<uca1ctl0_spi::UCA1CTL0_SPI_SPEC>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0_spi;
#[doc = "UCA1CTL1_SPI (rw) register accessor: an alias for `Reg<UCA1CTL1_SPI_SPEC>`"]
pub type UCA1CTL1_SPI = crate::Reg<uca1ctl1_spi::UCA1CTL1_SPI_SPEC>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1_spi;
#[doc = "UCA1BR0_SPI (rw) register accessor: an alias for `Reg<UCA1BR0_SPI_SPEC>`"]
pub type UCA1BR0_SPI = crate::Reg<uca1br0_spi::UCA1BR0_SPI_SPEC>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0_spi;
#[doc = "UCA1BR1_SPI (rw) register accessor: an alias for `Reg<UCA1BR1_SPI_SPEC>`"]
pub type UCA1BR1_SPI = crate::Reg<uca1br1_spi::UCA1BR1_SPI_SPEC>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1_spi;
#[doc = "UCA1MCTL_SPI (rw) register accessor: an alias for `Reg<UCA1MCTL_SPI_SPEC>`"]
pub type UCA1MCTL_SPI = crate::Reg<uca1mctl_spi::UCA1MCTL_SPI_SPEC>;
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctl_spi;
#[doc = "UCA1STAT_SPI (rw) register accessor: an alias for `Reg<UCA1STAT_SPI_SPEC>`"]
pub type UCA1STAT_SPI = crate::Reg<uca1stat_spi::UCA1STAT_SPI_SPEC>;
#[doc = "USCI A1 Status Register"]
pub mod uca1stat_spi;
#[doc = "UCA1RXBUF_SPI (rw) register accessor: an alias for `Reg<UCA1RXBUF_SPI_SPEC>`"]
pub type UCA1RXBUF_SPI = crate::Reg<uca1rxbuf_spi::UCA1RXBUF_SPI_SPEC>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf_spi;
#[doc = "UCA1TXBUF_SPI (rw) register accessor: an alias for `Reg<UCA1TXBUF_SPI_SPEC>`"]
pub type UCA1TXBUF_SPI = crate::Reg<uca1txbuf_spi::UCA1TXBUF_SPI_SPEC>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf_spi;
