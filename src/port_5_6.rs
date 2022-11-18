#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 5 Resistor Enable"]
    pub p5ren: P5REN,
    #[doc = "0x01 - Port 6 Resistor Enable"]
    pub p6ren: P6REN,
    _reserved2: [u8; 0x1c],
    #[doc = "0x1e - Port 5 Input"]
    pub p5in: P5IN,
    #[doc = "0x1f - Port 5 Output"]
    pub p5out: P5OUT,
    #[doc = "0x20 - Port 5 Direction"]
    pub p5dir: P5DIR,
    #[doc = "0x21 - Port 5 Selection"]
    pub p5sel: P5SEL,
    #[doc = "0x22 - Port 6 Input"]
    pub p6in: P6IN,
    #[doc = "0x23 - Port 6 Output"]
    pub p6out: P6OUT,
    #[doc = "0x24 - Port 6 Direction"]
    pub p6dir: P6DIR,
    #[doc = "0x25 - Port 6 Selection"]
    pub p6sel: P6SEL,
}
#[doc = "P5REN (rw) register accessor: an alias for `Reg<P5REN_SPEC>`"]
pub type P5REN = crate::Reg<p5ren::P5REN_SPEC>;
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "P6REN (rw) register accessor: an alias for `Reg<P6REN_SPEC>`"]
pub type P6REN = crate::Reg<p6ren::P6REN_SPEC>;
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "P5IN (rw) register accessor: an alias for `Reg<P5IN_SPEC>`"]
pub type P5IN = crate::Reg<p5in::P5IN_SPEC>;
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "P5OUT (rw) register accessor: an alias for `Reg<P5OUT_SPEC>`"]
pub type P5OUT = crate::Reg<p5out::P5OUT_SPEC>;
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "P5DIR (rw) register accessor: an alias for `Reg<P5DIR_SPEC>`"]
pub type P5DIR = crate::Reg<p5dir::P5DIR_SPEC>;
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "P5SEL (rw) register accessor: an alias for `Reg<P5SEL_SPEC>`"]
pub type P5SEL = crate::Reg<p5sel::P5SEL_SPEC>;
#[doc = "Port 5 Selection"]
pub mod p5sel;
#[doc = "P6IN (rw) register accessor: an alias for `Reg<P6IN_SPEC>`"]
pub type P6IN = crate::Reg<p6in::P6IN_SPEC>;
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "P6OUT (rw) register accessor: an alias for `Reg<P6OUT_SPEC>`"]
pub type P6OUT = crate::Reg<p6out::P6OUT_SPEC>;
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "P6DIR (rw) register accessor: an alias for `Reg<P6DIR_SPEC>`"]
pub type P6DIR = crate::Reg<p6dir::P6DIR_SPEC>;
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "P6SEL (rw) register accessor: an alias for `Reg<P6SEL_SPEC>`"]
pub type P6SEL = crate::Reg<p6sel::P6SEL_SPEC>;
#[doc = "Port 6 Selection"]
pub mod p6sel;
