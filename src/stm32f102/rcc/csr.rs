#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSR {
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
#[doc = "Possible values of the field `LSION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIONR {
    #[doc = "LSI oscillator Off"]
    OFF,
    #[doc = "LSI oscillator On"]
    ON,
}
impl LSIONR {
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
            LSIONR::OFF => false,
            LSIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSIONR {
        match value {
            false => LSIONR::OFF,
            true => LSIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == LSIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == LSIONR::ON
    }
}
#[doc = "Possible values of the field `LSIRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYR {
    #[doc = "LSI oscillator not ready"]
    NOTREADY,
    #[doc = "LSI oscillator ready"]
    READY,
}
impl LSIRDYR {
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
            LSIRDYR::NOTREADY => false,
            LSIRDYR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSIRDYR {
        match value {
            false => LSIRDYR::NOTREADY,
            true => LSIRDYR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDYR::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDYR::READY
    }
}
#[doc = "Possible values of the field `RMVF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMVFR {
    #[doc = "Clears the reset flag"]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RMVFR {
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
            RMVFR::CLEAR => true,
            RMVFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMVFR {
        match value {
            true => RMVFR::CLEAR,
            i => RMVFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == RMVFR::CLEAR
    }
}
#[doc = "Possible values of the field `PINRSTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINRSTFR {
    #[doc = "No reset has occured"]
    NORESET,
    #[doc = "A reset has occured"]
    RESET,
}
impl PINRSTFR {
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
            PINRSTFR::NORESET => false,
            PINRSTFR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINRSTFR {
        match value {
            false => PINRSTFR::NORESET,
            true => PINRSTFR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline]
    pub fn is_no_reset(&self) -> bool {
        *self == PINRSTFR::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PINRSTFR::RESET
    }
}
#[doc = "Possible values of the field `PORRSTF`"]
pub type PORRSTFR = PINRSTFR;
#[doc = "Possible values of the field `SFTRSTF`"]
pub type SFTRSTFR = PINRSTFR;
#[doc = "Possible values of the field `IWDGRSTF`"]
pub type IWDGRSTFR = PINRSTFR;
#[doc = "Possible values of the field `WWDGRSTF`"]
pub type WWDGRSTFR = PINRSTFR;
#[doc = "Possible values of the field `LPWRRSTF`"]
pub type LPWRRSTFR = PINRSTFR;
#[doc = "Values that can be written to the field `LSION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIONW {
    #[doc = "LSI oscillator Off"]
    OFF,
    #[doc = "LSI oscillator On"]
    ON,
}
impl LSIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSIONW::OFF => false,
            LSIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _LSIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSI oscillator Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(LSIONW::OFF)
    }
    #[doc = "LSI oscillator On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(LSIONW::ON)
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
#[doc = "Values that can be written to the field `RMVF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMVFW {
    #[doc = "Clears the reset flag"]
    CLEAR,
}
impl RMVFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMVFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMVFW<'a> {
    w: &'a mut W,
}
impl<'a> _RMVFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMVFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the reset flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVFW::CLEAR)
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
#[doc = "Values that can be written to the field `PINRSTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINRSTFW {
    #[doc = "No reset has occured"]
    NORESET,
    #[doc = "A reset has occured"]
    RESET,
}
impl PINRSTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINRSTFW::NORESET => false,
            PINRSTFW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINRSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _PINRSTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINRSTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset has occured"]
    #[inline]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PINRSTFW::NORESET)
    }
    #[doc = "A reset has occured"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PINRSTFW::RESET)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PORRSTF`"]
pub type PORRSTFW = PINRSTFW;
#[doc = r" Proxy"]
pub struct _PORRSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _PORRSTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORRSTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset has occured"]
    #[inline]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PINRSTFW::NORESET)
    }
    #[doc = "A reset has occured"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PINRSTFW::RESET)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SFTRSTF`"]
pub type SFTRSTFW = PINRSTFW;
#[doc = r" Proxy"]
pub struct _SFTRSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFTRSTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset has occured"]
    #[inline]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PINRSTFW::NORESET)
    }
    #[doc = "A reset has occured"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PINRSTFW::RESET)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IWDGRSTF`"]
pub type IWDGRSTFW = PINRSTFW;
#[doc = r" Proxy"]
pub struct _IWDGRSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _IWDGRSTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IWDGRSTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset has occured"]
    #[inline]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PINRSTFW::NORESET)
    }
    #[doc = "A reset has occured"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PINRSTFW::RESET)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WWDGRSTF`"]
pub type WWDGRSTFW = PINRSTFW;
#[doc = r" Proxy"]
pub struct _WWDGRSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDGRSTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDGRSTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset has occured"]
    #[inline]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PINRSTFW::NORESET)
    }
    #[doc = "A reset has occured"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PINRSTFW::RESET)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPWRRSTF`"]
pub type LPWRRSTFW = PINRSTFW;
#[doc = r" Proxy"]
pub struct _LPWRRSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _LPWRRSTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPWRRSTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset has occured"]
    #[inline]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PINRSTFW::NORESET)
    }
    #[doc = "A reset has occured"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PINRSTFW::RESET)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline]
    pub fn lsion(&self) -> LSIONR {
        LSIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Internal low speed oscillator ready"]
    #[inline]
    pub fn lsirdy(&self) -> LSIRDYR {
        LSIRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline]
    pub fn rmvf(&self) -> RMVFR {
        RMVFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline]
    pub fn pinrstf(&self) -> PINRSTFR {
        PINRSTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline]
    pub fn porrstf(&self) -> PORRSTFR {
        PORRSTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline]
    pub fn sftrstf(&self) -> SFTRSTFR {
        SFTRSTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline]
    pub fn iwdgrstf(&self) -> IWDGRSTFR {
        IWDGRSTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline]
    pub fn wwdgrstf(&self) -> WWDGRSTFR {
        WWDGRSTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline]
    pub fn lpwrrstf(&self) -> LPWRRSTFR {
        LPWRRSTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 201326592 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline]
    pub fn lsion(&mut self) -> _LSIONW {
        _LSIONW { w: self }
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline]
    pub fn rmvf(&mut self) -> _RMVFW {
        _RMVFW { w: self }
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline]
    pub fn pinrstf(&mut self) -> _PINRSTFW {
        _PINRSTFW { w: self }
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline]
    pub fn porrstf(&mut self) -> _PORRSTFW {
        _PORRSTFW { w: self }
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline]
    pub fn sftrstf(&mut self) -> _SFTRSTFW {
        _SFTRSTFW { w: self }
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline]
    pub fn iwdgrstf(&mut self) -> _IWDGRSTFW {
        _IWDGRSTFW { w: self }
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline]
    pub fn wwdgrstf(&mut self) -> _WWDGRSTFW {
        _WWDGRSTFW { w: self }
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline]
    pub fn lpwrrstf(&mut self) -> _LPWRRSTFW {
        _LPWRRSTFW { w: self }
    }
}
