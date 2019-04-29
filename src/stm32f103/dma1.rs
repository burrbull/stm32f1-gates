#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_ISR)"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_IFCR)"]
    pub ifcr: IFCR,
    #[doc = "0x08 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch1: CH,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch2: CH,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch3: CH,
    _reserved2: [u8; 4usize],
    #[doc = "0x44 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch4: CH,
    _reserved3: [u8; 4usize],
    #[doc = "0x58 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch5: CH,
    _reserved4: [u8; 4usize],
    #[doc = "0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch6: CH,
    _reserved5: [u8; 4usize],
    #[doc = "0x80 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch7: CH,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel configuration register (DMA_CCR)"]
    pub cr: self::ch::CR,
    #[doc = "0x04 - DMA channel 1 number of data register"]
    pub ndtr: self::ch::NDTR,
    #[doc = "0x08 - DMA channel 1 peripheral address register"]
    pub par: self::ch::PAR,
    #[doc = "0x0c - DMA channel 1 memory address register"]
    pub mar: self::ch::MAR,
}
#[doc = r" Register block"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub mod ch;
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub mod isr;
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub mod ifcr;
