#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR3 {
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
#[doc = "Possible values of the field `CTSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIER {
    #[doc = "CTS interrupt disabled"]
    DISABLED,
    #[doc = "CTS interrupt enabled"]
    ENABLED,
}
impl CTSIER {
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
            CTSIER::DISABLED => false,
            CTSIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSIER {
        match value {
            false => CTSIER::DISABLED,
            true => CTSIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIER::ENABLED
    }
}
#[doc = "Possible values of the field `CTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSER {
    #[doc = "CTS hardware flow control disabled"]
    DISABLED,
    #[doc = "CTS hardware flow control enabled"]
    ENABLED,
}
impl CTSER {
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
            CTSER::DISABLED => false,
            CTSER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSER {
        match value {
            false => CTSER::DISABLED,
            true => CTSER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CTSER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CTSER::ENABLED
    }
}
#[doc = "Possible values of the field `RTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSER {
    #[doc = "RTS hardware flow control disabled"]
    DISABLED,
    #[doc = "RTS hardware flow control enabled"]
    ENABLED,
}
impl RTSER {
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
            RTSER::DISABLED => false,
            RTSER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSER {
        match value {
            false => RTSER::DISABLED,
            true => RTSER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RTSER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RTSER::ENABLED
    }
}
#[doc = "Possible values of the field `DMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATR {
    #[doc = "DMA mode is disabled for transmission"]
    DISABLED,
    #[doc = "DMA mode is enabled for transmission"]
    ENABLED,
}
impl DMATR {
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
            DMATR::DISABLED => false,
            DMATR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMATR {
        match value {
            false => DMATR::DISABLED,
            true => DMATR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMATR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMATR::ENABLED
    }
}
#[doc = "Possible values of the field `DMAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARR {
    #[doc = "DMA mode is disabled for reception"]
    DISABLED,
    #[doc = "DMA mode is enabled for reception"]
    ENABLED,
}
impl DMARR {
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
            DMARR::DISABLED => false,
            DMARR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMARR {
        match value {
            false => DMARR::DISABLED,
            true => DMARR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMARR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMARR::ENABLED
    }
}
#[doc = "Possible values of the field `SCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCENR {
    #[doc = "Smartcard mode disabled"]
    DISABLED,
    #[doc = "Smartcard mode enabled"]
    ENABLED,
}
impl SCENR {
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
            SCENR::DISABLED => false,
            SCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCENR {
        match value {
            false => SCENR::DISABLED,
            true => SCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SCENR::ENABLED
    }
}
#[doc = "Possible values of the field `NACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKR {
    #[doc = "NACK transmission in case of parity error is disabled"]
    DISABLED,
    #[doc = "NACK transmission during parity error is enabled"]
    ENABLED,
}
impl NACKR {
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
            NACKR::DISABLED => false,
            NACKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKR {
        match value {
            false => NACKR::DISABLED,
            true => NACKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NACKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NACKR::ENABLED
    }
}
#[doc = "Possible values of the field `HDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSELR {
    #[doc = "Half duplex mode is not selected"]
    FULLDUPLEX,
    #[doc = "Half duplex mode is selected"]
    HALFDUPLEX,
}
impl HDSELR {
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
            HDSELR::FULLDUPLEX => false,
            HDSELR::HALFDUPLEX => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDSELR {
        match value {
            false => HDSELR::FULLDUPLEX,
            true => HDSELR::HALFDUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `FULLDUPLEX`"]
    #[inline]
    pub fn is_full_duplex(&self) -> bool {
        *self == HDSELR::FULLDUPLEX
    }
    #[doc = "Checks if the value of the field is `HALFDUPLEX`"]
    #[inline]
    pub fn is_half_duplex(&self) -> bool {
        *self == HDSELR::HALFDUPLEX
    }
}
#[doc = "Possible values of the field `IRLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRLPR {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "Low-power mode"]
    LOWPOWER,
}
impl IRLPR {
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
            IRLPR::NORMAL => false,
            IRLPR::LOWPOWER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRLPR {
        match value {
            false => IRLPR::NORMAL,
            true => IRLPR::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == IRLPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline]
    pub fn is_low_power(&self) -> bool {
        *self == IRLPR::LOWPOWER
    }
}
#[doc = "Possible values of the field `IREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRENR {
    #[doc = "IrDA disabled"]
    DISABLED,
    #[doc = "IrDA enabled"]
    ENABLED,
}
impl IRENR {
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
            IRENR::DISABLED => false,
            IRENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRENR {
        match value {
            false => IRENR::DISABLED,
            true => IRENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IRENR::ENABLED
    }
}
#[doc = "Possible values of the field `EIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIER {
    #[doc = "Error interrupt disabled"]
    DISABLED,
    #[doc = "Error interrupt enabled"]
    ENABLED,
}
impl EIER {
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
            EIER::DISABLED => false,
            EIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EIER {
        match value {
            false => EIER::DISABLED,
            true => EIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EIER::ENABLED
    }
}
#[doc = "Values that can be written to the field `CTSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIEW {
    #[doc = "CTS interrupt disabled"]
    DISABLED,
    #[doc = "CTS interrupt enabled"]
    ENABLED,
}
impl CTSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSIEW::DISABLED => false,
            CTSIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CTS interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSIEW::DISABLED)
    }
    #[doc = "CTS interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSIEW::ENABLED)
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
#[doc = "Values that can be written to the field `CTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEW {
    #[doc = "CTS hardware flow control disabled"]
    DISABLED,
    #[doc = "CTS hardware flow control enabled"]
    ENABLED,
}
impl CTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSEW::DISABLED => false,
            CTSEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSEW::DISABLED)
    }
    #[doc = "CTS hardware flow control enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSEW::ENABLED)
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
#[doc = "Values that can be written to the field `RTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSEW {
    #[doc = "RTS hardware flow control disabled"]
    DISABLED,
    #[doc = "RTS hardware flow control enabled"]
    ENABLED,
}
impl RTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSEW::DISABLED => false,
            RTSEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTSEW::DISABLED)
    }
    #[doc = "RTS hardware flow control enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTSEW::ENABLED)
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
#[doc = "Values that can be written to the field `DMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATW {
    #[doc = "DMA mode is disabled for transmission"]
    DISABLED,
    #[doc = "DMA mode is enabled for transmission"]
    ENABLED,
}
impl DMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMATW::DISABLED => false,
            DMATW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMATW<'a> {
    w: &'a mut W,
}
impl<'a> _DMATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATW::DISABLED)
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATW::ENABLED)
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
#[doc = "Values that can be written to the field `DMAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARW {
    #[doc = "DMA mode is disabled for reception"]
    DISABLED,
    #[doc = "DMA mode is enabled for reception"]
    ENABLED,
}
impl DMARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMARW::DISABLED => false,
            DMARW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMARW<'a> {
    w: &'a mut W,
}
impl<'a> _DMARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMARW::DISABLED)
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMARW::ENABLED)
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
#[doc = "Values that can be written to the field `SCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCENW {
    #[doc = "Smartcard mode disabled"]
    DISABLED,
    #[doc = "Smartcard mode enabled"]
    ENABLED,
}
impl SCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCENW::DISABLED => false,
            SCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Smartcard mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCENW::DISABLED)
    }
    #[doc = "Smartcard mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCENW::ENABLED)
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
#[doc = "Values that can be written to the field `NACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKW {
    #[doc = "NACK transmission in case of parity error is disabled"]
    DISABLED,
    #[doc = "NACK transmission during parity error is enabled"]
    ENABLED,
}
impl NACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKW::DISABLED => false,
            NACKW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKW::DISABLED)
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKW::ENABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSELW {
    #[doc = "Half duplex mode is not selected"]
    FULLDUPLEX,
    #[doc = "Half duplex mode is selected"]
    HALFDUPLEX,
}
impl HDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDSELW::FULLDUPLEX => false,
            HDSELW::HALFDUPLEX => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Half duplex mode is not selected"]
    #[inline]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(HDSELW::FULLDUPLEX)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(HDSELW::HALFDUPLEX)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRLPW {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "Low-power mode"]
    LOWPOWER,
}
impl IRLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRLPW::NORMAL => false,
            IRLPW::LOWPOWER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRLPW<'a> {
    w: &'a mut W,
}
impl<'a> _IRLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRLPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLPW::NORMAL)
    }
    #[doc = "Low-power mode"]
    #[inline]
    pub fn low_power(self) -> &'a mut W {
        self.variant(IRLPW::LOWPOWER)
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
#[doc = "Values that can be written to the field `IREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRENW {
    #[doc = "IrDA disabled"]
    DISABLED,
    #[doc = "IrDA enabled"]
    ENABLED,
}
impl IRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRENW::DISABLED => false,
            IRENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IrDA disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IRENW::DISABLED)
    }
    #[doc = "IrDA enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IRENW::ENABLED)
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
#[doc = "Values that can be written to the field `EIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIEW {
    #[doc = "Error interrupt disabled"]
    DISABLED,
    #[doc = "Error interrupt enabled"]
    ENABLED,
}
impl EIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EIEW::DISABLED => false,
            EIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIEW::DISABLED)
    }
    #[doc = "Error interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIEW::ENABLED)
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
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline]
    pub fn ctsie(&self) -> CTSIER {
        CTSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline]
    pub fn ctse(&self) -> CTSER {
        CTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline]
    pub fn rtse(&self) -> RTSER {
        RTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline]
    pub fn dmat(&self) -> DMATR {
        DMATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline]
    pub fn dmar(&self) -> DMARR {
        DMARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline]
    pub fn scen(&self) -> SCENR {
        SCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline]
    pub fn nack(&self) -> NACKR {
        NACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline]
    pub fn hdsel(&self) -> HDSELR {
        HDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline]
    pub fn irlp(&self) -> IRLPR {
        IRLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline]
    pub fn iren(&self) -> IRENR {
        IRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline]
    pub fn eie(&self) -> EIER {
        EIER::_from({
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
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline]
    pub fn ctsie(&mut self) -> _CTSIEW {
        _CTSIEW { w: self }
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline]
    pub fn ctse(&mut self) -> _CTSEW {
        _CTSEW { w: self }
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline]
    pub fn rtse(&mut self) -> _RTSEW {
        _RTSEW { w: self }
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline]
    pub fn dmat(&mut self) -> _DMATW {
        _DMATW { w: self }
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline]
    pub fn dmar(&mut self) -> _DMARW {
        _DMARW { w: self }
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline]
    pub fn scen(&mut self) -> _SCENW {
        _SCENW { w: self }
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline]
    pub fn nack(&mut self) -> _NACKW {
        _NACKW { w: self }
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline]
    pub fn hdsel(&mut self) -> _HDSELW {
        _HDSELW { w: self }
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline]
    pub fn irlp(&mut self) -> _IRLPW {
        _IRLPW { w: self }
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline]
    pub fn iren(&mut self) -> _IRENW {
        _IRENW { w: self }
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline]
    pub fn eie(&mut self) -> _EIEW {
        _EIEW { w: self }
    }
}
