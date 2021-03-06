#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR {
    #[doc = "HSI selected as system clock"]
    HSI,
    #[doc = "HSE selected as system clock"]
    HSE,
    #[doc = "PLL selected as system clock"]
    PLL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWR::HSI => 0,
            SWR::HSE => 1,
            SWR::PLL => 2,
            SWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWR {
        match value {
            0 => SWR::HSI,
            1 => SWR::HSE,
            2 => SWR::PLL,
            i => SWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == SWR::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == SWR::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == SWR::PLL
    }
}
#[doc = "Possible values of the field `SWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSR {
    #[doc = "HSE oscillator used as system clock"]
    HSI,
    #[doc = "HSI oscillator used as system clock"]
    HSE,
    #[doc = "PLL used as system clock"]
    PLL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWSR::HSI => 0,
            SWSR::HSE => 1,
            SWSR::PLL => 2,
            SWSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWSR {
        match value {
            0 => SWSR::HSI,
            1 => SWSR::HSE,
            2 => SWSR::PLL,
            i => SWSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == SWSR::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == SWSR::PLL
    }
}
#[doc = "Possible values of the field `HPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPRER {
    #[doc = "SYSCLK not divided"]
    DIV1,
    #[doc = "SYSCLK divided by 2"]
    DIV2,
    #[doc = "SYSCLK divided by 4"]
    DIV4,
    #[doc = "SYSCLK divided by 8"]
    DIV8,
    #[doc = "SYSCLK divided by 16"]
    DIV16,
    #[doc = "SYSCLK divided by 64"]
    DIV64,
    #[doc = "SYSCLK divided by 128"]
    DIV128,
    #[doc = "SYSCLK divided by 256"]
    DIV256,
    #[doc = "SYSCLK divided by 512"]
    DIV512,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPRER::DIV1 => 0,
            HPRER::DIV2 => 8,
            HPRER::DIV4 => 9,
            HPRER::DIV8 => 10,
            HPRER::DIV16 => 11,
            HPRER::DIV64 => 12,
            HPRER::DIV128 => 13,
            HPRER::DIV256 => 14,
            HPRER::DIV512 => 15,
            HPRER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPRER {
        match value {
            0 => HPRER::DIV1,
            8 => HPRER::DIV2,
            9 => HPRER::DIV4,
            10 => HPRER::DIV8,
            11 => HPRER::DIV16,
            12 => HPRER::DIV64,
            13 => HPRER::DIV128,
            14 => HPRER::DIV256,
            15 => HPRER::DIV512,
            i => HPRER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == HPRER::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == HPRER::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == HPRER::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == HPRER::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == HPRER::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == HPRER::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == HPRER::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == HPRER::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == HPRER::DIV512
    }
}
#[doc = "Possible values of the field `PPRE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRE1R {
    #[doc = "HCLK not divided"]
    DIV1,
    #[doc = "HCLK divided by 2"]
    DIV2,
    #[doc = "HCLK divided by 4"]
    DIV4,
    #[doc = "HCLK divided by 8"]
    DIV8,
    #[doc = "HCLK divided by 16"]
    DIV16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PPRE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PPRE1R::DIV1 => 0,
            PPRE1R::DIV2 => 4,
            PPRE1R::DIV4 => 5,
            PPRE1R::DIV8 => 6,
            PPRE1R::DIV16 => 7,
            PPRE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PPRE1R {
        match value {
            0 => PPRE1R::DIV1,
            4 => PPRE1R::DIV2,
            5 => PPRE1R::DIV4,
            6 => PPRE1R::DIV8,
            7 => PPRE1R::DIV16,
            i => PPRE1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PPRE1R::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1R::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1R::DIV16
    }
}
#[doc = "Possible values of the field `PPRE2`"]
pub type PPRE2R = PPRE1R;
#[doc = "Possible values of the field `ADCPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPRER {
    #[doc = "PCLK2 divided by 2"]
    DIV2,
    #[doc = "PCLK2 divided by 4"]
    DIV4,
    #[doc = "PCLK2 divided by 8"]
    DIV6,
    #[doc = "PCLK2 divided by 16"]
    DIV8,
}
impl ADCPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCPRER::DIV2 => 0,
            ADCPRER::DIV4 => 1,
            ADCPRER::DIV6 => 2,
            ADCPRER::DIV8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCPRER {
        match value {
            0 => ADCPRER::DIV2,
            1 => ADCPRER::DIV4,
            2 => ADCPRER::DIV6,
            3 => ADCPRER::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRER::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRER::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRER::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRER::DIV8
    }
}
#[doc = "Possible values of the field `PLLSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRCR {
    #[doc = "HSI divided by 2 selected as PLL input clock"]
    HSI_DIV2,
    #[doc = "HSE divided by PREDIV selected as PLL input clock"]
    HSE_DIV_PREDIV,
}
impl PLLSRCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PLLSRCR::HSI_DIV2 => false,
            PLLSRCR::HSE_DIV_PREDIV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSRCR {
        match value {
            false => PLLSRCR::HSI_DIV2,
            true => PLLSRCR::HSE_DIV_PREDIV,
        }
    }
    #[doc = "Checks if the value of the field is `HSI_DIV2`"]
    #[inline]
    pub fn is_hsi_div2(&self) -> bool {
        *self == PLLSRCR::HSI_DIV2
    }
    #[doc = "Checks if the value of the field is `HSE_DIV_PREDIV`"]
    #[inline]
    pub fn is_hse_div_prediv(&self) -> bool {
        *self == PLLSRCR::HSE_DIV_PREDIV
    }
}
#[doc = "Possible values of the field `PLLXTPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLXTPRER {
    #[doc = "HSE clock not divided"]
    DIV1,
    #[doc = "HSE clock divided by 2"]
    DIV2,
}
impl PLLXTPRER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PLLXTPRER::DIV1 => false,
            PLLXTPRER::DIV2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLXTPRER {
        match value {
            false => PLLXTPRER::DIV1,
            true => PLLXTPRER::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PLLXTPRER::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PLLXTPRER::DIV2
    }
}
#[doc = "Possible values of the field `PLLMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLMULR {
    #[doc = "PLL input clock x2"]
    MUL2,
    #[doc = "PLL input clock x3"]
    MUL3,
    #[doc = "PLL input clock x4"]
    MUL4,
    #[doc = "PLL input clock x5"]
    MUL5,
    #[doc = "PLL input clock x6"]
    MUL6,
    #[doc = "PLL input clock x7"]
    MUL7,
    #[doc = "PLL input clock x8"]
    MUL8,
    #[doc = "PLL input clock x9"]
    MUL9,
    #[doc = "PLL input clock x10"]
    MUL10,
    #[doc = "PLL input clock x11"]
    MUL11,
    #[doc = "PLL input clock x12"]
    MUL12,
    #[doc = "PLL input clock x13"]
    MUL13,
    #[doc = "PLL input clock x14"]
    MUL14,
    #[doc = "PLL input clock x15"]
    MUL15,
    #[doc = "PLL input clock x16"]
    MUL16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLLMULR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLMULR::MUL2 => 0,
            PLLMULR::MUL3 => 1,
            PLLMULR::MUL4 => 2,
            PLLMULR::MUL5 => 3,
            PLLMULR::MUL6 => 4,
            PLLMULR::MUL7 => 5,
            PLLMULR::MUL8 => 6,
            PLLMULR::MUL9 => 7,
            PLLMULR::MUL10 => 8,
            PLLMULR::MUL11 => 9,
            PLLMULR::MUL12 => 10,
            PLLMULR::MUL13 => 11,
            PLLMULR::MUL14 => 12,
            PLLMULR::MUL15 => 13,
            PLLMULR::MUL16 => 15,
            PLLMULR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLMULR {
        match value {
            0 => PLLMULR::MUL2,
            1 => PLLMULR::MUL3,
            2 => PLLMULR::MUL4,
            3 => PLLMULR::MUL5,
            4 => PLLMULR::MUL6,
            5 => PLLMULR::MUL7,
            6 => PLLMULR::MUL8,
            7 => PLLMULR::MUL9,
            8 => PLLMULR::MUL10,
            9 => PLLMULR::MUL11,
            10 => PLLMULR::MUL12,
            11 => PLLMULR::MUL13,
            12 => PLLMULR::MUL14,
            13 => PLLMULR::MUL15,
            15 => PLLMULR::MUL16,
            i => PLLMULR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline]
    pub fn is_mul2(&self) -> bool {
        *self == PLLMULR::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL3`"]
    #[inline]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMULR::MUL3
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMULR::MUL4
    }
    #[doc = "Checks if the value of the field is `MUL5`"]
    #[inline]
    pub fn is_mul5(&self) -> bool {
        *self == PLLMULR::MUL5
    }
    #[doc = "Checks if the value of the field is `MUL6`"]
    #[inline]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMULR::MUL6
    }
    #[doc = "Checks if the value of the field is `MUL7`"]
    #[inline]
    pub fn is_mul7(&self) -> bool {
        *self == PLLMULR::MUL7
    }
    #[doc = "Checks if the value of the field is `MUL8`"]
    #[inline]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMULR::MUL8
    }
    #[doc = "Checks if the value of the field is `MUL9`"]
    #[inline]
    pub fn is_mul9(&self) -> bool {
        *self == PLLMULR::MUL9
    }
    #[doc = "Checks if the value of the field is `MUL10`"]
    #[inline]
    pub fn is_mul10(&self) -> bool {
        *self == PLLMULR::MUL10
    }
    #[doc = "Checks if the value of the field is `MUL11`"]
    #[inline]
    pub fn is_mul11(&self) -> bool {
        *self == PLLMULR::MUL11
    }
    #[doc = "Checks if the value of the field is `MUL12`"]
    #[inline]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMULR::MUL12
    }
    #[doc = "Checks if the value of the field is `MUL13`"]
    #[inline]
    pub fn is_mul13(&self) -> bool {
        *self == PLLMULR::MUL13
    }
    #[doc = "Checks if the value of the field is `MUL14`"]
    #[inline]
    pub fn is_mul14(&self) -> bool {
        *self == PLLMULR::MUL14
    }
    #[doc = "Checks if the value of the field is `MUL15`"]
    #[inline]
    pub fn is_mul15(&self) -> bool {
        *self == PLLMULR::MUL15
    }
    #[doc = "Checks if the value of the field is `MUL16`"]
    #[inline]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMULR::MUL16
    }
}
#[doc = "Possible values of the field `OTGFSPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSPRER {
    #[doc = "PLL clock is divided by 1.5"]
    DIV1_5,
    #[doc = "PLL clock is not divided"]
    DIV1,
}
impl OTGFSPRER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OTGFSPRER::DIV1_5 => false,
            OTGFSPRER::DIV1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGFSPRER {
        match value {
            false => OTGFSPRER::DIV1_5,
            true => OTGFSPRER::DIV1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1_5`"]
    #[inline]
    pub fn is_div1_5(&self) -> bool {
        *self == OTGFSPRER::DIV1_5
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == OTGFSPRER::DIV1
    }
}
#[doc = "Possible values of the field `MCO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCOR {
    #[doc = "MCO output disabled, no clock on MCO"]
    NOMCO,
    #[doc = "System clock selected"]
    SYSCLK,
    #[doc = "HSI oscillator clock selected"]
    HSI,
    #[doc = "HSE oscillator clock selected"]
    HSE,
    #[doc = "PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    PLL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MCOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCOR::NOMCO => 0,
            MCOR::SYSCLK => 4,
            MCOR::HSI => 5,
            MCOR::HSE => 6,
            MCOR::PLL => 7,
            MCOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCOR {
        match value {
            0 => MCOR::NOMCO,
            4 => MCOR::SYSCLK,
            5 => MCOR::HSI,
            6 => MCOR::HSE,
            7 => MCOR::PLL,
            i => MCOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOMCO`"]
    #[inline]
    pub fn is_no_mco(&self) -> bool {
        *self == MCOR::NOMCO
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline]
    pub fn is_sysclk(&self) -> bool {
        *self == MCOR::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == MCOR::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == MCOR::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == MCOR::PLL
    }
}
#[doc = "Values that can be written to the field `SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWW {
    #[doc = "HSI selected as system clock"]
    HSI,
    #[doc = "HSE selected as system clock"]
    HSE,
    #[doc = "PLL selected as system clock"]
    PLL,
}
impl SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWW::HSI => 0,
            SWW::HSE => 1,
            SWW::PLL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWW<'a> {
    w: &'a mut W,
}
impl<'a> _SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HSI selected as system clock"]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SWW::HSI)
    }
    #[doc = "HSE selected as system clock"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(SWW::HSE)
    }
    #[doc = "PLL selected as system clock"]
    #[inline]
    pub fn pll(self) -> &'a mut W {
        self.variant(SWW::PLL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPREW {
    #[doc = "SYSCLK not divided"]
    DIV1,
    #[doc = "SYSCLK divided by 2"]
    DIV2,
    #[doc = "SYSCLK divided by 4"]
    DIV4,
    #[doc = "SYSCLK divided by 8"]
    DIV8,
    #[doc = "SYSCLK divided by 16"]
    DIV16,
    #[doc = "SYSCLK divided by 64"]
    DIV64,
    #[doc = "SYSCLK divided by 128"]
    DIV128,
    #[doc = "SYSCLK divided by 256"]
    DIV256,
    #[doc = "SYSCLK divided by 512"]
    DIV512,
}
impl HPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPREW::DIV1 => 0,
            HPREW::DIV2 => 8,
            HPREW::DIV4 => 9,
            HPREW::DIV8 => 10,
            HPREW::DIV16 => 11,
            HPREW::DIV64 => 12,
            HPREW::DIV128 => 13,
            HPREW::DIV256 => 14,
            HPREW::DIV512 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPREW<'a> {
    w: &'a mut W,
}
impl<'a> _HPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SYSCLK not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPREW::DIV1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPREW::DIV2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPREW::DIV4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPREW::DIV8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPREW::DIV16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPREW::DIV64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPREW::DIV128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPREW::DIV256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPREW::DIV512)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PPRE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRE1W {
    #[doc = "HCLK not divided"]
    DIV1,
    #[doc = "HCLK divided by 2"]
    DIV2,
    #[doc = "HCLK divided by 4"]
    DIV4,
    #[doc = "HCLK divided by 8"]
    DIV8,
    #[doc = "HCLK divided by 16"]
    DIV16,
}
impl PPRE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PPRE1W::DIV1 => 0,
            PPRE1W::DIV2 => 4,
            PPRE1W::DIV4 => 5,
            PPRE1W::DIV8 => 6,
            PPRE1W::DIV16 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPRE1W<'a> {
    w: &'a mut W,
}
impl<'a> _PPRE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPRE1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HCLK not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1W::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1W::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1W::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1W::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1W::DIV16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PPRE2`"]
pub type PPRE2W = PPRE1W;
#[doc = r" Proxy"]
pub struct _PPRE2W<'a> {
    w: &'a mut W,
}
impl<'a> _PPRE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPRE2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HCLK not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1W::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1W::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1W::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1W::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1W::DIV16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPREW {
    #[doc = "PCLK2 divided by 2"]
    DIV2,
    #[doc = "PCLK2 divided by 4"]
    DIV4,
    #[doc = "PCLK2 divided by 8"]
    DIV6,
    #[doc = "PCLK2 divided by 16"]
    DIV8,
}
impl ADCPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCPREW::DIV2 => 0,
            ADCPREW::DIV4 => 1,
            ADCPREW::DIV6 => 2,
            ADCPREW::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCPREW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCPREW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PCLK2 divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADCPREW::DIV2)
    }
    #[doc = "PCLK2 divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADCPREW::DIV4)
    }
    #[doc = "PCLK2 divided by 8"]
    #[inline]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADCPREW::DIV6)
    }
    #[doc = "PCLK2 divided by 16"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADCPREW::DIV8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRCW {
    #[doc = "HSI divided by 2 selected as PLL input clock"]
    HSI_DIV2,
    #[doc = "HSE divided by PREDIV selected as PLL input clock"]
    HSE_DIV_PREDIV,
}
impl PLLSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLSRCW::HSI_DIV2 => false,
            PLLSRCW::HSE_DIV_PREDIV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI divided by 2 selected as PLL input clock"]
    #[inline]
    pub fn hsi_div2(self) -> &'a mut W {
        self.variant(PLLSRCW::HSI_DIV2)
    }
    #[doc = "HSE divided by PREDIV selected as PLL input clock"]
    #[inline]
    pub fn hse_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRCW::HSE_DIV_PREDIV)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLXTPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLXTPREW {
    #[doc = "HSE clock not divided"]
    DIV1,
    #[doc = "HSE clock divided by 2"]
    DIV2,
}
impl PLLXTPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLXTPREW::DIV1 => false,
            PLLXTPREW::DIV2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLXTPREW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLXTPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLXTPREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSE clock not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLXTPREW::DIV1)
    }
    #[doc = "HSE clock divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLXTPREW::DIV2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLMULW {
    #[doc = "PLL input clock x2"]
    MUL2,
    #[doc = "PLL input clock x3"]
    MUL3,
    #[doc = "PLL input clock x4"]
    MUL4,
    #[doc = "PLL input clock x5"]
    MUL5,
    #[doc = "PLL input clock x6"]
    MUL6,
    #[doc = "PLL input clock x7"]
    MUL7,
    #[doc = "PLL input clock x8"]
    MUL8,
    #[doc = "PLL input clock x9"]
    MUL9,
    #[doc = "PLL input clock x10"]
    MUL10,
    #[doc = "PLL input clock x11"]
    MUL11,
    #[doc = "PLL input clock x12"]
    MUL12,
    #[doc = "PLL input clock x13"]
    MUL13,
    #[doc = "PLL input clock x14"]
    MUL14,
    #[doc = "PLL input clock x15"]
    MUL15,
    #[doc = "PLL input clock x16"]
    MUL16,
}
impl PLLMULW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLMULW::MUL2 => 0,
            PLLMULW::MUL3 => 1,
            PLLMULW::MUL4 => 2,
            PLLMULW::MUL5 => 3,
            PLLMULW::MUL6 => 4,
            PLLMULW::MUL7 => 5,
            PLLMULW::MUL8 => 6,
            PLLMULW::MUL9 => 7,
            PLLMULW::MUL10 => 8,
            PLLMULW::MUL11 => 9,
            PLLMULW::MUL12 => 10,
            PLLMULW::MUL13 => 11,
            PLLMULW::MUL14 => 12,
            PLLMULW::MUL15 => 13,
            PLLMULW::MUL16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLMULW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLMULW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLMULW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PLL input clock x2"]
    #[inline]
    pub fn mul2(self) -> &'a mut W {
        self.variant(PLLMULW::MUL2)
    }
    #[doc = "PLL input clock x3"]
    #[inline]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMULW::MUL3)
    }
    #[doc = "PLL input clock x4"]
    #[inline]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMULW::MUL4)
    }
    #[doc = "PLL input clock x5"]
    #[inline]
    pub fn mul5(self) -> &'a mut W {
        self.variant(PLLMULW::MUL5)
    }
    #[doc = "PLL input clock x6"]
    #[inline]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMULW::MUL6)
    }
    #[doc = "PLL input clock x7"]
    #[inline]
    pub fn mul7(self) -> &'a mut W {
        self.variant(PLLMULW::MUL7)
    }
    #[doc = "PLL input clock x8"]
    #[inline]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMULW::MUL8)
    }
    #[doc = "PLL input clock x9"]
    #[inline]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLLMULW::MUL9)
    }
    #[doc = "PLL input clock x10"]
    #[inline]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLLMULW::MUL10)
    }
    #[doc = "PLL input clock x11"]
    #[inline]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLLMULW::MUL11)
    }
    #[doc = "PLL input clock x12"]
    #[inline]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMULW::MUL12)
    }
    #[doc = "PLL input clock x13"]
    #[inline]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLLMULW::MUL13)
    }
    #[doc = "PLL input clock x14"]
    #[inline]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLLMULW::MUL14)
    }
    #[doc = "PLL input clock x15"]
    #[inline]
    pub fn mul15(self) -> &'a mut W {
        self.variant(PLLMULW::MUL15)
    }
    #[doc = "PLL input clock x16"]
    #[inline]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMULW::MUL16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OTGFSPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSPREW {
    #[doc = "PLL clock is divided by 1.5"]
    DIV1_5,
    #[doc = "PLL clock is not divided"]
    DIV1,
}
impl OTGFSPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGFSPREW::DIV1_5 => false,
            OTGFSPREW::DIV1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGFSPREW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGFSPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGFSPREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL clock is divided by 1.5"]
    #[inline]
    pub fn div1_5(self) -> &'a mut W {
        self.variant(OTGFSPREW::DIV1_5)
    }
    #[doc = "PLL clock is not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(OTGFSPREW::DIV1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCOW {
    #[doc = "MCO output disabled, no clock on MCO"]
    NOMCO,
    #[doc = "System clock selected"]
    SYSCLK,
    #[doc = "HSI oscillator clock selected"]
    HSI,
    #[doc = "HSE oscillator clock selected"]
    HSE,
    #[doc = "PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    PLL,
}
impl MCOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCOW::NOMCO => 0,
            MCOW::SYSCLK => 4,
            MCOW::HSI => 5,
            MCOW::HSE => 6,
            MCOW::PLL => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCOW<'a> {
    w: &'a mut W,
}
impl<'a> _MCOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline]
    pub fn no_mco(self) -> &'a mut W {
        self.variant(MCOW::NOMCO)
    }
    #[doc = "System clock selected"]
    #[inline]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCOW::SYSCLK)
    }
    #[doc = "HSI oscillator clock selected"]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCOW::HSI)
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCOW::HSE)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    #[inline]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCOW::PLL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline]
    pub fn sw(&self) -> SWR {
        SWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline]
    pub fn sws(&self) -> SWSR {
        SWSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline]
    pub fn hpre(&self) -> HPRER {
        HPRER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline]
    pub fn ppre1(&self) -> PPRE1R {
        PPRE1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - APB High speed prescaler (APB2)"]
    #[inline]
    pub fn ppre2(&self) -> PPRE2R {
        PPRE2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - ADC prescaler"]
    #[inline]
    pub fn adcpre(&self) -> ADCPRER {
        ADCPRER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline]
    pub fn pllsrc(&self) -> PLLSRCR {
        PLLSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline]
    pub fn pllxtpre(&self) -> PLLXTPRER {
        PLLXTPRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline]
    pub fn pllmul(&self) -> PLLMULR {
        PLLMULR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - USB OTG FS prescaler"]
    #[inline]
    pub fn otgfspre(&self) -> OTGFSPRER {
        OTGFSPRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline]
    pub fn mco(&self) -> MCOR {
        MCOR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline]
    pub fn sw(&mut self) -> _SWW {
        _SWW { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline]
    pub fn hpre(&mut self) -> _HPREW {
        _HPREW { w: self }
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline]
    pub fn ppre1(&mut self) -> _PPRE1W {
        _PPRE1W { w: self }
    }
    #[doc = "Bits 11:13 - APB High speed prescaler (APB2)"]
    #[inline]
    pub fn ppre2(&mut self) -> _PPRE2W {
        _PPRE2W { w: self }
    }
    #[doc = "Bits 14:15 - ADC prescaler"]
    #[inline]
    pub fn adcpre(&mut self) -> _ADCPREW {
        _ADCPREW { w: self }
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline]
    pub fn pllsrc(&mut self) -> _PLLSRCW {
        _PLLSRCW { w: self }
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline]
    pub fn pllxtpre(&mut self) -> _PLLXTPREW {
        _PLLXTPREW { w: self }
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline]
    pub fn pllmul(&mut self) -> _PLLMULW {
        _PLLMULW { w: self }
    }
    #[doc = "Bit 22 - USB OTG FS prescaler"]
    #[inline]
    pub fn otgfspre(&mut self) -> _OTGFSPREW {
        _OTGFSPREW { w: self }
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline]
    pub fn mco(&mut self) -> _MCOW {
        _MCOW { w: self }
    }
}
