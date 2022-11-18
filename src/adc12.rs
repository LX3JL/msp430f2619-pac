#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC12 Memory Control 0"]
    pub adc12mctl0: ADC12MCTL0,
    #[doc = "0x01 - ADC12 Memory Control 1"]
    pub adc12mctl1: ADC12MCTL1,
    #[doc = "0x02 - ADC12 Memory Control 2"]
    pub adc12mctl2: ADC12MCTL2,
    #[doc = "0x03 - ADC12 Memory Control 3"]
    pub adc12mctl3: ADC12MCTL3,
    #[doc = "0x04 - ADC12 Memory Control 4"]
    pub adc12mctl4: ADC12MCTL4,
    #[doc = "0x05 - ADC12 Memory Control 5"]
    pub adc12mctl5: ADC12MCTL5,
    #[doc = "0x06 - ADC12 Memory Control 6"]
    pub adc12mctl6: ADC12MCTL6,
    #[doc = "0x07 - ADC12 Memory Control 7"]
    pub adc12mctl7: ADC12MCTL7,
    #[doc = "0x08 - ADC12 Memory Control 8"]
    pub adc12mctl8: ADC12MCTL8,
    #[doc = "0x09 - ADC12 Memory Control 9"]
    pub adc12mctl9: ADC12MCTL9,
    #[doc = "0x0a - ADC12 Memory Control 10"]
    pub adc12mctl10: ADC12MCTL10,
    #[doc = "0x0b - ADC12 Memory Control 11"]
    pub adc12mctl11: ADC12MCTL11,
    #[doc = "0x0c - ADC12 Memory Control 12"]
    pub adc12mctl12: ADC12MCTL12,
    #[doc = "0x0d - ADC12 Memory Control 13"]
    pub adc12mctl13: ADC12MCTL13,
    #[doc = "0x0e - ADC12 Memory Control 14"]
    pub adc12mctl14: ADC12MCTL14,
    #[doc = "0x0f - ADC12 Memory Control 15"]
    pub adc12mctl15: ADC12MCTL15,
    _reserved16: [u8; 0xb0],
    #[doc = "0xc0 - ADC12 Conversion Memory 0"]
    pub adc12mem0: ADC12MEM0,
    #[doc = "0xc2 - ADC12 Conversion Memory 1"]
    pub adc12mem1: ADC12MEM1,
    #[doc = "0xc4 - ADC12 Conversion Memory 2"]
    pub adc12mem2: ADC12MEM2,
    #[doc = "0xc6 - ADC12 Conversion Memory 3"]
    pub adc12mem3: ADC12MEM3,
    #[doc = "0xc8 - ADC12 Conversion Memory 4"]
    pub adc12mem4: ADC12MEM4,
    #[doc = "0xca - ADC12 Conversion Memory 5"]
    pub adc12mem5: ADC12MEM5,
    #[doc = "0xcc - ADC12 Conversion Memory 6"]
    pub adc12mem6: ADC12MEM6,
    #[doc = "0xce - ADC12 Conversion Memory 7"]
    pub adc12mem7: ADC12MEM7,
    #[doc = "0xd0 - ADC12 Conversion Memory 8"]
    pub adc12mem8: ADC12MEM8,
    #[doc = "0xd2 - ADC12 Conversion Memory 9"]
    pub adc12mem9: ADC12MEM9,
    #[doc = "0xd4 - ADC12 Conversion Memory 10"]
    pub adc12mem10: ADC12MEM10,
    #[doc = "0xd6 - ADC12 Conversion Memory 11"]
    pub adc12mem11: ADC12MEM11,
    #[doc = "0xd8 - ADC12 Conversion Memory 12"]
    pub adc12mem12: ADC12MEM12,
    #[doc = "0xda - ADC12 Conversion Memory 13"]
    pub adc12mem13: ADC12MEM13,
    #[doc = "0xdc - ADC12 Conversion Memory 14"]
    pub adc12mem14: ADC12MEM14,
    #[doc = "0xde - ADC12 Conversion Memory 15"]
    pub adc12mem15: ADC12MEM15,
    _reserved32: [u8; 0x40],
    #[doc = "0x120 - ADC12 Control 0"]
    pub adc12ctl0: ADC12CTL0,
    #[doc = "0x122 - ADC12 Control 1"]
    pub adc12ctl1: ADC12CTL1,
    #[doc = "0x124 - ADC12 Interrupt Flag"]
    pub adc12ifg: ADC12IFG,
    #[doc = "0x126 - ADC12 Interrupt Enable"]
    pub adc12ie: ADC12IE,
    #[doc = "0x128 - ADC12 Interrupt Vector Word"]
    pub adc12iv: ADC12IV,
}
#[doc = "ADC12MCTL0 (rw) register accessor: an alias for `Reg<ADC12MCTL0_SPEC>`"]
pub type ADC12MCTL0 = crate::Reg<adc12mctl0::ADC12MCTL0_SPEC>;
#[doc = "ADC12 Memory Control 0"]
pub mod adc12mctl0;
#[doc = "ADC12MCTL1 (rw) register accessor: an alias for `Reg<ADC12MCTL1_SPEC>`"]
pub type ADC12MCTL1 = crate::Reg<adc12mctl1::ADC12MCTL1_SPEC>;
#[doc = "ADC12 Memory Control 1"]
pub mod adc12mctl1;
#[doc = "ADC12MCTL2 (rw) register accessor: an alias for `Reg<ADC12MCTL2_SPEC>`"]
pub type ADC12MCTL2 = crate::Reg<adc12mctl2::ADC12MCTL2_SPEC>;
#[doc = "ADC12 Memory Control 2"]
pub mod adc12mctl2;
#[doc = "ADC12MCTL3 (rw) register accessor: an alias for `Reg<ADC12MCTL3_SPEC>`"]
pub type ADC12MCTL3 = crate::Reg<adc12mctl3::ADC12MCTL3_SPEC>;
#[doc = "ADC12 Memory Control 3"]
pub mod adc12mctl3;
#[doc = "ADC12MCTL4 (rw) register accessor: an alias for `Reg<ADC12MCTL4_SPEC>`"]
pub type ADC12MCTL4 = crate::Reg<adc12mctl4::ADC12MCTL4_SPEC>;
#[doc = "ADC12 Memory Control 4"]
pub mod adc12mctl4;
#[doc = "ADC12MCTL5 (rw) register accessor: an alias for `Reg<ADC12MCTL5_SPEC>`"]
pub type ADC12MCTL5 = crate::Reg<adc12mctl5::ADC12MCTL5_SPEC>;
#[doc = "ADC12 Memory Control 5"]
pub mod adc12mctl5;
#[doc = "ADC12MCTL6 (rw) register accessor: an alias for `Reg<ADC12MCTL6_SPEC>`"]
pub type ADC12MCTL6 = crate::Reg<adc12mctl6::ADC12MCTL6_SPEC>;
#[doc = "ADC12 Memory Control 6"]
pub mod adc12mctl6;
#[doc = "ADC12MCTL7 (rw) register accessor: an alias for `Reg<ADC12MCTL7_SPEC>`"]
pub type ADC12MCTL7 = crate::Reg<adc12mctl7::ADC12MCTL7_SPEC>;
#[doc = "ADC12 Memory Control 7"]
pub mod adc12mctl7;
#[doc = "ADC12MCTL8 (rw) register accessor: an alias for `Reg<ADC12MCTL8_SPEC>`"]
pub type ADC12MCTL8 = crate::Reg<adc12mctl8::ADC12MCTL8_SPEC>;
#[doc = "ADC12 Memory Control 8"]
pub mod adc12mctl8;
#[doc = "ADC12MCTL9 (rw) register accessor: an alias for `Reg<ADC12MCTL9_SPEC>`"]
pub type ADC12MCTL9 = crate::Reg<adc12mctl9::ADC12MCTL9_SPEC>;
#[doc = "ADC12 Memory Control 9"]
pub mod adc12mctl9;
#[doc = "ADC12MCTL10 (rw) register accessor: an alias for `Reg<ADC12MCTL10_SPEC>`"]
pub type ADC12MCTL10 = crate::Reg<adc12mctl10::ADC12MCTL10_SPEC>;
#[doc = "ADC12 Memory Control 10"]
pub mod adc12mctl10;
#[doc = "ADC12MCTL11 (rw) register accessor: an alias for `Reg<ADC12MCTL11_SPEC>`"]
pub type ADC12MCTL11 = crate::Reg<adc12mctl11::ADC12MCTL11_SPEC>;
#[doc = "ADC12 Memory Control 11"]
pub mod adc12mctl11;
#[doc = "ADC12MCTL12 (rw) register accessor: an alias for `Reg<ADC12MCTL12_SPEC>`"]
pub type ADC12MCTL12 = crate::Reg<adc12mctl12::ADC12MCTL12_SPEC>;
#[doc = "ADC12 Memory Control 12"]
pub mod adc12mctl12;
#[doc = "ADC12MCTL13 (rw) register accessor: an alias for `Reg<ADC12MCTL13_SPEC>`"]
pub type ADC12MCTL13 = crate::Reg<adc12mctl13::ADC12MCTL13_SPEC>;
#[doc = "ADC12 Memory Control 13"]
pub mod adc12mctl13;
#[doc = "ADC12MCTL14 (rw) register accessor: an alias for `Reg<ADC12MCTL14_SPEC>`"]
pub type ADC12MCTL14 = crate::Reg<adc12mctl14::ADC12MCTL14_SPEC>;
#[doc = "ADC12 Memory Control 14"]
pub mod adc12mctl14;
#[doc = "ADC12MCTL15 (rw) register accessor: an alias for `Reg<ADC12MCTL15_SPEC>`"]
pub type ADC12MCTL15 = crate::Reg<adc12mctl15::ADC12MCTL15_SPEC>;
#[doc = "ADC12 Memory Control 15"]
pub mod adc12mctl15;
#[doc = "ADC12MEM0 (rw) register accessor: an alias for `Reg<ADC12MEM0_SPEC>`"]
pub type ADC12MEM0 = crate::Reg<adc12mem0::ADC12MEM0_SPEC>;
#[doc = "ADC12 Conversion Memory 0"]
pub mod adc12mem0;
#[doc = "ADC12MEM1 (rw) register accessor: an alias for `Reg<ADC12MEM1_SPEC>`"]
pub type ADC12MEM1 = crate::Reg<adc12mem1::ADC12MEM1_SPEC>;
#[doc = "ADC12 Conversion Memory 1"]
pub mod adc12mem1;
#[doc = "ADC12MEM2 (rw) register accessor: an alias for `Reg<ADC12MEM2_SPEC>`"]
pub type ADC12MEM2 = crate::Reg<adc12mem2::ADC12MEM2_SPEC>;
#[doc = "ADC12 Conversion Memory 2"]
pub mod adc12mem2;
#[doc = "ADC12MEM3 (rw) register accessor: an alias for `Reg<ADC12MEM3_SPEC>`"]
pub type ADC12MEM3 = crate::Reg<adc12mem3::ADC12MEM3_SPEC>;
#[doc = "ADC12 Conversion Memory 3"]
pub mod adc12mem3;
#[doc = "ADC12MEM4 (rw) register accessor: an alias for `Reg<ADC12MEM4_SPEC>`"]
pub type ADC12MEM4 = crate::Reg<adc12mem4::ADC12MEM4_SPEC>;
#[doc = "ADC12 Conversion Memory 4"]
pub mod adc12mem4;
#[doc = "ADC12MEM5 (rw) register accessor: an alias for `Reg<ADC12MEM5_SPEC>`"]
pub type ADC12MEM5 = crate::Reg<adc12mem5::ADC12MEM5_SPEC>;
#[doc = "ADC12 Conversion Memory 5"]
pub mod adc12mem5;
#[doc = "ADC12MEM6 (rw) register accessor: an alias for `Reg<ADC12MEM6_SPEC>`"]
pub type ADC12MEM6 = crate::Reg<adc12mem6::ADC12MEM6_SPEC>;
#[doc = "ADC12 Conversion Memory 6"]
pub mod adc12mem6;
#[doc = "ADC12MEM7 (rw) register accessor: an alias for `Reg<ADC12MEM7_SPEC>`"]
pub type ADC12MEM7 = crate::Reg<adc12mem7::ADC12MEM7_SPEC>;
#[doc = "ADC12 Conversion Memory 7"]
pub mod adc12mem7;
#[doc = "ADC12MEM8 (rw) register accessor: an alias for `Reg<ADC12MEM8_SPEC>`"]
pub type ADC12MEM8 = crate::Reg<adc12mem8::ADC12MEM8_SPEC>;
#[doc = "ADC12 Conversion Memory 8"]
pub mod adc12mem8;
#[doc = "ADC12MEM9 (rw) register accessor: an alias for `Reg<ADC12MEM9_SPEC>`"]
pub type ADC12MEM9 = crate::Reg<adc12mem9::ADC12MEM9_SPEC>;
#[doc = "ADC12 Conversion Memory 9"]
pub mod adc12mem9;
#[doc = "ADC12MEM10 (rw) register accessor: an alias for `Reg<ADC12MEM10_SPEC>`"]
pub type ADC12MEM10 = crate::Reg<adc12mem10::ADC12MEM10_SPEC>;
#[doc = "ADC12 Conversion Memory 10"]
pub mod adc12mem10;
#[doc = "ADC12MEM11 (rw) register accessor: an alias for `Reg<ADC12MEM11_SPEC>`"]
pub type ADC12MEM11 = crate::Reg<adc12mem11::ADC12MEM11_SPEC>;
#[doc = "ADC12 Conversion Memory 11"]
pub mod adc12mem11;
#[doc = "ADC12MEM12 (rw) register accessor: an alias for `Reg<ADC12MEM12_SPEC>`"]
pub type ADC12MEM12 = crate::Reg<adc12mem12::ADC12MEM12_SPEC>;
#[doc = "ADC12 Conversion Memory 12"]
pub mod adc12mem12;
#[doc = "ADC12MEM13 (rw) register accessor: an alias for `Reg<ADC12MEM13_SPEC>`"]
pub type ADC12MEM13 = crate::Reg<adc12mem13::ADC12MEM13_SPEC>;
#[doc = "ADC12 Conversion Memory 13"]
pub mod adc12mem13;
#[doc = "ADC12MEM14 (rw) register accessor: an alias for `Reg<ADC12MEM14_SPEC>`"]
pub type ADC12MEM14 = crate::Reg<adc12mem14::ADC12MEM14_SPEC>;
#[doc = "ADC12 Conversion Memory 14"]
pub mod adc12mem14;
#[doc = "ADC12MEM15 (rw) register accessor: an alias for `Reg<ADC12MEM15_SPEC>`"]
pub type ADC12MEM15 = crate::Reg<adc12mem15::ADC12MEM15_SPEC>;
#[doc = "ADC12 Conversion Memory 15"]
pub mod adc12mem15;
#[doc = "ADC12CTL0 (rw) register accessor: an alias for `Reg<ADC12CTL0_SPEC>`"]
pub type ADC12CTL0 = crate::Reg<adc12ctl0::ADC12CTL0_SPEC>;
#[doc = "ADC12 Control 0"]
pub mod adc12ctl0;
#[doc = "ADC12CTL1 (rw) register accessor: an alias for `Reg<ADC12CTL1_SPEC>`"]
pub type ADC12CTL1 = crate::Reg<adc12ctl1::ADC12CTL1_SPEC>;
#[doc = "ADC12 Control 1"]
pub mod adc12ctl1;
#[doc = "ADC12IFG (rw) register accessor: an alias for `Reg<ADC12IFG_SPEC>`"]
pub type ADC12IFG = crate::Reg<adc12ifg::ADC12IFG_SPEC>;
#[doc = "ADC12 Interrupt Flag"]
pub mod adc12ifg;
#[doc = "ADC12IE (rw) register accessor: an alias for `Reg<ADC12IE_SPEC>`"]
pub type ADC12IE = crate::Reg<adc12ie::ADC12IE_SPEC>;
#[doc = "ADC12 Interrupt Enable"]
pub mod adc12ie;
#[doc = "ADC12IV (rw) register accessor: an alias for `Reg<ADC12IV_SPEC>`"]
pub type ADC12IV = crate::Reg<adc12iv::ADC12IV_SPEC>;
#[doc = "ADC12 Interrupt Vector Word"]
pub mod adc12iv;
