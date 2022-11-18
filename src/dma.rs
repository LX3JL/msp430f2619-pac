#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Module Control 0"]
    pub dmactl0: DMACTL0,
    #[doc = "0x02 - DMA Module Control 1"]
    pub dmactl1: DMACTL1,
    #[doc = "0x04 - DMA Interrupt Vector Word"]
    pub dmaiv: DMAIV,
    _reserved3: [u8; 0xa8],
    #[doc = "0xae - DMA Channel 0 Control"]
    pub dma0ctl: DMA0CTL,
    #[doc = "0xb0 - DMA Channel 0 Source Address"]
    pub dma0sal: DMA0SAL,
    _reserved5: [u8; 0x02],
    #[doc = "0xb4 - DMA Channel 0 Destination Address"]
    pub dma0dal: DMA0DAL,
    _reserved6: [u8; 0x02],
    #[doc = "0xb8 - DMA Channel 0 Transfer Size"]
    pub dma0sz: DMA0SZ,
    #[doc = "0xba - DMA Channel 1 Control"]
    pub dma1ctl: DMA1CTL,
    #[doc = "0xbc - DMA Channel 1 Source Address"]
    pub dma1sal: DMA1SAL,
    _reserved9: [u8; 0x02],
    #[doc = "0xc0 - DMA Channel 1 Destination Address"]
    pub dma1dal: DMA1DAL,
    _reserved10: [u8; 0x02],
    #[doc = "0xc4 - DMA Channel 1 Transfer Size"]
    pub dma1sz: DMA1SZ,
    #[doc = "0xc6 - DMA Channel 2 Control"]
    pub dma2ctl: DMA2CTL,
    #[doc = "0xc8 - DMA Channel 2 Source Address"]
    pub dma2sal: DMA2SAL,
    _reserved13: [u8; 0x02],
    #[doc = "0xcc - DMA Channel 2 Destination Address"]
    pub dma2dal: DMA2DAL,
    _reserved14: [u8; 0x02],
    #[doc = "0xd0 - DMA Channel 2 Transfer Size"]
    pub dma2sz: DMA2SZ,
}
#[doc = "DMACTL0 (rw) register accessor: an alias for `Reg<DMACTL0_SPEC>`"]
pub type DMACTL0 = crate::Reg<dmactl0::DMACTL0_SPEC>;
#[doc = "DMA Module Control 0"]
pub mod dmactl0;
#[doc = "DMACTL1 (rw) register accessor: an alias for `Reg<DMACTL1_SPEC>`"]
pub type DMACTL1 = crate::Reg<dmactl1::DMACTL1_SPEC>;
#[doc = "DMA Module Control 1"]
pub mod dmactl1;
#[doc = "DMAIV (rw) register accessor: an alias for `Reg<DMAIV_SPEC>`"]
pub type DMAIV = crate::Reg<dmaiv::DMAIV_SPEC>;
#[doc = "DMA Interrupt Vector Word"]
pub mod dmaiv;
#[doc = "DMA0CTL (rw) register accessor: an alias for `Reg<DMA0CTL_SPEC>`"]
pub type DMA0CTL = crate::Reg<dma0ctl::DMA0CTL_SPEC>;
#[doc = "DMA Channel 0 Control"]
pub mod dma0ctl;
#[doc = "DMA0SAL (rw) register accessor: an alias for `Reg<DMA0SAL_SPEC>`"]
pub type DMA0SAL = crate::Reg<dma0sal::DMA0SAL_SPEC>;
#[doc = "DMA Channel 0 Source Address"]
pub mod dma0sal;
#[doc = "DMA0DAL (rw) register accessor: an alias for `Reg<DMA0DAL_SPEC>`"]
pub type DMA0DAL = crate::Reg<dma0dal::DMA0DAL_SPEC>;
#[doc = "DMA Channel 0 Destination Address"]
pub mod dma0dal;
#[doc = "DMA0SZ (rw) register accessor: an alias for `Reg<DMA0SZ_SPEC>`"]
pub type DMA0SZ = crate::Reg<dma0sz::DMA0SZ_SPEC>;
#[doc = "DMA Channel 0 Transfer Size"]
pub mod dma0sz;
#[doc = "DMA1CTL (rw) register accessor: an alias for `Reg<DMA1CTL_SPEC>`"]
pub type DMA1CTL = crate::Reg<dma1ctl::DMA1CTL_SPEC>;
#[doc = "DMA Channel 1 Control"]
pub mod dma1ctl;
#[doc = "DMA1SAL (rw) register accessor: an alias for `Reg<DMA1SAL_SPEC>`"]
pub type DMA1SAL = crate::Reg<dma1sal::DMA1SAL_SPEC>;
#[doc = "DMA Channel 1 Source Address"]
pub mod dma1sal;
#[doc = "DMA1DAL (rw) register accessor: an alias for `Reg<DMA1DAL_SPEC>`"]
pub type DMA1DAL = crate::Reg<dma1dal::DMA1DAL_SPEC>;
#[doc = "DMA Channel 1 Destination Address"]
pub mod dma1dal;
#[doc = "DMA1SZ (rw) register accessor: an alias for `Reg<DMA1SZ_SPEC>`"]
pub type DMA1SZ = crate::Reg<dma1sz::DMA1SZ_SPEC>;
#[doc = "DMA Channel 1 Transfer Size"]
pub mod dma1sz;
#[doc = "DMA2CTL (rw) register accessor: an alias for `Reg<DMA2CTL_SPEC>`"]
pub type DMA2CTL = crate::Reg<dma2ctl::DMA2CTL_SPEC>;
#[doc = "DMA Channel 2 Control"]
pub mod dma2ctl;
#[doc = "DMA2SAL (rw) register accessor: an alias for `Reg<DMA2SAL_SPEC>`"]
pub type DMA2SAL = crate::Reg<dma2sal::DMA2SAL_SPEC>;
#[doc = "DMA Channel 2 Source Address"]
pub mod dma2sal;
#[doc = "DMA2DAL (rw) register accessor: an alias for `Reg<DMA2DAL_SPEC>`"]
pub type DMA2DAL = crate::Reg<dma2dal::DMA2DAL_SPEC>;
#[doc = "DMA Channel 2 Destination Address"]
pub mod dma2dal;
#[doc = "DMA2SZ (rw) register accessor: an alias for `Reg<DMA2SZ_SPEC>`"]
pub type DMA2SZ = crate::Reg<dma2sz::DMA2SZ_SPEC>;
#[doc = "DMA Channel 2 Transfer Size"]
pub mod dma2sz;
