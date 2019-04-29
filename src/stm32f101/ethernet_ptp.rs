#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
    pub ptptscr: PTPTSCR,
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    pub ptpssir: PTPSSIR,
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    pub ptptshr: PTPTSHR,
    #[doc = "0x0c - Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
    pub ptptslr: PTPTSLR,
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    pub ptptshur: PTPTSHUR,
    #[doc = "0x14 - Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
    pub ptptslur: PTPTSLUR,
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    pub ptptsar: PTPTSAR,
    #[doc = "0x1c - Ethernet PTP target time high register"]
    pub ptptthr: PTPTTHR,
    #[doc = "0x20 - Ethernet PTP target time low register"]
    pub ptpttlr: PTPTTLR,
}
#[doc = "Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
pub struct PTPTSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
pub mod ptptscr;
#[doc = "Ethernet PTP subsecond increment register"]
pub struct PTPSSIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssir;
#[doc = "Ethernet PTP time stamp high register"]
pub struct PTPTSHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptshr;
#[doc = "Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
pub struct PTPTSLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
pub mod ptptslr;
#[doc = "Ethernet PTP time stamp high update register"]
pub struct PTPTSHUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshur;
#[doc = "Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
pub struct PTPTSLUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
pub mod ptptslur;
#[doc = "Ethernet PTP time stamp addend register"]
pub struct PTPTSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsar;
#[doc = "Ethernet PTP target time high register"]
pub struct PTPTTHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP target time high register"]
pub mod ptptthr;
#[doc = "Ethernet PTP target time low register"]
pub struct PTPTTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttlr;
