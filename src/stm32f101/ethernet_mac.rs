#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register (ETH_MACCR)"]
    pub maccr: MACCR,
    #[doc = "0x04 - Ethernet MAC frame filter register (ETH_MACCFFR)"]
    pub macffr: MACFFR,
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    pub machthr: MACHTHR,
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    pub machtlr: MACHTLR,
    #[doc = "0x10 - Ethernet MAC MII address register (ETH_MACMIIAR)"]
    pub macmiiar: MACMIIAR,
    #[doc = "0x14 - Ethernet MAC MII data register (ETH_MACMIIDR)"]
    pub macmiidr: MACMIIDR,
    #[doc = "0x18 - Ethernet MAC flow control register (ETH_MACFCR)"]
    pub macfcr: MACFCR,
    #[doc = "0x1c - Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
    pub macvlantr: MACVLANTR,
    _reserved0: [u8; 8usize],
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
    pub macrwuffr: MACRWUFFR,
    #[doc = "0x2c - Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
    pub macpmtcsr: MACPMTCSR,
    _reserved1: [u8; 8usize],
    #[doc = "0x38 - Ethernet MAC interrupt status register (ETH_MACSR)"]
    pub macsr: MACSR,
    #[doc = "0x3c - Ethernet MAC interrupt mask register (ETH_MACIMR)"]
    pub macimr: MACIMR,
    #[doc = "0x40 - Ethernet MAC address 0 high register (ETH_MACA0HR)"]
    pub maca0hr: MACA0HR,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub maca0lr: MACA0LR,
    #[doc = "0x48 - Ethernet MAC address 1 high register (ETH_MACA1HR)"]
    pub maca1hr: MACA1HR,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub maca1lr: MACA1LR,
    #[doc = "0x50 - Ethernet MAC address 2 high register (ETH_MACA2HR)"]
    pub maca2hr: MACA2HR,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub maca2lr: MACA2LR,
    #[doc = "0x58 - Ethernet MAC address 3 high register (ETH_MACA3HR)"]
    pub maca3hr: MACA3HR,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub maca3lr: MACA3LR,
}
#[doc = "Ethernet MAC configuration register (ETH_MACCR)"]
pub struct MACCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC configuration register (ETH_MACCR)"]
pub mod maccr;
#[doc = "Ethernet MAC frame filter register (ETH_MACCFFR)"]
pub struct MACFFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC frame filter register (ETH_MACCFFR)"]
pub mod macffr;
#[doc = "Ethernet MAC hash table high register"]
pub struct MACHTHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "Ethernet MAC hash table low register"]
pub struct MACHTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "Ethernet MAC MII address register (ETH_MACMIIAR)"]
pub struct MACMIIAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MII address register (ETH_MACMIIAR)"]
pub mod macmiiar;
#[doc = "Ethernet MAC MII data register (ETH_MACMIIDR)"]
pub struct MACMIIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MII data register (ETH_MACMIIDR)"]
pub mod macmiidr;
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)"]
pub struct MACFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)"]
pub mod macfcr;
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
pub struct MACVLANTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
pub mod macvlantr;
#[doc = "Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
pub struct MACRWUFFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
pub mod macrwuffr;
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
pub struct MACPMTCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
pub mod macpmtcsr;
#[doc = "Ethernet MAC interrupt status register (ETH_MACSR)"]
pub struct MACSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC interrupt status register (ETH_MACSR)"]
pub mod macsr;
#[doc = "Ethernet MAC interrupt mask register (ETH_MACIMR)"]
pub struct MACIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC interrupt mask register (ETH_MACIMR)"]
pub mod macimr;
#[doc = "Ethernet MAC address 0 high register (ETH_MACA0HR)"]
pub struct MACA0HR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 0 high register (ETH_MACA0HR)"]
pub mod maca0hr;
#[doc = "Ethernet MAC address 0 low register"]
pub struct MACA0LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "Ethernet MAC address 1 high register (ETH_MACA1HR)"]
pub struct MACA1HR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 1 high register (ETH_MACA1HR)"]
pub mod maca1hr;
#[doc = "Ethernet MAC address1 low register"]
pub struct MACA1LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "Ethernet MAC address 2 high register (ETH_MACA2HR)"]
pub struct MACA2HR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 2 high register (ETH_MACA2HR)"]
pub mod maca2hr;
#[doc = "Ethernet MAC address 2 low register"]
pub struct MACA2LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "Ethernet MAC address 3 high register (ETH_MACA3HR)"]
pub struct MACA3HR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 3 high register (ETH_MACA3HR)"]
pub mod maca3hr;
#[doc = "Ethernet MAC address 3 low register"]
pub struct MACA3LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;
