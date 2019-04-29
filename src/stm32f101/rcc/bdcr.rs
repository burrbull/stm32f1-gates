#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BDCR {
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
#[doc = "Possible values of the field `LSEON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEONR {
    #[doc = "LSE oscillator Off"]
    OFF,
    #[doc = "LSE oscillator On"]
    ON,
}
impl LSEONR {
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
            LSEONR::OFF => false,
            LSEONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSEONR {
        match value {
            false => LSEONR::OFF,
            true => LSEONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == LSEONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == LSEONR::ON
    }
}
#[doc = "Possible values of the field `LSERDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDYR {
    #[doc = "LSE oscillator not ready"]
    NOTREADY,
    #[doc = "LSE oscillator ready"]
    READY,
}
impl LSERDYR {
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
            LSERDYR::NOTREADY => false,
            LSERDYR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSERDYR {
        match value {
            false => LSERDYR::NOTREADY,
            true => LSERDYR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDYR::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == LSERDYR::READY
    }
}
#[doc = "Possible values of the field `LSEBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYPR {
    #[doc = "LSE crystal oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "LSE crystal oscillator bypassed with external clock"]
    BYPASSED,
}
impl LSEBYPR {
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
            LSEBYPR::NOTBYPASSED => false,
            LSEBYPR::BYPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSEBYPR {
        match value {
            false => LSEBYPR::NOTBYPASSED,
            true => LSEBYPR::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYPR::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYPR::BYPASSED
    }
}
#[doc = "Possible values of the field `RTCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSELR {
    #[doc = "No clock"]
    NOCLOCK,
    #[doc = "LSE oscillator clock used as RTC clock"]
    LSE,
    #[doc = "LSI oscillator clock used as RTC clock"]
    LSI,
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
    HSE,
}
impl RTCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCSELR::NOCLOCK => 0,
            RTCSELR::LSE => 1,
            RTCSELR::LSI => 2,
            RTCSELR::HSE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCSELR {
        match value {
            0 => RTCSELR::NOCLOCK,
            1 => RTCSELR::LSE,
            2 => RTCSELR::LSI,
            3 => RTCSELR::HSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSELR::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline]
    pub fn is_lse(&self) -> bool {
        *self == RTCSELR::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSELR::LSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == RTCSELR::HSE
    }
}
#[doc = "Possible values of the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENR {
    #[doc = "RTC clock disabled"]
    DISABLED,
    #[doc = "RTC clock enabled"]
    ENABLED,
}
impl RTCENR {
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
            RTCENR::DISABLED => false,
            RTCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCENR {
        match value {
            false => RTCENR::DISABLED,
            true => RTCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RTCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RTCENR::ENABLED
    }
}
#[doc = "Possible values of the field `BDRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDRSTR {
    #[doc = "Reset not activated"]
    DISABLED,
    #[doc = "Reset the entire RTC domain"]
    ENABLED,
}
impl BDRSTR {
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
            BDRSTR::DISABLED => false,
            BDRSTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BDRSTR {
        match value {
            false => BDRSTR::DISABLED,
            true => BDRSTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BDRSTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BDRSTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `LSEON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEONW {
    #[doc = "LSE oscillator Off"]
    OFF,
    #[doc = "LSE oscillator On"]
    ON,
}
impl LSEONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSEONW::OFF => false,
            LSEONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSEONW<'a> {
    w: &'a mut W,
}
impl<'a> _LSEONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSEONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSE oscillator Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEONW::OFF)
    }
    #[doc = "LSE oscillator On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEONW::ON)
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
#[doc = "Values that can be written to the field `LSEBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYPW {
    #[doc = "LSE crystal oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "LSE crystal oscillator bypassed with external clock"]
    BYPASSED,
}
impl LSEBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSEBYPW::NOTBYPASSED => false,
            LSEBYPW::BYPASSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSEBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _LSEBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSEBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSE crystal oscillator not bypassed"]
    #[inline]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYPW::NOTBYPASSED)
    }
    #[doc = "LSE crystal oscillator bypassed with external clock"]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYPW::BYPASSED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSELW {
    #[doc = "No clock"]
    NOCLOCK,
    #[doc = "LSE oscillator clock used as RTC clock"]
    LSE,
    #[doc = "LSI oscillator clock used as RTC clock"]
    LSI,
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
    HSE,
}
impl RTCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCSELW::NOCLOCK => 0,
            RTCSELW::LSE => 1,
            RTCSELW::LSI => 2,
            RTCSELW::HSE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No clock"]
    #[inline]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSELW::NOCLOCK)
    }
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSELW::LSE)
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSELW::LSI)
    }
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSELW::HSE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENW {
    #[doc = "RTC clock disabled"]
    DISABLED,
    #[doc = "RTC clock enabled"]
    ENABLED,
}
impl RTCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCENW::DISABLED => false,
            RTCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCENW::DISABLED)
    }
    #[doc = "RTC clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCENW::ENABLED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BDRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDRSTW {
    #[doc = "Reset not activated"]
    DISABLED,
    #[doc = "Reset the entire RTC domain"]
    ENABLED,
}
impl BDRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BDRSTW::DISABLED => false,
            BDRSTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BDRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BDRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BDRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not activated"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDRSTW::DISABLED)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDRSTW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External Low Speed oscillator enable"]
    #[inline]
    pub fn lseon(&self) -> LSEONR {
        LSEONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - External Low Speed oscillator ready"]
    #[inline]
    pub fn lserdy(&self) -> LSERDYR {
        LSERDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - External Low Speed oscillator bypass"]
    #[inline]
    pub fn lsebyp(&self) -> LSEBYPR {
        LSEBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline]
    pub fn rtcsel(&self) -> RTCSELR {
        RTCSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline]
    pub fn rtcen(&self) -> RTCENR {
        RTCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline]
    pub fn bdrst(&self) -> BDRSTR {
        BDRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - External Low Speed oscillator enable"]
    #[inline]
    pub fn lseon(&mut self) -> _LSEONW {
        _LSEONW { w: self }
    }
    #[doc = "Bit 2 - External Low Speed oscillator bypass"]
    #[inline]
    pub fn lsebyp(&mut self) -> _LSEBYPW {
        _LSEBYPW { w: self }
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline]
    pub fn rtcsel(&mut self) -> _RTCSELW {
        _RTCSELW { w: self }
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline]
    pub fn rtcen(&mut self) -> _RTCENW {
        _RTCENW { w: self }
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline]
    pub fn bdrst(&mut self) -> _BDRSTW {
        _BDRSTW { w: self }
    }
}
