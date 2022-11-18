#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 0"]
    pub uca0ctl0_spi: UCA0CTL0_SPI,
    #[doc = "0x01 - USCI A0 Control Register 1"]
    pub uca0ctl1_spi: UCA0CTL1_SPI,
    #[doc = "0x02 - USCI A0 Baud Rate 0"]
    pub uca0br0_spi: UCA0BR0_SPI,
    #[doc = "0x03 - USCI A0 Baud Rate 1"]
    pub uca0br1_spi: UCA0BR1_SPI,
    #[doc = "0x04 - USCI A0 Modulation Control"]
    pub uca0mctl_spi: UCA0MCTL_SPI,
    #[doc = "0x05 - USCI A0 Status Register"]
    pub uca0stat_spi: UCA0STAT_SPI,
    #[doc = "0x06 - USCI A0 Receive Buffer"]
    pub uca0rxbuf_spi: UCA0RXBUF_SPI,
    #[doc = "0x07 - USCI A0 Transmit Buffer"]
    pub uca0txbuf_spi: UCA0TXBUF_SPI,
}
#[doc = "UCA0CTL0_SPI (rw) register accessor: an alias for `Reg<UCA0CTL0_SPI_SPEC>`"]
pub type UCA0CTL0_SPI = crate::Reg<uca0ctl0_spi::UCA0CTL0_SPI_SPEC>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0_spi;
#[doc = "UCA0CTL1_SPI (rw) register accessor: an alias for `Reg<UCA0CTL1_SPI_SPEC>`"]
pub type UCA0CTL1_SPI = crate::Reg<uca0ctl1_spi::UCA0CTL1_SPI_SPEC>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1_spi;
#[doc = "UCA0BR0_SPI (rw) register accessor: an alias for `Reg<UCA0BR0_SPI_SPEC>`"]
pub type UCA0BR0_SPI = crate::Reg<uca0br0_spi::UCA0BR0_SPI_SPEC>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0_spi;
#[doc = "UCA0BR1_SPI (rw) register accessor: an alias for `Reg<UCA0BR1_SPI_SPEC>`"]
pub type UCA0BR1_SPI = crate::Reg<uca0br1_spi::UCA0BR1_SPI_SPEC>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1_spi;
#[doc = "UCA0MCTL_SPI (rw) register accessor: an alias for `Reg<UCA0MCTL_SPI_SPEC>`"]
pub type UCA0MCTL_SPI = crate::Reg<uca0mctl_spi::UCA0MCTL_SPI_SPEC>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctl_spi;
#[doc = "UCA0STAT_SPI (rw) register accessor: an alias for `Reg<UCA0STAT_SPI_SPEC>`"]
pub type UCA0STAT_SPI = crate::Reg<uca0stat_spi::UCA0STAT_SPI_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat_spi;
#[doc = "UCA0RXBUF_SPI (rw) register accessor: an alias for `Reg<UCA0RXBUF_SPI_SPEC>`"]
pub type UCA0RXBUF_SPI = crate::Reg<uca0rxbuf_spi::UCA0RXBUF_SPI_SPEC>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf_spi;
#[doc = "UCA0TXBUF_SPI (rw) register accessor: an alias for `Reg<UCA0TXBUF_SPI_SPEC>`"]
pub type UCA0TXBUF_SPI = crate::Reg<uca0txbuf_spi::UCA0TXBUF_SPI_SPEC>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf_spi;
