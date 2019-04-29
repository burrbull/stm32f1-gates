use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMPER();
    fn RTC();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2();
    fn DMA1_CHANNEL3();
    fn DMA1_CHANNEL4();
    fn DMA1_CHANNEL5();
    fn DMA1_CHANNEL6();
    fn DMA1_CHANNEL7();
    fn ADC1_2();
    fn USB_HP_CAN_TX();
    fn USB_LP_CAN_RX0();
    fn CAN_RX1();
    fn CAN_SCE();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTCALARM();
    fn USBWAKEUP();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FSMC();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6();
    fn TIM7();
    fn DMA2_CHANNEL1();
    fn DMA2_CHANNEL2();
    fn DMA2_CHANNEL3();
    fn DMA2_CHANNEL4();
    fn DMA2_CHANNEL5();
    fn ETH();
    fn ETH_WKUP();
    fn CAN2_TX();
    fn CAN2_RX0();
    fn CAN2_RX1();
    fn CAN2_SCE();
    fn OTG_FS();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 68] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: TAMPER },
    Vector { _handler: RTC },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2,
    },
    Vector {
        _handler: DMA1_CHANNEL3,
    },
    Vector {
        _handler: DMA1_CHANNEL4,
    },
    Vector {
        _handler: DMA1_CHANNEL5,
    },
    Vector {
        _handler: DMA1_CHANNEL6,
    },
    Vector {
        _handler: DMA1_CHANNEL7,
    },
    Vector { _handler: ADC1_2 },
    Vector {
        _handler: USB_HP_CAN_TX,
    },
    Vector {
        _handler: USB_LP_CAN_RX0,
    },
    Vector { _handler: CAN_RX1 },
    Vector { _handler: CAN_SCE },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector { _handler: RTCALARM },
    Vector {
        _handler: USBWAKEUP,
    },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _handler: FSMC },
    Vector { _handler: SDIO },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_CHANNEL1,
    },
    Vector {
        _handler: DMA2_CHANNEL2,
    },
    Vector {
        _handler: DMA2_CHANNEL3,
    },
    Vector {
        _handler: DMA2_CHANNEL4,
    },
    Vector {
        _handler: DMA2_CHANNEL5,
    },
    Vector { _handler: ETH },
    Vector { _handler: ETH_WKUP },
    Vector { _handler: CAN2_TX },
    Vector { _handler: CAN2_RX0 },
    Vector { _handler: CAN2_RX1 },
    Vector { _handler: CAN2_SCE },
    Vector { _handler: OTG_FS },
];
#[doc = r" Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD,
    #[doc = "2 - Tamper interrupt"]
    TAMPER,
    #[doc = "3 - RTC global interrupt"]
    RTC,
    #[doc = "4 - Flash global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line1 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line2 interrupt"]
    EXTI2,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "11 - DMA1 Channel1 global interrupt"]
    DMA1_CHANNEL1,
    #[doc = "12 - DMA1 Channel2 global interrupt"]
    DMA1_CHANNEL2,
    #[doc = "13 - DMA1 Channel3 global interrupt"]
    DMA1_CHANNEL3,
    #[doc = "14 - DMA1 Channel4 global interrupt"]
    DMA1_CHANNEL4,
    #[doc = "15 - DMA1 Channel5 global interrupt"]
    DMA1_CHANNEL5,
    #[doc = "16 - DMA1 Channel6 global interrupt"]
    DMA1_CHANNEL6,
    #[doc = "17 - DMA1 Channel7 global interrupt"]
    DMA1_CHANNEL7,
    #[doc = "18 - ADC1 and ADC2 global interrupt"]
    ADC1_2,
    #[doc = "19 - USB High Priority or CAN TX interrupts"]
    USB_HP_CAN_TX,
    #[doc = "20 - USB Low Priority or CAN RX0 interrupts"]
    USB_LP_CAN_RX0,
    #[doc = "21 - CAN RX1 interrupts"]
    CAN_RX1,
    #[doc = "22 - CAN SCE interrupt"]
    CAN_SCE,
    #[doc = "23 - EXTI Line\\[9:5\\] interrupts"]
    EXTI9_5,
    #[doc = "24 - TIM1 Break interrupt"]
    TIM1_BRK,
    #[doc = "25 - TIM1 Update interrupt"]
    TIM1_UP,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts"]
    TIM1_TRG_COM,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2,
    #[doc = "37 - USART1 global interrupt"]
    USART1,
    #[doc = "38 - USART2 global interrupt"]
    USART2,
    #[doc = "39 - USART3 global interrupt"]
    USART3,
    #[doc = "40 - EXTI Line\\[15:10\\] interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC Alarms through EXTI line interrupt"]
    RTCALARM,
    #[doc = "42 - USB wakeup from suspend through EXTI line interrupt"]
    USBWAKEUP,
    #[doc = "43 - TIM8 Break interrupt"]
    TIM8_BRK,
    #[doc = "44 - TIM8 Update interrupt"]
    TIM8_UP,
    #[doc = "45 - TIM8 Trigger and Commutation interrupts"]
    TIM8_TRG_COM,
    #[doc = "46 - TIM8 Capture Compare interrupt"]
    TIM8_CC,
    #[doc = "47 - ADC3 global interrupt"]
    ADC3,
    #[doc = "48 - FSMC global interrupt"]
    FSMC,
    #[doc = "49 - SDIO global interrupt"]
    SDIO,
    #[doc = "50 - TIM5 global interrupt"]
    TIM5,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3,
    #[doc = "52 - UART4 global interrupt"]
    UART4,
    #[doc = "53 - UART5 global interrupt"]
    UART5,
    #[doc = "54 - TIM6 global interrupt"]
    TIM6,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7,
    #[doc = "56 - DMA2 Channel1 global interrupt"]
    DMA2_CHANNEL1,
    #[doc = "57 - DMA2 Channel2 global interrupt"]
    DMA2_CHANNEL2,
    #[doc = "58 - DMA2 Channel3 global interrupt"]
    DMA2_CHANNEL3,
    #[doc = "59 - DMA2 Channel4 global interrupt"]
    DMA2_CHANNEL4,
    #[doc = "60 - DMA2 Channel5 global interrupt"]
    DMA2_CHANNEL5,
    #[doc = "61 - Ethernet global interrupt"]
    ETH,
    #[doc = "62 - Ethernet Wakeup through EXTI line interrupt"]
    ETH_WKUP,
    #[doc = "63 - CAN2 TX interrupts"]
    CAN2_TX,
    #[doc = "64 - CAN2 RX0 interrupts"]
    CAN2_RX0,
    #[doc = "65 - CAN2 RX1 interrupts"]
    CAN2_RX1,
    #[doc = "66 - CAN2 SCE interrupt"]
    CAN2_SCE,
    #[doc = "67 - USB On The Go FS global interrupt"]
    OTG_FS,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::TAMPER => 2,
            Interrupt::RTC => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_CHANNEL1 => 11,
            Interrupt::DMA1_CHANNEL2 => 12,
            Interrupt::DMA1_CHANNEL3 => 13,
            Interrupt::DMA1_CHANNEL4 => 14,
            Interrupt::DMA1_CHANNEL5 => 15,
            Interrupt::DMA1_CHANNEL6 => 16,
            Interrupt::DMA1_CHANNEL7 => 17,
            Interrupt::ADC1_2 => 18,
            Interrupt::USB_HP_CAN_TX => 19,
            Interrupt::USB_LP_CAN_RX0 => 20,
            Interrupt::CAN_RX1 => 21,
            Interrupt::CAN_SCE => 22,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK => 24,
            Interrupt::TIM1_UP => 25,
            Interrupt::TIM1_TRG_COM => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::TIM4 => 30,
            Interrupt::I2C1_EV => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::USART1 => 37,
            Interrupt::USART2 => 38,
            Interrupt::USART3 => 39,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTCALARM => 41,
            Interrupt::USBWAKEUP => 42,
            Interrupt::TIM8_BRK => 43,
            Interrupt::TIM8_UP => 44,
            Interrupt::TIM8_TRG_COM => 45,
            Interrupt::TIM8_CC => 46,
            Interrupt::ADC3 => 47,
            Interrupt::FSMC => 48,
            Interrupt::SDIO => 49,
            Interrupt::TIM5 => 50,
            Interrupt::SPI3 => 51,
            Interrupt::UART4 => 52,
            Interrupt::UART5 => 53,
            Interrupt::TIM6 => 54,
            Interrupt::TIM7 => 55,
            Interrupt::DMA2_CHANNEL1 => 56,
            Interrupt::DMA2_CHANNEL2 => 57,
            Interrupt::DMA2_CHANNEL3 => 58,
            Interrupt::DMA2_CHANNEL4 => 59,
            Interrupt::DMA2_CHANNEL5 => 60,
            Interrupt::ETH => 61,
            Interrupt::ETH_WKUP => 62,
            Interrupt::CAN2_TX => 63,
            Interrupt::CAN2_RX0 => 64,
            Interrupt::CAN2_RX1 => 65,
            Interrupt::CAN2_SCE => 66,
            Interrupt::OTG_FS => 67,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Power control"]
#[cfg(feature = "pwr")]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "pwr")]
unsafe impl Send for PWR {}
#[cfg(feature = "pwr")]
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
#[cfg(feature = "pwr")]
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
#[cfg(feature = "pwr")]
pub mod pwr;
#[doc = "Reset and clock control"]
#[cfg(feature = "rcc")]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "rcc")]
unsafe impl Send for RCC {}
#[cfg(feature = "rcc")]
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073876992 as *const _
    }
}
#[cfg(feature = "rcc")]
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
#[cfg(feature = "rcc")]
pub mod rcc;
#[doc = "General purpose I/O"]
#[cfg(feature = "gpioa")]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "gpioa")]
unsafe impl Send for GPIOA {}
#[cfg(feature = "gpioa")]
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073809408 as *const _
    }
}
#[cfg(feature = "gpioa")]
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General purpose I/O"]
#[cfg(feature = "gpioa")]
pub mod gpioa;
#[doc = "GPIOB"]
#[cfg(feature = "gpiob")]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "gpiob")]
unsafe impl Send for GPIOB {}
#[cfg(feature = "gpiob")]
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073810432 as *const _
    }
}
#[cfg(feature = "gpiob")]
impl Deref for GPIOB {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "GPIOC"]
#[cfg(feature = "gpioc")]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "gpioc")]
unsafe impl Send for GPIOC {}
#[cfg(feature = "gpioc")]
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073811456 as *const _
    }
}
#[cfg(feature = "gpioc")]
impl Deref for GPIOC {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIOD"]
#[cfg(feature = "gpiod")]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "gpiod")]
unsafe impl Send for GPIOD {}
#[cfg(feature = "gpiod")]
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073812480 as *const _
    }
}
#[cfg(feature = "gpiod")]
impl Deref for GPIOD {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOE"]
#[cfg(feature = "gpioe")]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "gpioe")]
unsafe impl Send for GPIOE {}
#[cfg(feature = "gpioe")]
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073813504 as *const _
    }
}
#[cfg(feature = "gpioe")]
impl Deref for GPIOE {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIOF"]
#[cfg(feature = "gpiof")]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "gpiof")]
unsafe impl Send for GPIOF {}
#[cfg(feature = "gpiof")]
impl GPIOF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073814528 as *const _
    }
}
#[cfg(feature = "gpiof")]
impl Deref for GPIOF {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "GPIOG"]
#[cfg(feature = "gpiog")]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "gpiog")]
unsafe impl Send for GPIOG {}
#[cfg(feature = "gpiog")]
impl GPIOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073815552 as *const _
    }
}
#[cfg(feature = "gpiog")]
impl Deref for GPIOG {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "Alternate function I/O"]
#[cfg(feature = "afio")]
pub struct AFIO {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "afio")]
unsafe impl Send for AFIO {}
#[cfg(feature = "afio")]
impl AFIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const afio::RegisterBlock {
        1073807360 as *const _
    }
}
#[cfg(feature = "afio")]
impl Deref for AFIO {
    type Target = afio::RegisterBlock;
    fn deref(&self) -> &afio::RegisterBlock {
        unsafe { &*AFIO::ptr() }
    }
}
#[doc = "Alternate function I/O"]
#[cfg(feature = "afio")]
pub mod afio;
#[doc = "EXTI"]
#[cfg(feature = "exti")]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "exti")]
unsafe impl Send for EXTI {}
#[cfg(feature = "exti")]
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073808384 as *const _
    }
}
#[cfg(feature = "exti")]
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "EXTI"]
#[cfg(feature = "exti")]
pub mod exti;
#[doc = "DMA controller"]
#[cfg(feature = "dma1")]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "dma1")]
unsafe impl Send for DMA1 {}
#[cfg(feature = "dma1")]
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073872896 as *const _
    }
}
#[cfg(feature = "dma1")]
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "DMA controller"]
#[cfg(feature = "dma1")]
pub mod dma1;
#[doc = "DMA2"]
#[cfg(feature = "dma2")]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "dma2")]
unsafe impl Send for DMA2 {}
#[cfg(feature = "dma2")]
impl DMA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073873920 as *const _
    }
}
#[cfg(feature = "dma2")]
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "Real time clock"]
#[cfg(feature = "rtc")]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "rtc")]
unsafe impl Send for RTC {}
#[cfg(feature = "rtc")]
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
#[cfg(feature = "rtc")]
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real time clock"]
#[cfg(feature = "rtc")]
pub mod rtc;
#[doc = "Backup registers"]
#[cfg(feature = "bkp")]
pub struct BKP {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "bkp")]
unsafe impl Send for BKP {}
#[cfg(feature = "bkp")]
impl BKP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bkp::RegisterBlock {
        1073769472 as *const _
    }
}
#[cfg(feature = "bkp")]
impl Deref for BKP {
    type Target = bkp::RegisterBlock;
    fn deref(&self) -> &bkp::RegisterBlock {
        unsafe { &*BKP::ptr() }
    }
}
#[doc = "Backup registers"]
#[cfg(feature = "bkp")]
pub mod bkp;
#[doc = "Independent watchdog"]
#[cfg(feature = "iwdg")]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "iwdg")]
unsafe impl Send for IWDG {}
#[cfg(feature = "iwdg")]
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
#[cfg(feature = "iwdg")]
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
#[cfg(feature = "iwdg")]
pub mod iwdg;
#[doc = "Window watchdog"]
#[cfg(feature = "wwdg")]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "wwdg")]
unsafe impl Send for WWDG {}
#[cfg(feature = "wwdg")]
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
#[cfg(feature = "wwdg")]
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "Window watchdog"]
#[cfg(feature = "wwdg")]
pub mod wwdg;
#[doc = "Advanced timer"]
#[cfg(feature = "tim1")]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim1")]
unsafe impl Send for TIM1 {}
#[cfg(feature = "tim1")]
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073818624 as *const _
    }
}
#[cfg(feature = "tim1")]
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced timer"]
#[cfg(feature = "tim1")]
pub mod tim1;
#[doc = "General purpose timer"]
#[cfg(feature = "tim2")]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim2")]
unsafe impl Send for TIM2 {}
#[cfg(feature = "tim2")]
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
#[cfg(feature = "tim2")]
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General purpose timer"]
#[cfg(feature = "tim2")]
pub mod tim2;
#[doc = "TIM3"]
#[cfg(feature = "tim3")]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim3")]
unsafe impl Send for TIM3 {}
#[cfg(feature = "tim3")]
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073742848 as *const _
    }
}
#[cfg(feature = "tim3")]
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "TIM4"]
#[cfg(feature = "tim4")]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim4")]
unsafe impl Send for TIM4 {}
#[cfg(feature = "tim4")]
impl TIM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073743872 as *const _
    }
}
#[cfg(feature = "tim4")]
impl Deref for TIM4 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "TIM5"]
#[cfg(feature = "tim5")]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim5")]
unsafe impl Send for TIM5 {}
#[cfg(feature = "tim5")]
impl TIM5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073744896 as *const _
    }
}
#[cfg(feature = "tim5")]
impl Deref for TIM5 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "Basic timer"]
#[cfg(feature = "tim6")]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim6")]
unsafe impl Send for TIM6 {}
#[cfg(feature = "tim6")]
impl TIM6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073745920 as *const _
    }
}
#[cfg(feature = "tim6")]
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "Basic timer"]
#[cfg(feature = "tim6")]
pub mod tim6;
#[doc = "TIM7"]
#[cfg(feature = "tim7")]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim7")]
unsafe impl Send for TIM7 {}
#[cfg(feature = "tim7")]
impl TIM7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073746944 as *const _
    }
}
#[cfg(feature = "tim7")]
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "Inter integrated circuit"]
#[cfg(feature = "i2c1")]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "i2c1")]
unsafe impl Send for I2C1 {}
#[cfg(feature = "i2c1")]
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073763328 as *const _
    }
}
#[cfg(feature = "i2c1")]
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter integrated circuit"]
#[cfg(feature = "i2c1")]
pub mod i2c1;
#[doc = "I2C2"]
#[cfg(feature = "i2c2")]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "i2c2")]
unsafe impl Send for I2C2 {}
#[cfg(feature = "i2c2")]
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073764352 as *const _
    }
}
#[cfg(feature = "i2c2")]
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
#[cfg(feature = "spi1")]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "spi1")]
unsafe impl Send for SPI1 {}
#[cfg(feature = "spi1")]
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
#[cfg(feature = "spi1")]
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
#[cfg(feature = "spi1")]
pub mod spi1;
#[doc = "SPI2"]
#[cfg(feature = "spi2")]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "spi2")]
unsafe impl Send for SPI2 {}
#[cfg(feature = "spi2")]
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073756160 as *const _
    }
}
#[cfg(feature = "spi2")]
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "SPI3"]
#[cfg(feature = "spi3")]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "spi3")]
unsafe impl Send for SPI3 {}
#[cfg(feature = "spi3")]
impl SPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073757184 as *const _
    }
}
#[cfg(feature = "spi3")]
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
#[cfg(feature = "usart1")]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "usart1")]
unsafe impl Send for USART1 {}
#[cfg(feature = "usart1")]
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073821696 as *const _
    }
}
#[cfg(feature = "usart1")]
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
#[cfg(feature = "usart1")]
pub mod usart1;
#[doc = "USART2"]
#[cfg(feature = "usart2")]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "usart2")]
unsafe impl Send for USART2 {}
#[cfg(feature = "usart2")]
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073759232 as *const _
    }
}
#[cfg(feature = "usart2")]
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART3"]
#[cfg(feature = "usart3")]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "usart3")]
unsafe impl Send for USART3 {}
#[cfg(feature = "usart3")]
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073760256 as *const _
    }
}
#[cfg(feature = "usart3")]
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Analog to digital converter"]
#[cfg(feature = "adc1")]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "adc1")]
unsafe impl Send for ADC1 {}
#[cfg(feature = "adc1")]
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073816576 as *const _
    }
}
#[cfg(feature = "adc1")]
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog to digital converter"]
#[cfg(feature = "adc1")]
pub mod adc1;
#[doc = "Analog to digital converter"]
#[cfg(feature = "adc2")]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "adc2")]
unsafe impl Send for ADC2 {}
#[cfg(feature = "adc2")]
impl ADC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc2::RegisterBlock {
        1073817600 as *const _
    }
}
#[cfg(feature = "adc2")]
impl Deref for ADC2 {
    type Target = adc2::RegisterBlock;
    fn deref(&self) -> &adc2::RegisterBlock {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "Analog to digital converter"]
#[cfg(feature = "adc2")]
pub mod adc2;
#[doc = "Controller area network"]
#[cfg(feature = "can2")]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "can2")]
unsafe impl Send for CAN2 {}
#[cfg(feature = "can2")]
impl CAN2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can2::RegisterBlock {
        1073768448 as *const _
    }
}
#[cfg(feature = "can2")]
impl Deref for CAN2 {
    type Target = can2::RegisterBlock;
    fn deref(&self) -> &can2::RegisterBlock {
        unsafe { &*CAN2::ptr() }
    }
}
#[doc = "Controller area network"]
#[cfg(feature = "can2")]
pub mod can2;
#[doc = "Controller area network"]
#[cfg(feature = "can1")]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "can1")]
unsafe impl Send for CAN1 {}
#[cfg(feature = "can1")]
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1073767424 as *const _
    }
}
#[cfg(feature = "can1")]
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Controller area network"]
#[cfg(feature = "can1")]
pub mod can1;
#[doc = "Ethernet: media access control"]
#[cfg(feature = "ethernet_mac")]
pub struct ETHERNET_MAC {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "ethernet_mac")]
unsafe impl Send for ETHERNET_MAC {}
#[cfg(feature = "ethernet_mac")]
impl ETHERNET_MAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_mac::RegisterBlock {
        1073905664 as *const _
    }
}
#[cfg(feature = "ethernet_mac")]
impl Deref for ETHERNET_MAC {
    type Target = ethernet_mac::RegisterBlock;
    fn deref(&self) -> &ethernet_mac::RegisterBlock {
        unsafe { &*ETHERNET_MAC::ptr() }
    }
}
#[doc = "Ethernet: media access control"]
#[cfg(feature = "ethernet_mac")]
pub mod ethernet_mac;
#[doc = "Ethernet: MAC management counters"]
#[cfg(feature = "ethernet_mmc")]
pub struct ETHERNET_MMC {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "ethernet_mmc")]
unsafe impl Send for ETHERNET_MMC {}
#[cfg(feature = "ethernet_mmc")]
impl ETHERNET_MMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_mmc::RegisterBlock {
        1073905920 as *const _
    }
}
#[cfg(feature = "ethernet_mmc")]
impl Deref for ETHERNET_MMC {
    type Target = ethernet_mmc::RegisterBlock;
    fn deref(&self) -> &ethernet_mmc::RegisterBlock {
        unsafe { &*ETHERNET_MMC::ptr() }
    }
}
#[doc = "Ethernet: MAC management counters"]
#[cfg(feature = "ethernet_mmc")]
pub mod ethernet_mmc;
#[doc = "Ethernet: Precision time protocol"]
#[cfg(feature = "ethernet_ptp")]
pub struct ETHERNET_PTP {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "ethernet_ptp")]
unsafe impl Send for ETHERNET_PTP {}
#[cfg(feature = "ethernet_ptp")]
impl ETHERNET_PTP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_ptp::RegisterBlock {
        1073907456 as *const _
    }
}
#[cfg(feature = "ethernet_ptp")]
impl Deref for ETHERNET_PTP {
    type Target = ethernet_ptp::RegisterBlock;
    fn deref(&self) -> &ethernet_ptp::RegisterBlock {
        unsafe { &*ETHERNET_PTP::ptr() }
    }
}
#[doc = "Ethernet: Precision time protocol"]
#[cfg(feature = "ethernet_ptp")]
pub mod ethernet_ptp;
#[doc = "Ethernet: DMA controller operation"]
#[cfg(feature = "ethernet_dma")]
pub struct ETHERNET_DMA {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "ethernet_dma")]
unsafe impl Send for ETHERNET_DMA {}
#[cfg(feature = "ethernet_dma")]
impl ETHERNET_DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_dma::RegisterBlock {
        1073909760 as *const _
    }
}
#[cfg(feature = "ethernet_dma")]
impl Deref for ETHERNET_DMA {
    type Target = ethernet_dma::RegisterBlock;
    fn deref(&self) -> &ethernet_dma::RegisterBlock {
        unsafe { &*ETHERNET_DMA::ptr() }
    }
}
#[doc = "Ethernet: DMA controller operation"]
#[cfg(feature = "ethernet_dma")]
pub mod ethernet_dma;
#[doc = "USB on the go full speed"]
#[cfg(feature = "usb_otg_global")]
#[cfg(feature = "usb")]
pub struct USB_OTG_GLOBAL {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "usb_otg_global")]
unsafe impl Send for USB_OTG_GLOBAL {}
#[cfg(feature = "usb_otg_global")]
impl USB_OTG_GLOBAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb_otg_global::RegisterBlock {
        1342177280 as *const _
    }
}
#[cfg(feature = "usb_otg_global")]
impl Deref for USB_OTG_GLOBAL {
    type Target = usb_otg_global::RegisterBlock;
    fn deref(&self) -> &usb_otg_global::RegisterBlock {
        unsafe { &*USB_OTG_GLOBAL::ptr() }
    }
}
#[doc = "USB on the go full speed"]
#[cfg(feature = "usb_otg_global")]
pub mod usb_otg_global;
#[doc = "USB on the go full speed"]
#[cfg(feature = "usb_otg_host")]
#[cfg(feature = "usb")]
pub struct USB_OTG_HOST {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "usb_otg_host")]
unsafe impl Send for USB_OTG_HOST {}
#[cfg(feature = "usb_otg_host")]
impl USB_OTG_HOST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb_otg_host::RegisterBlock {
        1342178304 as *const _
    }
}
#[cfg(feature = "usb_otg_host")]
impl Deref for USB_OTG_HOST {
    type Target = usb_otg_host::RegisterBlock;
    fn deref(&self) -> &usb_otg_host::RegisterBlock {
        unsafe { &*USB_OTG_HOST::ptr() }
    }
}
#[doc = "USB on the go full speed"]
#[cfg(feature = "usb_otg_host")]
pub mod usb_otg_host;
#[doc = "USB on the go full speed"]
#[cfg(feature = "usb_otg_device")]
#[cfg(feature = "usb")]
pub struct USB_OTG_DEVICE {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "usb_otg_device")]
unsafe impl Send for USB_OTG_DEVICE {}
#[cfg(feature = "usb_otg_device")]
impl USB_OTG_DEVICE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb_otg_device::RegisterBlock {
        1342179328 as *const _
    }
}
#[cfg(feature = "usb_otg_device")]
impl Deref for USB_OTG_DEVICE {
    type Target = usb_otg_device::RegisterBlock;
    fn deref(&self) -> &usb_otg_device::RegisterBlock {
        unsafe { &*USB_OTG_DEVICE::ptr() }
    }
}
#[doc = "USB on the go full speed"]
#[cfg(feature = "usb_otg_device")]
pub mod usb_otg_device;
#[doc = "USB on the go full speed"]
#[cfg(feature = "usb_otg_pwrclk")]
#[cfg(feature = "usb")]
pub struct USB_OTG_PWRCLK {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "usb_otg_pwrclk")]
unsafe impl Send for USB_OTG_PWRCLK {}
#[cfg(feature = "usb_otg_pwrclk")]
impl USB_OTG_PWRCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb_otg_pwrclk::RegisterBlock {
        1342180864 as *const _
    }
}
#[cfg(feature = "usb_otg_pwrclk")]
impl Deref for USB_OTG_PWRCLK {
    type Target = usb_otg_pwrclk::RegisterBlock;
    fn deref(&self) -> &usb_otg_pwrclk::RegisterBlock {
        unsafe { &*USB_OTG_PWRCLK::ptr() }
    }
}
#[doc = "USB on the go full speed"]
#[cfg(feature = "usb_otg_pwrclk")]
pub mod usb_otg_pwrclk;
#[doc = "Digital to analog converter"]
#[cfg(feature = "dac")]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "dac")]
unsafe impl Send for DAC {}
#[cfg(feature = "dac")]
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1073771520 as *const _
    }
}
#[cfg(feature = "dac")]
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital to analog converter"]
#[cfg(feature = "dac")]
pub mod dac;
#[doc = "Debug support"]
#[cfg(feature = "dbgmcu")]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "dbgmcu")]
unsafe impl Send for DBGMCU {}
#[cfg(feature = "dbgmcu")]
impl DBGMCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbgmcu::RegisterBlock {
        3758366720 as *const _
    }
}
#[cfg(feature = "dbgmcu")]
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    fn deref(&self) -> &dbgmcu::RegisterBlock {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Debug support"]
#[cfg(feature = "dbgmcu")]
pub mod dbgmcu;
#[doc = "Universal asynchronous receiver transmitter"]
#[cfg(feature = "uart4")]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "uart4")]
unsafe impl Send for UART4 {}
#[cfg(feature = "uart4")]
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1073761280 as *const _
    }
}
#[cfg(feature = "uart4")]
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal asynchronous receiver transmitter"]
#[cfg(feature = "uart4")]
pub mod uart4;
#[doc = "UART5"]
#[cfg(feature = "uart5")]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "uart5")]
unsafe impl Send for UART5 {}
#[cfg(feature = "uart5")]
impl UART5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1073762304 as *const _
    }
}
#[cfg(feature = "uart5")]
impl Deref for UART5 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "CRC calculation unit"]
#[cfg(feature = "crc")]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "crc")]
unsafe impl Send for CRC {}
#[cfg(feature = "crc")]
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
#[cfg(feature = "crc")]
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "CRC calculation unit"]
#[cfg(feature = "crc")]
pub mod crc;
#[doc = "FLASH"]
#[cfg(feature = "flash")]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "flash")]
unsafe impl Send for FLASH {}
#[cfg(feature = "flash")]
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073881088 as *const _
    }
}
#[cfg(feature = "flash")]
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "FLASH"]
#[cfg(feature = "flash")]
pub mod flash;
#[doc = "Flexible static memory controller"]
#[cfg(feature = "fsmc")]
pub struct FSMC {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "fsmc")]
unsafe impl Send for FSMC {}
#[cfg(feature = "fsmc")]
impl FSMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fsmc::RegisterBlock {
        2684354560 as *const _
    }
}
#[cfg(feature = "fsmc")]
impl Deref for FSMC {
    type Target = fsmc::RegisterBlock;
    fn deref(&self) -> &fsmc::RegisterBlock {
        unsafe { &*FSMC::ptr() }
    }
}
#[doc = "Flexible static memory controller"]
#[cfg(feature = "fsmc")]
pub mod fsmc;
#[doc = "Secure digital input/output interface"]
#[cfg(feature = "sdio")]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "sdio")]
unsafe impl Send for SDIO {}
#[cfg(feature = "sdio")]
impl SDIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdio::RegisterBlock {
        1073840128 as *const _
    }
}
#[cfg(feature = "sdio")]
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    fn deref(&self) -> &sdio::RegisterBlock {
        unsafe { &*SDIO::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
#[cfg(feature = "sdio")]
pub mod sdio;
#[doc = "General purpose timer"]
#[cfg(feature = "tim1")]
#[cfg(feature = "tim11")]
pub struct TIM11 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim11")]
unsafe impl Send for TIM11 {}
#[cfg(feature = "tim11")]
impl TIM11 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim11::RegisterBlock {
        1073828864 as *const _
    }
}
#[cfg(feature = "tim11")]
impl Deref for TIM11 {
    type Target = tim11::RegisterBlock;
    fn deref(&self) -> &tim11::RegisterBlock {
        unsafe { &*TIM11::ptr() }
    }
}
#[doc = "General purpose timer"]
#[cfg(feature = "tim11")]
pub mod tim11;
#[doc = "TIM10"]
#[cfg(feature = "tim1")]
#[cfg(feature = "tim10")]
pub struct TIM10 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim10")]
unsafe impl Send for TIM10 {}
#[cfg(feature = "tim10")]
impl TIM10 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim11::RegisterBlock {
        1073827840 as *const _
    }
}
#[cfg(feature = "tim10")]
impl Deref for TIM10 {
    type Target = tim11::RegisterBlock;
    fn deref(&self) -> &tim11::RegisterBlock {
        unsafe { &*TIM10::ptr() }
    }
}
#[doc = "General purpose timer"]
#[cfg(feature = "tim9")]
pub struct TIM9 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim9")]
unsafe impl Send for TIM9 {}
#[cfg(feature = "tim9")]
impl TIM9 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073826816 as *const _
    }
}
#[cfg(feature = "tim9")]
impl Deref for TIM9 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM9::ptr() }
    }
}
#[doc = "General purpose timer"]
#[cfg(feature = "tim9")]
pub mod tim9;
#[doc = "TIM12"]
#[cfg(feature = "tim1")]
#[cfg(feature = "tim12")]
pub struct TIM12 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim12")]
unsafe impl Send for TIM12 {}
#[cfg(feature = "tim12")]
impl TIM12 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073747968 as *const _
    }
}
#[cfg(feature = "tim12")]
impl Deref for TIM12 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM12::ptr() }
    }
}
#[doc = "Analog to digital converter"]
#[cfg(feature = "adc3")]
pub struct ADC3 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "adc3")]
unsafe impl Send for ADC3 {}
#[cfg(feature = "adc3")]
impl ADC3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc3::RegisterBlock {
        1073822720 as *const _
    }
}
#[cfg(feature = "adc3")]
impl Deref for ADC3 {
    type Target = adc3::RegisterBlock;
    fn deref(&self) -> &adc3::RegisterBlock {
        unsafe { &*ADC3::ptr() }
    }
}
#[doc = "Analog to digital converter"]
#[cfg(feature = "adc3")]
pub mod adc3;
#[doc = "Advanced timer"]
#[cfg(feature = "tim8")]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim8")]
unsafe impl Send for TIM8 {}
#[cfg(feature = "tim8")]
impl TIM8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim8::RegisterBlock {
        1073820672 as *const _
    }
}
#[cfg(feature = "tim8")]
impl Deref for TIM8 {
    type Target = tim8::RegisterBlock;
    fn deref(&self) -> &tim8::RegisterBlock {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "Advanced timer"]
#[cfg(feature = "tim8")]
pub mod tim8;
#[doc = "Universal serial bus full-speed device interface"]
#[cfg(feature = "usb")]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "usb")]
unsafe impl Send for USB {}
#[cfg(feature = "usb")]
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1073765376 as *const _
    }
}
#[cfg(feature = "usb")]
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Universal serial bus full-speed device interface"]
#[cfg(feature = "usb")]
pub mod usb;
#[doc = "General purpose timer"]
#[cfg(feature = "tim1")]
#[cfg(feature = "tim13")]
pub struct TIM13 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim13")]
unsafe impl Send for TIM13 {}
#[cfg(feature = "tim13")]
impl TIM13 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim13::RegisterBlock {
        1073748992 as *const _
    }
}
#[cfg(feature = "tim13")]
impl Deref for TIM13 {
    type Target = tim13::RegisterBlock;
    fn deref(&self) -> &tim13::RegisterBlock {
        unsafe { &*TIM13::ptr() }
    }
}
#[doc = "General purpose timer"]
#[cfg(feature = "tim13")]
pub mod tim13;
#[doc = "TIM14"]
#[cfg(feature = "tim1")]
#[cfg(feature = "tim14")]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "tim14")]
unsafe impl Send for TIM14 {}
#[cfg(feature = "tim14")]
impl TIM14 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim13::RegisterBlock {
        1073750016 as *const _
    }
}
#[cfg(feature = "tim14")]
impl Deref for TIM14 {
    type Target = tim13::RegisterBlock;
    fn deref(&self) -> &tim13::RegisterBlock {
        unsafe { &*TIM14::ptr() }
    }
}
#[doc = "System control block ACTLR"]
#[cfg(feature = "scb_actrl")]
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "scb_actrl")]
unsafe impl Send for SCB_ACTRL {}
#[cfg(feature = "scb_actrl")]
impl SCB_ACTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scb_actrl::RegisterBlock {
        3758153736 as *const _
    }
}
#[cfg(feature = "scb_actrl")]
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    fn deref(&self) -> &scb_actrl::RegisterBlock {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
#[doc = "System control block ACTLR"]
#[cfg(feature = "scb_actrl")]
pub mod scb_actrl;
#[doc = "Nested vectored interrupt controller"]
#[cfg(feature = "nvic_stir")]
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "nvic_stir")]
unsafe impl Send for NVIC_STIR {}
#[cfg(feature = "nvic_stir")]
impl NVIC_STIR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvic_stir::RegisterBlock {
        3758157568 as *const _
    }
}
#[cfg(feature = "nvic_stir")]
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    fn deref(&self) -> &nvic_stir::RegisterBlock {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
#[doc = "Nested vectored interrupt controller"]
#[cfg(feature = "nvic_stir")]
pub mod nvic_stir;
#[doc = "SysTick timer"]
#[cfg(feature = "stk")]
pub struct STK {
    _marker: PhantomData<*const ()>,
}
#[cfg(feature = "stk")]
unsafe impl Send for STK {}
#[cfg(feature = "stk")]
impl STK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const stk::RegisterBlock {
        3758153744 as *const _
    }
}
#[cfg(feature = "stk")]
impl Deref for STK {
    type Target = stk::RegisterBlock;
    fn deref(&self) -> &stk::RegisterBlock {
        unsafe { &*STK::ptr() }
    }
}
#[doc = "SysTick timer"]
#[cfg(feature = "stk")]
pub mod stk;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PWR"]
    #[cfg(feature = "pwr")]
    pub PWR: PWR,
    #[doc = "RCC"]
    #[cfg(feature = "rcc")]
    pub RCC: RCC,
    #[doc = "GPIOA"]
    #[cfg(feature = "gpioa")]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    #[cfg(feature = "gpiob")]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    #[cfg(feature = "gpioc")]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    #[cfg(feature = "gpiod")]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    #[cfg(feature = "gpioe")]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    #[cfg(feature = "gpiof")]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    #[cfg(feature = "gpiog")]
    pub GPIOG: GPIOG,
    #[doc = "AFIO"]
    #[cfg(feature = "afio")]
    pub AFIO: AFIO,
    #[doc = "EXTI"]
    #[cfg(feature = "exti")]
    pub EXTI: EXTI,
    #[doc = "DMA1"]
    #[cfg(feature = "dma1")]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    #[cfg(feature = "dma2")]
    pub DMA2: DMA2,
    #[doc = "RTC"]
    #[cfg(feature = "rtc")]
    pub RTC: RTC,
    #[doc = "BKP"]
    #[cfg(feature = "bkp")]
    pub BKP: BKP,
    #[doc = "IWDG"]
    #[cfg(feature = "iwdg")]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    #[cfg(feature = "wwdg")]
    pub WWDG: WWDG,
    #[doc = "TIM1"]
    #[cfg(feature = "tim1")]
    pub TIM1: TIM1,
    #[doc = "TIM2"]
    #[cfg(feature = "tim2")]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    #[cfg(feature = "tim3")]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    #[cfg(feature = "tim4")]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    #[cfg(feature = "tim5")]
    pub TIM5: TIM5,
    #[doc = "TIM6"]
    #[cfg(feature = "tim6")]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    #[cfg(feature = "tim7")]
    pub TIM7: TIM7,
    #[doc = "I2C1"]
    #[cfg(feature = "i2c1")]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    #[cfg(feature = "i2c2")]
    pub I2C2: I2C2,
    #[doc = "SPI1"]
    #[cfg(feature = "spi1")]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    #[cfg(feature = "spi2")]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    #[cfg(feature = "spi3")]
    pub SPI3: SPI3,
    #[doc = "USART1"]
    #[cfg(feature = "usart1")]
    pub USART1: USART1,
    #[doc = "USART2"]
    #[cfg(feature = "usart2")]
    pub USART2: USART2,
    #[doc = "USART3"]
    #[cfg(feature = "usart3")]
    pub USART3: USART3,
    #[doc = "ADC1"]
    #[cfg(feature = "adc1")]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    #[cfg(feature = "adc2")]
    pub ADC2: ADC2,
    #[doc = "CAN2"]
    #[cfg(feature = "can2")]
    pub CAN2: CAN2,
    #[doc = "CAN1"]
    #[cfg(feature = "can1")]
    pub CAN1: CAN1,
    #[doc = "ETHERNET_MAC"]
    #[cfg(feature = "ethernet_mac")]
    pub ETHERNET_MAC: ETHERNET_MAC,
    #[doc = "ETHERNET_MMC"]
    #[cfg(feature = "ethernet_mmc")]
    pub ETHERNET_MMC: ETHERNET_MMC,
    #[doc = "ETHERNET_PTP"]
    #[cfg(feature = "ethernet_ptp")]
    pub ETHERNET_PTP: ETHERNET_PTP,
    #[doc = "ETHERNET_DMA"]
    #[cfg(feature = "ethernet_dma")]
    pub ETHERNET_DMA: ETHERNET_DMA,
    #[doc = "USB_OTG_GLOBAL"]
    #[cfg(feature = "usb_otg_global")]
    pub USB_OTG_GLOBAL: USB_OTG_GLOBAL,
    #[doc = "USB_OTG_HOST"]
    #[cfg(feature = "usb_otg_host")]
    pub USB_OTG_HOST: USB_OTG_HOST,
    #[doc = "USB_OTG_DEVICE"]
    #[cfg(feature = "usb_otg_device")]
    pub USB_OTG_DEVICE: USB_OTG_DEVICE,
    #[doc = "USB_OTG_PWRCLK"]
    #[cfg(feature = "usb_otg_pwrclk")]
    pub USB_OTG_PWRCLK: USB_OTG_PWRCLK,
    #[doc = "DAC"]
    #[cfg(feature = "dac")]
    pub DAC: DAC,
    #[doc = "DBGMCU"]
    #[cfg(feature = "dbgmcu")]
    pub DBGMCU: DBGMCU,
    #[doc = "UART4"]
    #[cfg(feature = "uart4")]
    pub UART4: UART4,
    #[doc = "UART5"]
    #[cfg(feature = "uart5")]
    pub UART5: UART5,
    #[doc = "CRC"]
    #[cfg(feature = "crc")]
    pub CRC: CRC,
    #[doc = "FLASH"]
    #[cfg(feature = "flash")]
    pub FLASH: FLASH,
    #[doc = "FSMC"]
    #[cfg(feature = "fsmc")]
    pub FSMC: FSMC,
    #[doc = "SDIO"]
    #[cfg(feature = "sdio")]
    pub SDIO: SDIO,
    #[doc = "TIM11"]
    #[cfg(feature = "tim11")]
    pub TIM11: TIM11,
    #[doc = "TIM10"]
    #[cfg(feature = "tim10")]
    pub TIM10: TIM10,
    #[doc = "TIM9"]
    #[cfg(feature = "tim9")]
    pub TIM9: TIM9,
    #[doc = "TIM12"]
    #[cfg(feature = "tim12")]
    pub TIM12: TIM12,
    #[doc = "ADC3"]
    #[cfg(feature = "adc3")]
    pub ADC3: ADC3,
    #[doc = "TIM8"]
    #[cfg(feature = "tim8")]
    pub TIM8: TIM8,
    #[doc = "USB"]
    #[cfg(feature = "usb")]
    pub USB: USB,
    #[doc = "TIM13"]
    #[cfg(feature = "tim13")]
    pub TIM13: TIM13,
    #[doc = "TIM14"]
    #[cfg(feature = "tim14")]
    pub TIM14: TIM14,
    #[doc = "SCB_ACTRL"]
    #[cfg(feature = "scb_actrl")]
    pub SCB_ACTRL: SCB_ACTRL,
    #[doc = "NVIC_STIR"]
    #[cfg(feature = "nvic_stir")]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "STK"]
    #[cfg(feature = "stk")]
    pub STK: STK,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            #[cfg(feature = "pwr")]
            PWR: PWR {
                _marker: PhantomData,
            },
            #[cfg(feature = "rcc")]
            RCC: RCC {
                _marker: PhantomData,
            },
            #[cfg(feature = "gpioa")]
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            #[cfg(feature = "gpiob")]
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            #[cfg(feature = "gpioc")]
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            #[cfg(feature = "gpiod")]
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            #[cfg(feature = "gpioe")]
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            #[cfg(feature = "gpiof")]
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            #[cfg(feature = "gpiog")]
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            #[cfg(feature = "afio")]
            AFIO: AFIO {
                _marker: PhantomData,
            },
            #[cfg(feature = "exti")]
            EXTI: EXTI {
                _marker: PhantomData,
            },
            #[cfg(feature = "dma1")]
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            #[cfg(feature = "dma2")]
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            #[cfg(feature = "rtc")]
            RTC: RTC {
                _marker: PhantomData,
            },
            #[cfg(feature = "bkp")]
            BKP: BKP {
                _marker: PhantomData,
            },
            #[cfg(feature = "iwdg")]
            IWDG: IWDG {
                _marker: PhantomData,
            },
            #[cfg(feature = "wwdg")]
            WWDG: WWDG {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim1")]
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim2")]
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim3")]
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim4")]
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim5")]
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim6")]
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim7")]
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            #[cfg(feature = "i2c1")]
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            #[cfg(feature = "i2c2")]
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            #[cfg(feature = "spi1")]
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            #[cfg(feature = "spi2")]
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            #[cfg(feature = "spi3")]
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            #[cfg(feature = "usart1")]
            USART1: USART1 {
                _marker: PhantomData,
            },
            #[cfg(feature = "usart2")]
            USART2: USART2 {
                _marker: PhantomData,
            },
            #[cfg(feature = "usart3")]
            USART3: USART3 {
                _marker: PhantomData,
            },
            #[cfg(feature = "adc1")]
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            #[cfg(feature = "adc2")]
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            #[cfg(feature = "can2")]
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            #[cfg(feature = "can1")]
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            #[cfg(feature = "ethernet_mac")]
            ETHERNET_MAC: ETHERNET_MAC {
                _marker: PhantomData,
            },
            #[cfg(feature = "ethernet_mmc")]
            ETHERNET_MMC: ETHERNET_MMC {
                _marker: PhantomData,
            },
            #[cfg(feature = "ethernet_ptp")]
            ETHERNET_PTP: ETHERNET_PTP {
                _marker: PhantomData,
            },
            #[cfg(feature = "ethernet_dma")]
            ETHERNET_DMA: ETHERNET_DMA {
                _marker: PhantomData,
            },
            #[cfg(feature = "usb_otg_global")]
            USB_OTG_GLOBAL: USB_OTG_GLOBAL {
                _marker: PhantomData,
            },
            #[cfg(feature = "usb_otg_host")]
            USB_OTG_HOST: USB_OTG_HOST {
                _marker: PhantomData,
            },
            #[cfg(feature = "usb_otg_device")]
            USB_OTG_DEVICE: USB_OTG_DEVICE {
                _marker: PhantomData,
            },
            #[cfg(feature = "usb_otg_pwrclk")]
            USB_OTG_PWRCLK: USB_OTG_PWRCLK {
                _marker: PhantomData,
            },
            #[cfg(feature = "dac")]
            DAC: DAC {
                _marker: PhantomData,
            },
            #[cfg(feature = "dbgmcu")]
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            #[cfg(feature = "uart4")]
            UART4: UART4 {
                _marker: PhantomData,
            },
            #[cfg(feature = "uart5")]
            UART5: UART5 {
                _marker: PhantomData,
            },
            #[cfg(feature = "crc")]
            CRC: CRC {
                _marker: PhantomData,
            },
            #[cfg(feature = "flash")]
            FLASH: FLASH {
                _marker: PhantomData,
            },
            #[cfg(feature = "fsmc")]
            FSMC: FSMC {
                _marker: PhantomData,
            },
            #[cfg(feature = "sdio")]
            SDIO: SDIO {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim11")]
            TIM11: TIM11 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim10")]
            TIM10: TIM10 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim9")]
            TIM9: TIM9 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim12")]
            TIM12: TIM12 {
                _marker: PhantomData,
            },
            #[cfg(feature = "adc3")]
            ADC3: ADC3 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim8")]
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            #[cfg(feature = "usb")]
            USB: USB {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim13")]
            TIM13: TIM13 {
                _marker: PhantomData,
            },
            #[cfg(feature = "tim14")]
            TIM14: TIM14 {
                _marker: PhantomData,
            },
            #[cfg(feature = "scb_actrl")]
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
            #[cfg(feature = "nvic_stir")]
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            #[cfg(feature = "stk")]
            STK: STK {
                _marker: PhantomData,
            },
        }
    }
}
