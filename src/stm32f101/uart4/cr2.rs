#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
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
#[doc = r" Value of the field"]
pub struct ADDR {
    bits: u8,
}
impl ADDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LBDL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDLR {
    #[doc = "10-bit break detection"]
    LBDL10,
    #[doc = "11-bit break detection"]
    LBDL11,
}
impl LBDLR {
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
            LBDLR::LBDL10 => false,
            LBDLR::LBDL11 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBDLR {
        match value {
            false => LBDLR::LBDL10,
            true => LBDLR::LBDL11,
        }
    }
    #[doc = "Checks if the value of the field is `LBDL10`"]
    #[inline]
    pub fn is_lbdl10(&self) -> bool {
        *self == LBDLR::LBDL10
    }
    #[doc = "Checks if the value of the field is `LBDL11`"]
    #[inline]
    pub fn is_lbdl11(&self) -> bool {
        *self == LBDLR::LBDL11
    }
}
#[doc = "Possible values of the field `LBDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDIER {
    #[doc = "LIN break detection interrupt disabled"]
    DISABLED,
    #[doc = "LIN break detection interrupt enabled"]
    ENABLED,
}
impl LBDIER {
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
            LBDIER::DISABLED => false,
            LBDIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBDIER {
        match value {
            false => LBDIER::DISABLED,
            true => LBDIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIER::ENABLED
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "1 stop bit"]
    STOP1,
    #[doc = "2 stop bits"]
    STOP2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOPR::STOP1 => 0,
            STOPR::STOP2 => 2,
            STOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOPR {
        match value {
            0 => STOPR::STOP1,
            2 => STOPR::STOP2,
            i => STOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline]
    pub fn is_stop1(&self) -> bool {
        *self == STOPR::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline]
    pub fn is_stop2(&self) -> bool {
        *self == STOPR::STOP2
    }
}
#[doc = "Possible values of the field `LINEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINENR {
    #[doc = "LIN mode disabled"]
    DISABLED,
    #[doc = "LIN mode enabled"]
    ENABLED,
}
impl LINENR {
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
            LINENR::DISABLED => false,
            LINENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINENR {
        match value {
            false => LINENR::DISABLED,
            true => LINENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LINENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LINENR::ENABLED
    }
}
#[doc = r" Proxy"]
pub struct _ADDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDW<'a> {
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
#[doc = "Values that can be written to the field `LBDL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDLW {
    #[doc = "10-bit break detection"]
    LBDL10,
    #[doc = "11-bit break detection"]
    LBDL11,
}
impl LBDLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBDLW::LBDL10 => false,
            LBDLW::LBDL11 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBDLW<'a> {
    w: &'a mut W,
}
impl<'a> _LBDLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBDLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "10-bit break detection"]
    #[inline]
    pub fn lbdl10(self) -> &'a mut W {
        self.variant(LBDLW::LBDL10)
    }
    #[doc = "11-bit break detection"]
    #[inline]
    pub fn lbdl11(self) -> &'a mut W {
        self.variant(LBDLW::LBDL11)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LBDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDIEW {
    #[doc = "LIN break detection interrupt disabled"]
    DISABLED,
    #[doc = "LIN break detection interrupt enabled"]
    ENABLED,
}
impl LBDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBDIEW::DISABLED => false,
            LBDIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LIN break detection interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBDIEW::DISABLED)
    }
    #[doc = "LIN break detection interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBDIEW::ENABLED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPW {
    #[doc = "1 stop bit"]
    STOP1,
    #[doc = "2 stop bits"]
    STOP2,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOPW::STOP1 => 0,
            STOPW::STOP2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 stop bit"]
    #[inline]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STOPW::STOP1)
    }
    #[doc = "2 stop bits"]
    #[inline]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STOPW::STOP2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LINEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINENW {
    #[doc = "LIN mode disabled"]
    DISABLED,
    #[doc = "LIN mode enabled"]
    ENABLED,
}
impl LINENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINENW::DISABLED => false,
            LINENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINENW<'a> {
    w: &'a mut W,
}
impl<'a> _LINENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINENW::DISABLED)
    }
    #[doc = "LIN mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINENW::ENABLED)
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline]
    pub fn add(&self) -> ADDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDR { bits }
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline]
    pub fn lbdl(&self) -> LBDLR {
        LBDLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline]
    pub fn lbdie(&self) -> LBDIER {
        LBDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline]
    pub fn linen(&self) -> LINENR {
        LINENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline]
    pub fn add(&mut self) -> _ADDW {
        _ADDW { w: self }
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline]
    pub fn lbdl(&mut self) -> _LBDLW {
        _LBDLW { w: self }
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline]
    pub fn lbdie(&mut self) -> _LBDIEW {
        _LBDIEW { w: self }
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline]
    pub fn linen(&mut self) -> _LINENW {
        _LINENW { w: self }
    }
}
