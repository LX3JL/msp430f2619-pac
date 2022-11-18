#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH Control 1"]
    pub fctl1: FCTL1,
    #[doc = "0x02 - FLASH Control 2"]
    pub fctl2: FCTL2,
    #[doc = "0x04 - FLASH Control 3"]
    pub fctl3: FCTL3,
    _reserved3: [u8; 0x90],
    #[doc = "0x96 - FLASH Control 4"]
    pub fctl4: FCTL4,
}
#[doc = "FCTL1 (rw) register accessor: an alias for `Reg<FCTL1_SPEC>`"]
pub type FCTL1 = crate::Reg<fctl1::FCTL1_SPEC>;
#[doc = "FLASH Control 1"]
pub mod fctl1;
#[doc = "FCTL2 (rw) register accessor: an alias for `Reg<FCTL2_SPEC>`"]
pub type FCTL2 = crate::Reg<fctl2::FCTL2_SPEC>;
#[doc = "FLASH Control 2"]
pub mod fctl2;
#[doc = "FCTL3 (rw) register accessor: an alias for `Reg<FCTL3_SPEC>`"]
pub type FCTL3 = crate::Reg<fctl3::FCTL3_SPEC>;
#[doc = "FLASH Control 3"]
pub mod fctl3;
#[doc = "FCTL4 (rw) register accessor: an alias for `Reg<FCTL4_SPEC>`"]
pub type FCTL4 = crate::Reg<fctl4::FCTL4_SPEC>;
#[doc = "FLASH Control 4"]
pub mod fctl4;
