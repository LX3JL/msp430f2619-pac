#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Resistor Enable"]
    pub p3ren: P3REN,
    #[doc = "0x01 - Port 4 Resistor Enable"]
    pub p4ren: P4REN,
    _reserved2: [u8; 0x06],
    #[doc = "0x08 - Port 3 Input"]
    pub p3in: P3IN,
    #[doc = "0x09 - Port 3 Output"]
    pub p3out: P3OUT,
    #[doc = "0x0a - Port 3 Direction"]
    pub p3dir: P3DIR,
    #[doc = "0x0b - Port 3 Selection"]
    pub p3sel: P3SEL,
    #[doc = "0x0c - Port 4 Input"]
    pub p4in: P4IN,
    #[doc = "0x0d - Port 4 Output"]
    pub p4out: P4OUT,
    #[doc = "0x0e - Port 4 Direction"]
    pub p4dir: P4DIR,
    #[doc = "0x0f - Port 4 Selection"]
    pub p4sel: P4SEL,
}
#[doc = "P3REN (rw) register accessor: an alias for `Reg<P3REN_SPEC>`"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P4REN (rw) register accessor: an alias for `Reg<P4REN_SPEC>`"]
pub type P4REN = crate::Reg<p4ren::P4REN_SPEC>;
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "P3IN (rw) register accessor: an alias for `Reg<P3IN_SPEC>`"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT (rw) register accessor: an alias for `Reg<P3OUT_SPEC>`"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR (rw) register accessor: an alias for `Reg<P3DIR_SPEC>`"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3SEL (rw) register accessor: an alias for `Reg<P3SEL_SPEC>`"]
pub type P3SEL = crate::Reg<p3sel::P3SEL_SPEC>;
#[doc = "Port 3 Selection"]
pub mod p3sel;
#[doc = "P4IN (rw) register accessor: an alias for `Reg<P4IN_SPEC>`"]
pub type P4IN = crate::Reg<p4in::P4IN_SPEC>;
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "P4OUT (rw) register accessor: an alias for `Reg<P4OUT_SPEC>`"]
pub type P4OUT = crate::Reg<p4out::P4OUT_SPEC>;
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "P4DIR (rw) register accessor: an alias for `Reg<P4DIR_SPEC>`"]
pub type P4DIR = crate::Reg<p4dir::P4DIR_SPEC>;
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "P4SEL (rw) register accessor: an alias for `Reg<P4SEL_SPEC>`"]
pub type P4SEL = crate::Reg<p4sel::P4SEL_SPEC>;
#[doc = "Port 4 Selection"]
pub mod p4sel;
