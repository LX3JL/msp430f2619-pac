#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 0"]
    pub ucb0ctl0_spi: UCB0CTL0_SPI,
    #[doc = "0x01 - USCI B0 Control Register 1"]
    pub ucb0ctl1_spi: UCB0CTL1_SPI,
    #[doc = "0x02 - USCI B0 Baud Rate 0"]
    pub ucb0br0_spi: UCB0BR0_SPI,
    #[doc = "0x03 - USCI B0 Baud Rate 1"]
    pub ucb0br1_spi: UCB0BR1_SPI,
    _reserved4: [u8; 0x01],
    #[doc = "0x05 - USCI B0 Status Register"]
    pub ucb0stat_spi: UCB0STAT_SPI,
    #[doc = "0x06 - USCI B0 Receive Buffer"]
    pub ucb0rxbuf_spi: UCB0RXBUF_SPI,
    #[doc = "0x07 - USCI B0 Transmit Buffer"]
    pub ucb0txbuf_spi: UCB0TXBUF_SPI,
}
#[doc = "UCB0CTL0_SPI (rw) register accessor: an alias for `Reg<UCB0CTL0_SPI_SPEC>`"]
pub type UCB0CTL0_SPI = crate::Reg<ucb0ctl0_spi::UCB0CTL0_SPI_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0_spi;
#[doc = "UCB0CTL1_SPI (rw) register accessor: an alias for `Reg<UCB0CTL1_SPI_SPEC>`"]
pub type UCB0CTL1_SPI = crate::Reg<ucb0ctl1_spi::UCB0CTL1_SPI_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1_spi;
#[doc = "UCB0BR0_SPI (rw) register accessor: an alias for `Reg<UCB0BR0_SPI_SPEC>`"]
pub type UCB0BR0_SPI = crate::Reg<ucb0br0_spi::UCB0BR0_SPI_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0_spi;
#[doc = "UCB0BR1_SPI (rw) register accessor: an alias for `Reg<UCB0BR1_SPI_SPEC>`"]
pub type UCB0BR1_SPI = crate::Reg<ucb0br1_spi::UCB0BR1_SPI_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1_spi;
#[doc = "UCB0STAT_SPI (rw) register accessor: an alias for `Reg<UCB0STAT_SPI_SPEC>`"]
pub type UCB0STAT_SPI = crate::Reg<ucb0stat_spi::UCB0STAT_SPI_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_spi;
#[doc = "UCB0RXBUF_SPI (rw) register accessor: an alias for `Reg<UCB0RXBUF_SPI_SPEC>`"]
pub type UCB0RXBUF_SPI = crate::Reg<ucb0rxbuf_spi::UCB0RXBUF_SPI_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf_spi;
#[doc = "UCB0TXBUF_SPI (rw) register accessor: an alias for `Reg<UCB0TXBUF_SPI_SPEC>`"]
pub type UCB0TXBUF_SPI = crate::Reg<ucb0txbuf_spi::UCB0TXBUF_SPI_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf_spi;
