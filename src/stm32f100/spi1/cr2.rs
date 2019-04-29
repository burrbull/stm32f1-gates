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
#[doc = "Possible values of the field `TXEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIER {
    #[doc = "TXE interrupt masked"]
    MASKED,
    #[doc = "TXE interrupt not masked"]
    NOTMASKED,
}
impl TXEIER {
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
            TXEIER::MASKED => false,
            TXEIER::NOTMASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEIER {
        match value {
            false => TXEIER::MASKED,
            true => TXEIER::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == TXEIER::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIER::NOTMASKED
    }
}
#[doc = "Possible values of the field `RXNEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIER {
    #[doc = "RXE interrupt masked"]
    MASKED,
    #[doc = "RXE interrupt not masked"]
    NOTMASKED,
}
impl RXNEIER {
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
            RXNEIER::MASKED => false,
            RXNEIER::NOTMASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNEIER {
        match value {
            false => RXNEIER::MASKED,
            true => RXNEIER::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIER::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIER::NOTMASKED
    }
}
#[doc = "Possible values of the field `ERRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIER {
    #[doc = "Error interrupt masked"]
    MASKED,
    #[doc = "Error interrupt not masked"]
    NOTMASKED,
}
impl ERRIER {
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
            ERRIER::MASKED => false,
            ERRIER::NOTMASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRIER {
        match value {
            false => ERRIER::MASKED,
            true => ERRIER::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == ERRIER::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIER::NOTMASKED
    }
}
#[doc = "Possible values of the field `SSOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOER {
    #[doc = "SS output is disabled in master mode"]
    DISABLED,
    #[doc = "SS output is enabled in master mode"]
    ENABLED,
}
impl SSOER {
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
            SSOER::DISABLED => false,
            SSOER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSOER {
        match value {
            false => SSOER::DISABLED,
            true => SSOER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSOER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSOER::ENABLED
    }
}
#[doc = "Possible values of the field `TXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAENR {
    #[doc = "Tx buffer DMA disabled"]
    DISABLED,
    #[doc = "Tx buffer DMA enabled"]
    ENABLED,
}
impl TXDMAENR {
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
            TXDMAENR::DISABLED => false,
            TXDMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAENR {
        match value {
            false => TXDMAENR::DISABLED,
            true => TXDMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAENR::ENABLED
    }
}
#[doc = "Possible values of the field `RXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAENR {
    #[doc = "Rx buffer DMA disabled"]
    DISABLED,
    #[doc = "Rx buffer DMA enabled"]
    ENABLED,
}
impl RXDMAENR {
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
            RXDMAENR::DISABLED => false,
            RXDMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMAENR {
        match value {
            false => RXDMAENR::DISABLED,
            true => RXDMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `TXEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIEW {
    #[doc = "TXE interrupt masked"]
    MASKED,
    #[doc = "TXE interrupt not masked"]
    NOTMASKED,
}
impl TXEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEIEW::MASKED => false,
            TXEIEW::NOTMASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXE interrupt masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIEW::MASKED)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIEW::NOTMASKED)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXNEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIEW {
    #[doc = "RXE interrupt masked"]
    MASKED,
    #[doc = "RXE interrupt not masked"]
    NOTMASKED,
}
impl RXNEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXNEIEW::MASKED => false,
            RXNEIEW::NOTMASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXNEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXNEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXE interrupt masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIEW::MASKED)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIEW::NOTMASKED)
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
#[doc = "Values that can be written to the field `ERRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIEW {
    #[doc = "Error interrupt masked"]
    MASKED,
    #[doc = "Error interrupt not masked"]
    NOTMASKED,
}
impl ERRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRIEW::MASKED => false,
            ERRIEW::NOTMASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error interrupt masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIEW::MASKED)
    }
    #[doc = "Error interrupt not masked"]
    #[inline]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIEW::NOTMASKED)
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
#[doc = "Values that can be written to the field `SSOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOEW {
    #[doc = "SS output is disabled in master mode"]
    DISABLED,
    #[doc = "SS output is enabled in master mode"]
    ENABLED,
}
impl SSOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSOEW::DISABLED => false,
            SSOEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSOEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SS output is disabled in master mode"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOEW::DISABLED)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOEW::ENABLED)
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
#[doc = "Values that can be written to the field `TXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAENW {
    #[doc = "Tx buffer DMA disabled"]
    DISABLED,
    #[doc = "Tx buffer DMA enabled"]
    ENABLED,
}
impl TXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAENW::DISABLED => false,
            TXDMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAENW::DISABLED)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAENW::ENABLED)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAENW {
    #[doc = "Rx buffer DMA disabled"]
    DISABLED,
    #[doc = "Rx buffer DMA enabled"]
    ENABLED,
}
impl RXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMAENW::DISABLED => false,
            RXDMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAENW::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAENW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline]
    pub fn txeie(&self) -> TXEIER {
        TXEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline]
    pub fn rxneie(&self) -> RXNEIER {
        RXNEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline]
    pub fn errie(&self) -> ERRIER {
        ERRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline]
    pub fn ssoe(&self) -> SSOER {
        SSOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline]
    pub fn txdmaen(&self) -> TXDMAENR {
        TXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline]
    pub fn rxdmaen(&self) -> RXDMAENR {
        RXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline]
    pub fn txeie(&mut self) -> _TXEIEW {
        _TXEIEW { w: self }
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline]
    pub fn rxneie(&mut self) -> _RXNEIEW {
        _RXNEIEW { w: self }
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline]
    pub fn errie(&mut self) -> _ERRIEW {
        _ERRIEW { w: self }
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline]
    pub fn ssoe(&mut self) -> _SSOEW {
        _SSOEW { w: self }
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline]
    pub fn txdmaen(&mut self) -> _TXDMAENW {
        _TXDMAENW { w: self }
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline]
    pub fn rxdmaen(&mut self) -> _RXDMAENW {
        _RXDMAENW { w: self }
    }
}
