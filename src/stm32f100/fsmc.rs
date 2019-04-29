#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: BCR,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: BTR,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr2: BCR,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr2: BTR,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr3: BCR,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr3: BTR,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr4: BCR,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr4: BTR,
    _reserved0: [u8; 228usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR,
    _reserved1: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr2: BWTR,
    _reserved2: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr3: BWTR,
    _reserved3: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr4: BWTR,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub struct BCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub struct BWTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr;
