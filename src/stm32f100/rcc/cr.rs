#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `HSION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIONR {
    #[doc = "Clock Off"]
    OFF,
    #[doc = "Clock On"]
    ON,
}
impl HSIONR {
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
            HSIONR::OFF => false,
            HSIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSIONR {
        match value {
            false => HSIONR::OFF,
            true => HSIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == HSIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == HSIONR::ON
    }
}
#[doc = "Possible values of the field `HSIRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDYR {
    #[doc = "Clock not ready"]
    NOTREADY,
    #[doc = "Clock ready"]
    READY,
}
impl HSIRDYR {
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
            HSIRDYR::NOTREADY => false,
            HSIRDYR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSIRDYR {
        match value {
            false => HSIRDYR::NOTREADY,
            true => HSIRDYR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::READY
    }
}
#[doc = r" Value of the field"]
pub struct HSITRIMR {
    bits: u8,
}
impl HSITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSICALR {
    bits: u8,
}
impl HSICALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HSEON`"]
pub type HSEONR = HSIONR;
#[doc = "Possible values of the field `HSERDY`"]
pub type HSERDYR = HSIRDYR;
#[doc = "Possible values of the field `HSEBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYPR {
    #[doc = "HSE crystal oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    BYPASSED,
}
impl HSEBYPR {
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
            HSEBYPR::NOTBYPASSED => false,
            HSEBYPR::BYPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSEBYPR {
        match value {
            false => HSEBYPR::NOTBYPASSED,
            true => HSEBYPR::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYPR::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYPR::BYPASSED
    }
}
#[doc = "Possible values of the field `CSSON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSONR {
    #[doc = "Clock security system disabled (clock detector OFF)"]
    OFF,
    #[doc = "Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    ON,
}
impl CSSONR {
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
            CSSONR::OFF => false,
            CSSONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSSONR {
        match value {
            false => CSSONR::OFF,
            true => CSSONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == CSSONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == CSSONR::ON
    }
}
#[doc = "Possible values of the field `PLLON`"]
pub type PLLONR = HSIONR;
#[doc = "Possible values of the field `PLLRDY`"]
pub type PLLRDYR = HSIRDYR;
#[doc = "Values that can be written to the field `HSION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIONW {
    #[doc = "Clock Off"]
    OFF,
    #[doc = "Clock On"]
    ON,
}
impl HSIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSIONW::OFF => false,
            HSIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(HSIONW::OFF)
    }
    #[doc = "Clock On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(HSIONW::ON)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSITRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _HSITRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HSEON`"]
pub type HSEONW = HSIONW;
#[doc = r" Proxy"]
pub struct _HSEONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSEONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSEONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(HSIONW::OFF)
    }
    #[doc = "Clock On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(HSIONW::ON)
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
#[doc = "Values that can be written to the field `HSEBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYPW {
    #[doc = "HSE crystal oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    BYPASSED,
}
impl HSEBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSEBYPW::NOTBYPASSED => false,
            HSEBYPW::BYPASSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSEBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _HSEBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSEBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYPW::NOTBYPASSED)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYPW::BYPASSED)
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
#[doc = "Values that can be written to the field `CSSON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSONW {
    #[doc = "Clock security system disabled (clock detector OFF)"]
    OFF,
    #[doc = "Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    ON,
}
impl CSSONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSSONW::OFF => false,
            CSSONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSSONW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock security system disabled (clock detector OFF)"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(CSSONW::OFF)
    }
    #[doc = "Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(CSSONW::ON)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLON`"]
pub type PLLONW = HSIONW;
#[doc = r" Proxy"]
pub struct _PLLONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(HSIONW::OFF)
    }
    #[doc = "Clock On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(HSIONW::ON)
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
    #[doc = "Bit 0 - Internal High Speed clock enable"]
    #[inline]
    pub fn hsion(&self) -> HSIONR {
        HSIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Internal High Speed clock ready flag"]
    #[inline]
    pub fn hsirdy(&self) -> HSIRDYR {
        HSIRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:7 - Internal High Speed clock trimming"]
    #[inline]
    pub fn hsitrim(&self) -> HSITRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSITRIMR { bits }
    }
    #[doc = "Bits 8:15 - Internal High Speed clock Calibration"]
    #[inline]
    pub fn hsical(&self) -> HSICALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSICALR { bits }
    }
    #[doc = "Bit 16 - External High Speed clock enable"]
    #[inline]
    pub fn hseon(&self) -> HSEONR {
        HSEONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - External High Speed clock ready flag"]
    #[inline]
    pub fn hserdy(&self) -> HSERDYR {
        HSERDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - External High Speed clock Bypass"]
    #[inline]
    pub fn hsebyp(&self) -> HSEBYPR {
        HSEBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Clock Security System enable"]
    #[inline]
    pub fn csson(&self) -> CSSONR {
        CSSONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline]
    pub fn pllon(&self) -> PLLONR {
        PLLONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline]
    pub fn pllrdy(&self) -> PLLRDYR {
        PLLRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 131 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal High Speed clock enable"]
    #[inline]
    pub fn hsion(&mut self) -> _HSIONW {
        _HSIONW { w: self }
    }
    #[doc = "Bits 3:7 - Internal High Speed clock trimming"]
    #[inline]
    pub fn hsitrim(&mut self) -> _HSITRIMW {
        _HSITRIMW { w: self }
    }
    #[doc = "Bit 16 - External High Speed clock enable"]
    #[inline]
    pub fn hseon(&mut self) -> _HSEONW {
        _HSEONW { w: self }
    }
    #[doc = "Bit 18 - External High Speed clock Bypass"]
    #[inline]
    pub fn hsebyp(&mut self) -> _HSEBYPW {
        _HSEBYPW { w: self }
    }
    #[doc = "Bit 19 - Clock Security System enable"]
    #[inline]
    pub fn csson(&mut self) -> _CSSONW {
        _CSSONW { w: self }
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline]
    pub fn pllon(&mut self) -> _PLLONW {
        _PLLONW { w: self }
    }
}
