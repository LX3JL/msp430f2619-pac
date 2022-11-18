#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - SVS Control"]
    pub svsctl: SVSCTL,
}
#[doc = "SVSCTL (rw) register accessor: an alias for `Reg<SVSCTL_SPEC>`"]
pub type SVSCTL = crate::Reg<svsctl::SVSCTL_SPEC>;
#[doc = "SVS Control"]
pub mod svsctl;
