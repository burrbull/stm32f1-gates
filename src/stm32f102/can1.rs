#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN_MCR"]
    pub mcr: MCR,
    #[doc = "0x04 - CAN_MSR"]
    pub msr: MSR,
    #[doc = "0x08 - CAN_TSR"]
    pub tsr: TSR,
    #[doc = "0x0c - CAN_RF0R"]
    pub rfr: [RFR; 2],
    #[doc = "0x14 - CAN_IER"]
    pub ier: IER,
    #[doc = "0x18 - CAN_ESR"]
    pub esr: ESR,
    #[doc = "0x1c - CAN_BTR"]
    pub btr: BTR,
    _reserved0: [u8; 352usize],
    #[doc = "0x180 - CAN Transmit cluster"]
    pub tx: [TX; 3],
    #[doc = "0x1b0 - CAN Receive cluster"]
    pub rx: [RX; 2],
    _reserved1: [u8; 48usize],
    #[doc = "0x200 - CAN_FMR"]
    pub fmr: FMR,
    #[doc = "0x204 - CAN_FM1R"]
    pub fm1r: FM1R,
    _reserved2: [u8; 4usize],
    #[doc = "0x20c - CAN_FS1R"]
    pub fs1r: FS1R,
    _reserved3: [u8; 4usize],
    #[doc = "0x214 - CAN_FFA1R"]
    pub ffa1r: FFA1R,
    _reserved4: [u8; 4usize],
    #[doc = "0x21c - CAN_FA1R"]
    pub fa1r: FA1R,
    _reserved5: [u8; 32usize],
    #[doc = "0x240 - CAN Filter Bank cluster"]
    pub fb: [FB; 14],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - CAN_TI0R"]
    pub tir: self::tx::TIR,
    #[doc = "0x04 - CAN_TDT0R"]
    pub tdtr: self::tx::TDTR,
    #[doc = "0x08 - CAN_TDL0R"]
    pub tdlr: self::tx::TDLR,
    #[doc = "0x0c - CAN_TDH0R"]
    pub tdhr: self::tx::TDHR,
}
#[doc = r" Register block"]
#[doc = "CAN Transmit cluster"]
pub mod tx;
#[doc = r" Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - CAN_RI0R"]
    pub rir: self::rx::RIR,
    #[doc = "0x04 - CAN_RDT0R"]
    pub rdtr: self::rx::RDTR,
    #[doc = "0x08 - CAN_RDL0R"]
    pub rdlr: self::rx::RDLR,
    #[doc = "0x0c - CAN_RDH0R"]
    pub rdhr: self::rx::RDHR,
}
#[doc = r" Register block"]
#[doc = "CAN Receive cluster"]
pub mod rx;
#[doc = r" Register block"]
#[repr(C)]
pub struct FB {
    #[doc = "0x00 - Filter bank 0 register 1"]
    pub fr1: self::fb::FR1,
    #[doc = "0x04 - Filter bank 0 register 2"]
    pub fr2: self::fb::FR2,
}
#[doc = r" Register block"]
#[doc = "CAN Filter Bank cluster"]
pub mod fb;
#[doc = "CAN_MCR"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_MCR"]
pub mod mcr;
#[doc = "CAN_MSR"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_MSR"]
pub mod msr;
#[doc = "CAN_TSR"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TSR"]
pub mod tsr;
#[doc = "CAN_RF0R"]
pub struct RFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RF0R"]
pub mod rfr;
#[doc = "CAN_IER"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_IER"]
pub mod ier;
#[doc = "CAN_ESR"]
pub struct ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_ESR"]
pub mod esr;
#[doc = "CAN_BTR"]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_BTR"]
pub mod btr;
#[doc = "CAN_FMR"]
pub struct FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FMR"]
pub mod fmr;
#[doc = "CAN_FM1R"]
pub struct FM1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FM1R"]
pub mod fm1r;
#[doc = "CAN_FS1R"]
pub struct FS1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FS1R"]
pub mod fs1r;
#[doc = "CAN_FFA1R"]
pub struct FFA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FFA1R"]
pub mod ffa1r;
#[doc = "CAN_FA1R"]
pub struct FA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FA1R"]
pub mod fa1r;
