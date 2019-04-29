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
#[doc = "Possible values of the field `LAST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTR {
    #[doc = "Next DMA EOT is not the last transfer"]
    NOTLAST,
    #[doc = "Next DMA EOT is the last transfer"]
    LAST,
}
impl LASTR {
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
            LASTR::NOTLAST => false,
            LASTR::LAST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LASTR {
        match value {
            false => LASTR::NOTLAST,
            true => LASTR::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLAST`"]
    #[inline]
    pub fn is_not_last(&self) -> bool {
        *self == LASTR::NOTLAST
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline]
    pub fn is_last(&self) -> bool {
        *self == LASTR::LAST
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA requests disabled"]
    DISABLED,
    #[doc = "DMA request enabled when TxE=1 or RxNE=1"]
    ENABLED,
}
impl DMAENR {
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
            DMAENR::DISABLED => false,
            DMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::DISABLED,
            true => DMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMAENR::ENABLED
    }
}
#[doc = "Possible values of the field `ITBUFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITBUFENR {
    #[doc = "TxE=1 or RxNE=1 does not generate any interrupt"]
    DISABLED,
    #[doc = "TxE=1 or RxNE=1 generates Event interrupt"]
    ENABLED,
}
impl ITBUFENR {
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
            ITBUFENR::DISABLED => false,
            ITBUFENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITBUFENR {
        match value {
            false => ITBUFENR::DISABLED,
            true => ITBUFENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ITBUFENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ITBUFENR::ENABLED
    }
}
#[doc = "Possible values of the field `ITEVTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITEVTENR {
    #[doc = "Event interrupt disabled"]
    DISABLED,
    #[doc = "Event interrupt enabled"]
    ENABLED,
}
impl ITEVTENR {
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
            ITEVTENR::DISABLED => false,
            ITEVTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITEVTENR {
        match value {
            false => ITEVTENR::DISABLED,
            true => ITEVTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ITEVTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ITEVTENR::ENABLED
    }
}
#[doc = "Possible values of the field `ITERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITERRENR {
    #[doc = "Error interrupt disabled"]
    DISABLED,
    #[doc = "Error interrupt enabled"]
    ENABLED,
}
impl ITERRENR {
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
            ITERRENR::DISABLED => false,
            ITERRENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITERRENR {
        match value {
            false => ITERRENR::DISABLED,
            true => ITERRENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ITERRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ITERRENR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct FREQR {
    bits: u8,
}
impl FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `LAST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTW {
    #[doc = "Next DMA EOT is not the last transfer"]
    NOTLAST,
    #[doc = "Next DMA EOT is the last transfer"]
    LAST,
}
impl LASTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LASTW::NOTLAST => false,
            LASTW::LAST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LASTW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LASTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline]
    pub fn not_last(self) -> &'a mut W {
        self.variant(LASTW::NOTLAST)
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline]
    pub fn last(self) -> &'a mut W {
        self.variant(LASTW::LAST)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENW {
    #[doc = "DMA requests disabled"]
    DISABLED,
    #[doc = "DMA request enabled when TxE=1 or RxNE=1"]
    ENABLED,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::DISABLED => false,
            DMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA requests disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAENW::DISABLED)
    }
    #[doc = "DMA request enabled when TxE=1 or RxNE=1"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAENW::ENABLED)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ITBUFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITBUFENW {
    #[doc = "TxE=1 or RxNE=1 does not generate any interrupt"]
    DISABLED,
    #[doc = "TxE=1 or RxNE=1 generates Event interrupt"]
    ENABLED,
}
impl ITBUFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITBUFENW::DISABLED => false,
            ITBUFENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITBUFENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITBUFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITBUFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TxE=1 or RxNE=1 does not generate any interrupt"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITBUFENW::DISABLED)
    }
    #[doc = "TxE=1 or RxNE=1 generates Event interrupt"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITBUFENW::ENABLED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ITEVTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITEVTENW {
    #[doc = "Event interrupt disabled"]
    DISABLED,
    #[doc = "Event interrupt enabled"]
    ENABLED,
}
impl ITEVTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITEVTENW::DISABLED => false,
            ITEVTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITEVTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITEVTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITEVTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Event interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITEVTENW::DISABLED)
    }
    #[doc = "Event interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITEVTENW::ENABLED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ITERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITERRENW {
    #[doc = "Error interrupt disabled"]
    DISABLED,
    #[doc = "Error interrupt enabled"]
    ENABLED,
}
impl ITERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITERRENW::DISABLED => false,
            ITERRENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITERRENW::DISABLED)
    }
    #[doc = "Error interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITERRENW::ENABLED)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
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
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline]
    pub fn last(&self) -> LASTR {
        LASTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline]
    pub fn itbufen(&self) -> ITBUFENR {
        ITBUFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline]
    pub fn itevten(&self) -> ITEVTENR {
        ITEVTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline]
    pub fn iterren(&self) -> ITERRENR {
        ITERRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:5 - Peripheral clock frequency"]
    #[inline]
    pub fn freq(&self) -> FREQR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FREQR { bits }
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
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline]
    pub fn last(&mut self) -> _LASTW {
        _LASTW { w: self }
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline]
    pub fn itbufen(&mut self) -> _ITBUFENW {
        _ITBUFENW { w: self }
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline]
    pub fn itevten(&mut self) -> _ITEVTENW {
        _ITEVTENW { w: self }
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline]
    pub fn iterren(&mut self) -> _ITERRENW {
        _ITERRENW { w: self }
    }
    #[doc = "Bits 0:5 - Peripheral clock frequency"]
    #[inline]
    pub fn freq(&mut self) -> _FREQW {
        _FREQW { w: self }
    }
}
