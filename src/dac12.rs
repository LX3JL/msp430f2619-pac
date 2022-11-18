#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC12_0 Control"]
    pub dac12_0ctl: DAC12_0CTL,
    #[doc = "0x02 - DAC12_1 Control"]
    pub dac12_1ctl: DAC12_1CTL,
    _reserved2: [u8; 0x04],
    #[doc = "0x08 - DAC12_0 Data"]
    pub dac12_0dat: DAC12_0DAT,
    #[doc = "0x0a - DAC12_1 Data"]
    pub dac12_1dat: DAC12_1DAT,
}
#[doc = "DAC12_0CTL (rw) register accessor: an alias for `Reg<DAC12_0CTL_SPEC>`"]
pub type DAC12_0CTL = crate::Reg<dac12_0ctl::DAC12_0CTL_SPEC>;
#[doc = "DAC12_0 Control"]
pub mod dac12_0ctl;
#[doc = "DAC12_1CTL (rw) register accessor: an alias for `Reg<DAC12_1CTL_SPEC>`"]
pub type DAC12_1CTL = crate::Reg<dac12_1ctl::DAC12_1CTL_SPEC>;
#[doc = "DAC12_1 Control"]
pub mod dac12_1ctl;
#[doc = "DAC12_0DAT (rw) register accessor: an alias for `Reg<DAC12_0DAT_SPEC>`"]
pub type DAC12_0DAT = crate::Reg<dac12_0dat::DAC12_0DAT_SPEC>;
#[doc = "DAC12_0 Data"]
pub mod dac12_0dat;
#[doc = "DAC12_1DAT (rw) register accessor: an alias for `Reg<DAC12_1DAT_SPEC>`"]
pub type DAC12_1DAT = crate::Reg<dac12_1dat::DAC12_1DAT_SPEC>;
#[doc = "DAC12_1 Data"]
pub mod dac12_1dat;
