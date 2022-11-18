#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 0"]
    pub ucb1ctl0: UCB1CTL0,
    #[doc = "0x01 - USCI B1 Control Register 1"]
    pub ucb1ctl1: UCB1CTL1,
    #[doc = "0x02 - USCI B1 Baud Rate 0"]
    pub ucb1br0: UCB1BR0,
    #[doc = "0x03 - USCI B1 Baud Rate 1"]
    pub ucb1br1: UCB1BR1,
    #[doc = "0x04 - USCI B1 I2C Interrupt Enable Register"]
    pub ucb1i2cie: UCB1I2CIE,
    #[doc = "0x05 - USCI B1 Status Register"]
    pub ucb1stat: UCB1STAT,
    #[doc = "0x06 - USCI B1 Receive Buffer"]
    pub ucb1rxbuf: UCB1RXBUF,
    #[doc = "0x07 - USCI B1 Transmit Buffer"]
    pub ucb1txbuf: UCB1TXBUF,
    _reserved8: [u8; 0x9c],
    #[doc = "0xa4 - USCI B1 I2C Own Address"]
    pub ucb1i2coa: UCB1I2COA,
    #[doc = "0xa6 - USCI B1 I2C Slave Address"]
    pub ucb1i2csa: UCB1I2CSA,
}
#[doc = "UCB1CTL0 (rw) register accessor: an alias for `Reg<UCB1CTL0_SPEC>`"]
pub type UCB1CTL0 = crate::Reg<ucb1ctl0::UCB1CTL0_SPEC>;
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0;
#[doc = "UCB1CTL1 (rw) register accessor: an alias for `Reg<UCB1CTL1_SPEC>`"]
pub type UCB1CTL1 = crate::Reg<ucb1ctl1::UCB1CTL1_SPEC>;
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1;
#[doc = "UCB1BR0 (rw) register accessor: an alias for `Reg<UCB1BR0_SPEC>`"]
pub type UCB1BR0 = crate::Reg<ucb1br0::UCB1BR0_SPEC>;
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0;
#[doc = "UCB1BR1 (rw) register accessor: an alias for `Reg<UCB1BR1_SPEC>`"]
pub type UCB1BR1 = crate::Reg<ucb1br1::UCB1BR1_SPEC>;
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1;
#[doc = "UCB1I2CIE (rw) register accessor: an alias for `Reg<UCB1I2CIE_SPEC>`"]
pub type UCB1I2CIE = crate::Reg<ucb1i2cie::UCB1I2CIE_SPEC>;
#[doc = "USCI B1 I2C Interrupt Enable Register"]
pub mod ucb1i2cie;
#[doc = "UCB1STAT (rw) register accessor: an alias for `Reg<UCB1STAT_SPEC>`"]
pub type UCB1STAT = crate::Reg<ucb1stat::UCB1STAT_SPEC>;
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat;
#[doc = "UCB1RXBUF (rw) register accessor: an alias for `Reg<UCB1RXBUF_SPEC>`"]
pub type UCB1RXBUF = crate::Reg<ucb1rxbuf::UCB1RXBUF_SPEC>;
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf;
#[doc = "UCB1TXBUF (rw) register accessor: an alias for `Reg<UCB1TXBUF_SPEC>`"]
pub type UCB1TXBUF = crate::Reg<ucb1txbuf::UCB1TXBUF_SPEC>;
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf;
#[doc = "UCB1I2COA (rw) register accessor: an alias for `Reg<UCB1I2COA_SPEC>`"]
pub type UCB1I2COA = crate::Reg<ucb1i2coa::UCB1I2COA_SPEC>;
#[doc = "USCI B1 I2C Own Address"]
pub mod ucb1i2coa;
#[doc = "UCB1I2CSA (rw) register accessor: an alias for `Reg<UCB1I2CSA_SPEC>`"]
pub type UCB1I2CSA = crate::Reg<ucb1i2csa::UCB1I2CSA_SPEC>;
#[doc = "USCI B1 I2C Slave Address"]
pub mod ucb1i2csa;
