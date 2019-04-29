#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x04 - CEC own address register"]
    pub oar: OAR,
    #[doc = "0x08 - Rx Data Register"]
    pub pres: PRES,
    #[doc = "0x0c - CEC error status register"]
    pub esr: ESR,
    #[doc = "0x10 - CEC control and status register"]
    pub csr: CSR,
    #[doc = "0x14 - CEC Tx data register"]
    pub txd: TXD,
    #[doc = "0x18 - CEC Rx data register"]
    pub rxd: RXD,
}
#[doc = "configuration register"]
pub struct CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "CEC own address register"]
pub struct OAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CEC own address register"]
pub mod oar;
#[doc = "Rx Data Register"]
pub struct PRES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Data Register"]
pub mod pres;
#[doc = "CEC error status register"]
pub struct ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CEC error status register"]
pub mod esr;
#[doc = "CEC control and status register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CEC control and status register"]
pub mod csr;
#[doc = "CEC Tx data register"]
pub struct TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CEC Tx data register"]
pub mod txd;
#[doc = "CEC Rx data register"]
pub struct RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CEC Rx data register"]
pub mod rxd;
