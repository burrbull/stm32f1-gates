#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: BCR1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: BTR1,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: BCR2,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    pub btr2: BTR2,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    pub bcr3: BCR3,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    pub btr3: BTR3,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bcr4: BCR4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub btr4: BTR4,
    _reserved0: [u8; 228usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR1,
    _reserved1: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub bwtr2: BWTR2,
    _reserved2: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub bwtr3: BWTR3,
    _reserved3: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bwtr4: BWTR4,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub struct BCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub struct BTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub struct BCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub struct BTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub struct BCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub struct BTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub struct BCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub struct BTR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub struct BWTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub struct BWTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub struct BWTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub struct BWTR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
