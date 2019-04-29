#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR2 {
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
#[doc = "Possible values of the field `PREDIV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREDIV1R {
    #[doc = "PREDIV input clock not divided"]
    DIV1,
    #[doc = "PREDIV input clock divided by 2"]
    DIV2,
    #[doc = "PREDIV input clock divided by 3"]
    DIV3,
    #[doc = "PREDIV input clock divided by 4"]
    DIV4,
    #[doc = "PREDIV input clock divided by 5"]
    DIV5,
    #[doc = "PREDIV input clock divided by 6"]
    DIV6,
    #[doc = "PREDIV input clock divided by 7"]
    DIV7,
    #[doc = "PREDIV input clock divided by 8"]
    DIV8,
    #[doc = "PREDIV input clock divided by 9"]
    DIV9,
    #[doc = "PREDIV input clock divided by 10"]
    DIV10,
    #[doc = "PREDIV input clock divided by 11"]
    DIV11,
    #[doc = "PREDIV input clock divided by 12"]
    DIV12,
    #[doc = "PREDIV input clock divided by 13"]
    DIV13,
    #[doc = "PREDIV input clock divided by 14"]
    DIV14,
    #[doc = "PREDIV input clock divided by 15"]
    DIV15,
    #[doc = "PREDIV input clock divided by 16"]
    DIV16,
}
impl PREDIV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PREDIV1R::DIV1 => 0,
            PREDIV1R::DIV2 => 1,
            PREDIV1R::DIV3 => 2,
            PREDIV1R::DIV4 => 3,
            PREDIV1R::DIV5 => 4,
            PREDIV1R::DIV6 => 5,
            PREDIV1R::DIV7 => 6,
            PREDIV1R::DIV8 => 7,
            PREDIV1R::DIV9 => 8,
            PREDIV1R::DIV10 => 9,
            PREDIV1R::DIV11 => 10,
            PREDIV1R::DIV12 => 11,
            PREDIV1R::DIV13 => 12,
            PREDIV1R::DIV14 => 13,
            PREDIV1R::DIV15 => 14,
            PREDIV1R::DIV16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PREDIV1R {
        match value {
            0 => PREDIV1R::DIV1,
            1 => PREDIV1R::DIV2,
            2 => PREDIV1R::DIV3,
            3 => PREDIV1R::DIV4,
            4 => PREDIV1R::DIV5,
            5 => PREDIV1R::DIV6,
            6 => PREDIV1R::DIV7,
            7 => PREDIV1R::DIV8,
            8 => PREDIV1R::DIV9,
            9 => PREDIV1R::DIV10,
            10 => PREDIV1R::DIV11,
            11 => PREDIV1R::DIV12,
            12 => PREDIV1R::DIV13,
            13 => PREDIV1R::DIV14,
            14 => PREDIV1R::DIV15,
            15 => PREDIV1R::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PREDIV1R::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PREDIV1R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline]
    pub fn is_div3(&self) -> bool {
        *self == PREDIV1R::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PREDIV1R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline]
    pub fn is_div5(&self) -> bool {
        *self == PREDIV1R::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline]
    pub fn is_div6(&self) -> bool {
        *self == PREDIV1R::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline]
    pub fn is_div7(&self) -> bool {
        *self == PREDIV1R::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PREDIV1R::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline]
    pub fn is_div9(&self) -> bool {
        *self == PREDIV1R::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline]
    pub fn is_div10(&self) -> bool {
        *self == PREDIV1R::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline]
    pub fn is_div11(&self) -> bool {
        *self == PREDIV1R::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline]
    pub fn is_div12(&self) -> bool {
        *self == PREDIV1R::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline]
    pub fn is_div13(&self) -> bool {
        *self == PREDIV1R::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline]
    pub fn is_div14(&self) -> bool {
        *self == PREDIV1R::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline]
    pub fn is_div15(&self) -> bool {
        *self == PREDIV1R::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PREDIV1R::DIV16
    }
}
#[doc = "Possible values of the field `PREDIV2`"]
pub type PREDIV2R = PREDIV1R;
#[doc = "Possible values of the field `PLL2MUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL2MULR {
    #[doc = "PLL clock entry x8"]
    MUL8,
    #[doc = "PLL clock entry x9"]
    MUL9,
    #[doc = "PLL clock entry x10"]
    MUL10,
    #[doc = "PLL clock entry x11"]
    MUL11,
    #[doc = "PLL clock entry x12"]
    MUL12,
    #[doc = "PLL clock entry x13"]
    MUL13,
    #[doc = "PLL clock entry x14"]
    MUL14,
    #[doc = "PLL clock entry x16"]
    MUL16,
    #[doc = "PLL clock entry x20"]
    MUL20,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLL2MULR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLL2MULR::MUL8 => 6,
            PLL2MULR::MUL9 => 7,
            PLL2MULR::MUL10 => 8,
            PLL2MULR::MUL11 => 9,
            PLL2MULR::MUL12 => 10,
            PLL2MULR::MUL13 => 11,
            PLL2MULR::MUL14 => 12,
            PLL2MULR::MUL16 => 14,
            PLL2MULR::MUL20 => 15,
            PLL2MULR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLL2MULR {
        match value {
            6 => PLL2MULR::MUL8,
            7 => PLL2MULR::MUL9,
            8 => PLL2MULR::MUL10,
            9 => PLL2MULR::MUL11,
            10 => PLL2MULR::MUL12,
            11 => PLL2MULR::MUL13,
            12 => PLL2MULR::MUL14,
            14 => PLL2MULR::MUL16,
            15 => PLL2MULR::MUL20,
            i => PLL2MULR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MUL8`"]
    #[inline]
    pub fn is_mul8(&self) -> bool {
        *self == PLL2MULR::MUL8
    }
    #[doc = "Checks if the value of the field is `MUL9`"]
    #[inline]
    pub fn is_mul9(&self) -> bool {
        *self == PLL2MULR::MUL9
    }
    #[doc = "Checks if the value of the field is `MUL10`"]
    #[inline]
    pub fn is_mul10(&self) -> bool {
        *self == PLL2MULR::MUL10
    }
    #[doc = "Checks if the value of the field is `MUL11`"]
    #[inline]
    pub fn is_mul11(&self) -> bool {
        *self == PLL2MULR::MUL11
    }
    #[doc = "Checks if the value of the field is `MUL12`"]
    #[inline]
    pub fn is_mul12(&self) -> bool {
        *self == PLL2MULR::MUL12
    }
    #[doc = "Checks if the value of the field is `MUL13`"]
    #[inline]
    pub fn is_mul13(&self) -> bool {
        *self == PLL2MULR::MUL13
    }
    #[doc = "Checks if the value of the field is `MUL14`"]
    #[inline]
    pub fn is_mul14(&self) -> bool {
        *self == PLL2MULR::MUL14
    }
    #[doc = "Checks if the value of the field is `MUL16`"]
    #[inline]
    pub fn is_mul16(&self) -> bool {
        *self == PLL2MULR::MUL16
    }
    #[doc = "Checks if the value of the field is `MUL20`"]
    #[inline]
    pub fn is_mul20(&self) -> bool {
        *self == PLL2MULR::MUL20
    }
}
#[doc = "Possible values of the field `PLL3MUL`"]
pub type PLL3MULR = PLL2MULR;
#[doc = "Possible values of the field `PREDIV1SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREDIV1SRCR {
    #[doc = "HSE oscillator clock selected as PREDIV1 clock entry"]
    HSE,
    #[doc = "PLL2 selected as PREDIV1 clock entry"]
    PLL2,
}
impl PREDIV1SRCR {
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
            PREDIV1SRCR::HSE => false,
            PREDIV1SRCR::PLL2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PREDIV1SRCR {
        match value {
            false => PREDIV1SRCR::HSE,
            true => PREDIV1SRCR::PLL2,
        }
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == PREDIV1SRCR::HSE
    }
    #[doc = "Checks if the value of the field is `PLL2`"]
    #[inline]
    pub fn is_pll2(&self) -> bool {
        *self == PREDIV1SRCR::PLL2
    }
}
#[doc = "Possible values of the field `I2S2SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S2SRCR {
    #[doc = "System clock (SYSCLK) selected as I2S clock entry"]
    SYSCLK,
    #[doc = "PLL3 VCO clock selected as I2S clock entry"]
    PLL3,
}
impl I2S2SRCR {
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
            I2S2SRCR::SYSCLK => false,
            I2S2SRCR::PLL3 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2S2SRCR {
        match value {
            false => I2S2SRCR::SYSCLK,
            true => I2S2SRCR::PLL3,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline]
    pub fn is_sysclk(&self) -> bool {
        *self == I2S2SRCR::SYSCLK
    }
    #[doc = "Checks if the value of the field is `PLL3`"]
    #[inline]
    pub fn is_pll3(&self) -> bool {
        *self == I2S2SRCR::PLL3
    }
}
#[doc = "Possible values of the field `I2S3SRC`"]
pub type I2S3SRCR = I2S2SRCR;
#[doc = "Values that can be written to the field `PREDIV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREDIV1W {
    #[doc = "PREDIV input clock not divided"]
    DIV1,
    #[doc = "PREDIV input clock divided by 2"]
    DIV2,
    #[doc = "PREDIV input clock divided by 3"]
    DIV3,
    #[doc = "PREDIV input clock divided by 4"]
    DIV4,
    #[doc = "PREDIV input clock divided by 5"]
    DIV5,
    #[doc = "PREDIV input clock divided by 6"]
    DIV6,
    #[doc = "PREDIV input clock divided by 7"]
    DIV7,
    #[doc = "PREDIV input clock divided by 8"]
    DIV8,
    #[doc = "PREDIV input clock divided by 9"]
    DIV9,
    #[doc = "PREDIV input clock divided by 10"]
    DIV10,
    #[doc = "PREDIV input clock divided by 11"]
    DIV11,
    #[doc = "PREDIV input clock divided by 12"]
    DIV12,
    #[doc = "PREDIV input clock divided by 13"]
    DIV13,
    #[doc = "PREDIV input clock divided by 14"]
    DIV14,
    #[doc = "PREDIV input clock divided by 15"]
    DIV15,
    #[doc = "PREDIV input clock divided by 16"]
    DIV16,
}
impl PREDIV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PREDIV1W::DIV1 => 0,
            PREDIV1W::DIV2 => 1,
            PREDIV1W::DIV3 => 2,
            PREDIV1W::DIV4 => 3,
            PREDIV1W::DIV5 => 4,
            PREDIV1W::DIV6 => 5,
            PREDIV1W::DIV7 => 6,
            PREDIV1W::DIV8 => 7,
            PREDIV1W::DIV9 => 8,
            PREDIV1W::DIV10 => 9,
            PREDIV1W::DIV11 => 10,
            PREDIV1W::DIV12 => 11,
            PREDIV1W::DIV13 => 12,
            PREDIV1W::DIV14 => 13,
            PREDIV1W::DIV15 => 14,
            PREDIV1W::DIV16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PREDIV1W<'a> {
    w: &'a mut W,
}
impl<'a> _PREDIV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PREDIV1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PREDIV input clock not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PREDIV2`"]
pub type PREDIV2W = PREDIV1W;
#[doc = r" Proxy"]
pub struct _PREDIV2W<'a> {
    w: &'a mut W,
}
impl<'a> _PREDIV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PREDIV2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PREDIV input clock not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIV1W::DIV16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLL2MUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL2MULW {
    #[doc = "PLL clock entry x8"]
    MUL8,
    #[doc = "PLL clock entry x9"]
    MUL9,
    #[doc = "PLL clock entry x10"]
    MUL10,
    #[doc = "PLL clock entry x11"]
    MUL11,
    #[doc = "PLL clock entry x12"]
    MUL12,
    #[doc = "PLL clock entry x13"]
    MUL13,
    #[doc = "PLL clock entry x14"]
    MUL14,
    #[doc = "PLL clock entry x16"]
    MUL16,
    #[doc = "PLL clock entry x20"]
    MUL20,
}
impl PLL2MULW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLL2MULW::MUL8 => 6,
            PLL2MULW::MUL9 => 7,
            PLL2MULW::MUL10 => 8,
            PLL2MULW::MUL11 => 9,
            PLL2MULW::MUL12 => 10,
            PLL2MULW::MUL13 => 11,
            PLL2MULW::MUL14 => 12,
            PLL2MULW::MUL16 => 14,
            PLL2MULW::MUL20 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL2MULW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL2MULW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL2MULW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PLL clock entry x8"]
    #[inline]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL8)
    }
    #[doc = "PLL clock entry x9"]
    #[inline]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL9)
    }
    #[doc = "PLL clock entry x10"]
    #[inline]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL10)
    }
    #[doc = "PLL clock entry x11"]
    #[inline]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL11)
    }
    #[doc = "PLL clock entry x12"]
    #[inline]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL12)
    }
    #[doc = "PLL clock entry x13"]
    #[inline]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL13)
    }
    #[doc = "PLL clock entry x14"]
    #[inline]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL14)
    }
    #[doc = "PLL clock entry x16"]
    #[inline]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL16)
    }
    #[doc = "PLL clock entry x20"]
    #[inline]
    pub fn mul20(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL20)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLL3MUL`"]
pub type PLL3MULW = PLL2MULW;
#[doc = r" Proxy"]
pub struct _PLL3MULW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL3MULW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL3MULW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PLL clock entry x8"]
    #[inline]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL8)
    }
    #[doc = "PLL clock entry x9"]
    #[inline]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL9)
    }
    #[doc = "PLL clock entry x10"]
    #[inline]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL10)
    }
    #[doc = "PLL clock entry x11"]
    #[inline]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL11)
    }
    #[doc = "PLL clock entry x12"]
    #[inline]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL12)
    }
    #[doc = "PLL clock entry x13"]
    #[inline]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL13)
    }
    #[doc = "PLL clock entry x14"]
    #[inline]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL14)
    }
    #[doc = "PLL clock entry x16"]
    #[inline]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL16)
    }
    #[doc = "PLL clock entry x20"]
    #[inline]
    pub fn mul20(self) -> &'a mut W {
        self.variant(PLL2MULW::MUL20)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PREDIV1SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREDIV1SRCW {
    #[doc = "HSE oscillator clock selected as PREDIV1 clock entry"]
    HSE,
    #[doc = "PLL2 selected as PREDIV1 clock entry"]
    PLL2,
}
impl PREDIV1SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PREDIV1SRCW::HSE => false,
            PREDIV1SRCW::PLL2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PREDIV1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _PREDIV1SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PREDIV1SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSE oscillator clock selected as PREDIV1 clock entry"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(PREDIV1SRCW::HSE)
    }
    #[doc = "PLL2 selected as PREDIV1 clock entry"]
    #[inline]
    pub fn pll2(self) -> &'a mut W {
        self.variant(PREDIV1SRCW::PLL2)
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
#[doc = "Values that can be written to the field `I2S2SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S2SRCW {
    #[doc = "System clock (SYSCLK) selected as I2S clock entry"]
    SYSCLK,
    #[doc = "PLL3 VCO clock selected as I2S clock entry"]
    PLL3,
}
impl I2S2SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2S2SRCW::SYSCLK => false,
            I2S2SRCW::PLL3 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2S2SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _I2S2SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2S2SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System clock (SYSCLK) selected as I2S clock entry"]
    #[inline]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2S2SRCW::SYSCLK)
    }
    #[doc = "PLL3 VCO clock selected as I2S clock entry"]
    #[inline]
    pub fn pll3(self) -> &'a mut W {
        self.variant(I2S2SRCW::PLL3)
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
#[doc = "Values that can be written to the field `I2S3SRC`"]
pub type I2S3SRCW = I2S2SRCW;
#[doc = r" Proxy"]
pub struct _I2S3SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _I2S3SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2S3SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System clock (SYSCLK) selected as I2S clock entry"]
    #[inline]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2S2SRCW::SYSCLK)
    }
    #[doc = "PLL3 VCO clock selected as I2S clock entry"]
    #[inline]
    pub fn pll3(self) -> &'a mut W {
        self.variant(I2S2SRCW::PLL3)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:3 - PREDIV1 division factor"]
    #[inline]
    pub fn prediv1(&self) -> PREDIV1R {
        PREDIV1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - PREDIV2 division factor"]
    #[inline]
    pub fn prediv2(&self) -> PREDIV2R {
        PREDIV2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - PLL2 Multiplication Factor"]
    #[inline]
    pub fn pll2mul(&self) -> PLL2MULR {
        PLL2MULR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - PLL3 Multiplication Factor"]
    #[inline]
    pub fn pll3mul(&self) -> PLL3MULR {
        PLL3MULR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - PREDIV1 entry clock source"]
    #[inline]
    pub fn prediv1src(&self) -> PREDIV1SRCR {
        PREDIV1SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - I2S2 clock source"]
    #[inline]
    pub fn i2s2src(&self) -> I2S2SRCR {
        I2S2SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - I2S3 clock source"]
    #[inline]
    pub fn i2s3src(&self) -> I2S3SRCR {
        I2S3SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:3 - PREDIV1 division factor"]
    #[inline]
    pub fn prediv1(&mut self) -> _PREDIV1W {
        _PREDIV1W { w: self }
    }
    #[doc = "Bits 4:7 - PREDIV2 division factor"]
    #[inline]
    pub fn prediv2(&mut self) -> _PREDIV2W {
        _PREDIV2W { w: self }
    }
    #[doc = "Bits 8:11 - PLL2 Multiplication Factor"]
    #[inline]
    pub fn pll2mul(&mut self) -> _PLL2MULW {
        _PLL2MULW { w: self }
    }
    #[doc = "Bits 12:15 - PLL3 Multiplication Factor"]
    #[inline]
    pub fn pll3mul(&mut self) -> _PLL3MULW {
        _PLL3MULW { w: self }
    }
    #[doc = "Bit 16 - PREDIV1 entry clock source"]
    #[inline]
    pub fn prediv1src(&mut self) -> _PREDIV1SRCW {
        _PREDIV1SRCW { w: self }
    }
    #[doc = "Bit 17 - I2S2 clock source"]
    #[inline]
    pub fn i2s2src(&mut self) -> _I2S2SRCW {
        _I2S2SRCW { w: self }
    }
    #[doc = "Bit 18 - I2S3 clock source"]
    #[inline]
    pub fn i2s3src(&mut self) -> _I2S3SRCW {
        _I2S3SRCW { w: self }
    }
}
