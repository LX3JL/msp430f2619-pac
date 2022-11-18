#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer B Interrupt Vector Word"]
    pub tbiv: TBIV,
    _reserved1: [u8; 0x60],
    #[doc = "0x62 - Timer B Control"]
    pub tbctl: TBCTL,
    #[doc = "0x64 - Timer B Capture/Compare Control 0"]
    pub tbcctl0: TBCCTL0,
    #[doc = "0x66 - Timer B Capture/Compare Control 1"]
    pub tbcctl1: TBCCTL1,
    #[doc = "0x68 - Timer B Capture/Compare Control 2"]
    pub tbcctl2: TBCCTL2,
    #[doc = "0x6a - Timer B Capture/Compare Control 3"]
    pub tbcctl3: TBCCTL3,
    #[doc = "0x6c - Timer B Capture/Compare Control 4"]
    pub tbcctl4: TBCCTL4,
    #[doc = "0x6e - Timer B Capture/Compare Control 5"]
    pub tbcctl5: TBCCTL5,
    #[doc = "0x70 - Timer B Capture/Compare Control 6"]
    pub tbcctl6: TBCCTL6,
    #[doc = "0x72 - Timer B Counter Register"]
    pub tbr: TBR,
    #[doc = "0x74 - Timer B Capture/Compare 0"]
    pub tbccr0: TBCCR0,
    #[doc = "0x76 - Timer B Capture/Compare 1"]
    pub tbccr1: TBCCR1,
    #[doc = "0x78 - Timer B Capture/Compare 2"]
    pub tbccr2: TBCCR2,
    #[doc = "0x7a - Timer B Capture/Compare 3"]
    pub tbccr3: TBCCR3,
    #[doc = "0x7c - Timer B Capture/Compare 4"]
    pub tbccr4: TBCCR4,
    #[doc = "0x7e - Timer B Capture/Compare 5"]
    pub tbccr5: TBCCR5,
    #[doc = "0x80 - Timer B Capture/Compare 6"]
    pub tbccr6: TBCCR6,
}
#[doc = "TBIV (rw) register accessor: an alias for `Reg<TBIV_SPEC>`"]
pub type TBIV = crate::Reg<tbiv::TBIV_SPEC>;
#[doc = "Timer B Interrupt Vector Word"]
pub mod tbiv;
#[doc = "TBCTL (rw) register accessor: an alias for `Reg<TBCTL_SPEC>`"]
pub type TBCTL = crate::Reg<tbctl::TBCTL_SPEC>;
#[doc = "Timer B Control"]
pub mod tbctl;
#[doc = "TBCCTL0 (rw) register accessor: an alias for `Reg<TBCCTL0_SPEC>`"]
pub type TBCCTL0 = crate::Reg<tbcctl0::TBCCTL0_SPEC>;
#[doc = "Timer B Capture/Compare Control 0"]
pub mod tbcctl0;
#[doc = "TBCCTL1 (rw) register accessor: an alias for `Reg<TBCCTL1_SPEC>`"]
pub type TBCCTL1 = crate::Reg<tbcctl1::TBCCTL1_SPEC>;
#[doc = "Timer B Capture/Compare Control 1"]
pub mod tbcctl1;
#[doc = "TBCCTL2 (rw) register accessor: an alias for `Reg<TBCCTL2_SPEC>`"]
pub type TBCCTL2 = crate::Reg<tbcctl2::TBCCTL2_SPEC>;
#[doc = "Timer B Capture/Compare Control 2"]
pub mod tbcctl2;
#[doc = "TBCCTL3 (rw) register accessor: an alias for `Reg<TBCCTL3_SPEC>`"]
pub type TBCCTL3 = crate::Reg<tbcctl3::TBCCTL3_SPEC>;
#[doc = "Timer B Capture/Compare Control 3"]
pub mod tbcctl3;
#[doc = "TBCCTL4 (rw) register accessor: an alias for `Reg<TBCCTL4_SPEC>`"]
pub type TBCCTL4 = crate::Reg<tbcctl4::TBCCTL4_SPEC>;
#[doc = "Timer B Capture/Compare Control 4"]
pub mod tbcctl4;
#[doc = "TBCCTL5 (rw) register accessor: an alias for `Reg<TBCCTL5_SPEC>`"]
pub type TBCCTL5 = crate::Reg<tbcctl5::TBCCTL5_SPEC>;
#[doc = "Timer B Capture/Compare Control 5"]
pub mod tbcctl5;
#[doc = "TBCCTL6 (rw) register accessor: an alias for `Reg<TBCCTL6_SPEC>`"]
pub type TBCCTL6 = crate::Reg<tbcctl6::TBCCTL6_SPEC>;
#[doc = "Timer B Capture/Compare Control 6"]
pub mod tbcctl6;
#[doc = "TBR (rw) register accessor: an alias for `Reg<TBR_SPEC>`"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "Timer B Counter Register"]
pub mod tbr;
#[doc = "TBCCR0 (rw) register accessor: an alias for `Reg<TBCCR0_SPEC>`"]
pub type TBCCR0 = crate::Reg<tbccr0::TBCCR0_SPEC>;
#[doc = "Timer B Capture/Compare 0"]
pub mod tbccr0;
#[doc = "TBCCR1 (rw) register accessor: an alias for `Reg<TBCCR1_SPEC>`"]
pub type TBCCR1 = crate::Reg<tbccr1::TBCCR1_SPEC>;
#[doc = "Timer B Capture/Compare 1"]
pub mod tbccr1;
#[doc = "TBCCR2 (rw) register accessor: an alias for `Reg<TBCCR2_SPEC>`"]
pub type TBCCR2 = crate::Reg<tbccr2::TBCCR2_SPEC>;
#[doc = "Timer B Capture/Compare 2"]
pub mod tbccr2;
#[doc = "TBCCR3 (rw) register accessor: an alias for `Reg<TBCCR3_SPEC>`"]
pub type TBCCR3 = crate::Reg<tbccr3::TBCCR3_SPEC>;
#[doc = "Timer B Capture/Compare 3"]
pub mod tbccr3;
#[doc = "TBCCR4 (rw) register accessor: an alias for `Reg<TBCCR4_SPEC>`"]
pub type TBCCR4 = crate::Reg<tbccr4::TBCCR4_SPEC>;
#[doc = "Timer B Capture/Compare 4"]
pub mod tbccr4;
#[doc = "TBCCR5 (rw) register accessor: an alias for `Reg<TBCCR5_SPEC>`"]
pub type TBCCR5 = crate::Reg<tbccr5::TBCCR5_SPEC>;
#[doc = "Timer B Capture/Compare 5"]
pub mod tbccr5;
#[doc = "TBCCR6 (rw) register accessor: an alias for `Reg<TBCCR6_SPEC>`"]
pub type TBCCR6 = crate::Reg<tbccr6::TBCCR6_SPEC>;
#[doc = "Timer B Capture/Compare 6"]
pub mod tbccr6;
