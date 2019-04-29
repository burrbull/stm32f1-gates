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
    _reserved0: [u8; 64usize],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub pcr2: PCR,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub sr2: SR,
    #[doc = "0x68 - Common memory space timing register 2"]
    pub pmem2: PMEM,
    #[doc = "0x6c - Attribute memory space timing register 2"]
    pub patt2: PATT,
    _reserved1: [u8; 4usize],
    #[doc = "0x74 - ECC result register 2"]
    pub eccr2: ECCR,
    _reserved2: [u8; 8usize],
    #[doc = "0x80 - PC Card/NAND Flash control register 2"]
    pub pcr3: PCR,
    #[doc = "0x84 - FIFO status and interrupt register 2"]
    pub sr3: SR,
    #[doc = "0x88 - Common memory space timing register 2"]
    pub pmem3: PMEM,
    #[doc = "0x8c - Attribute memory space timing register 2"]
    pub patt3: PATT,
    _reserved3: [u8; 4usize],
    #[doc = "0x94 - ECC result register 2"]
    pub eccr3: ECCR,
    _reserved4: [u8; 8usize],
    #[doc = "0xa0 - PC Card/NAND Flash control register 2"]
    pub pcr4: PCR,
    #[doc = "0xa4 - FIFO status and interrupt register 2"]
    pub sr4: SR,
    #[doc = "0xa8 - Common memory space timing register 2"]
    pub pmem4: PMEM,
    #[doc = "0xac - Attribute memory space timing register 2"]
    pub patt4: PATT,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub pio4: PIO4,
    _reserved5: [u8; 80usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR,
    _reserved6: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr2: BWTR,
    _reserved7: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr3: BWTR,
    _reserved8: [u8; 4usize],
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
#[doc = "PC Card/NAND Flash control register 2"]
pub struct PCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr;
#[doc = "FIFO status and interrupt register 2"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register 2"]
pub mod sr;
#[doc = "Common memory space timing register 2"]
pub struct PMEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Common memory space timing register 2"]
pub mod pmem;
#[doc = "Attribute memory space timing register 2"]
pub struct PATT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register 2"]
pub mod patt;
#[doc = "ECC result register 2"]
pub struct ECCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC result register 2"]
pub mod eccr;
#[doc = "I/O space timing register 4"]
pub struct PIO4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub struct BWTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr;
