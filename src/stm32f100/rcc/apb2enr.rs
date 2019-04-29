#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB2ENR {
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
#[doc = "Possible values of the field `AFIOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIOENR {
    #[doc = "The selected clock is disabled"]
    DISABLED,
    #[doc = "The selected clock is enabled"]
    ENABLED,
}
impl AFIOENR {
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
            AFIOENR::DISABLED => false,
            AFIOENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AFIOENR {
        match value {
            false => AFIOENR::DISABLED,
            true => AFIOENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AFIOENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AFIOENR::ENABLED
    }
}
#[doc = "Possible values of the field `IOPAEN`"]
pub type IOPAENR = AFIOENR;
#[doc = "Possible values of the field `IOPBEN`"]
pub type IOPBENR = AFIOENR;
#[doc = "Possible values of the field `IOPCEN`"]
pub type IOPCENR = AFIOENR;
#[doc = "Possible values of the field `IOPDEN`"]
pub type IOPDENR = AFIOENR;
#[doc = "Possible values of the field `IOPEEN`"]
pub type IOPEENR = AFIOENR;
#[doc = "Possible values of the field `IOPFEN`"]
pub type IOPFENR = AFIOENR;
#[doc = "Possible values of the field `IOPGEN`"]
pub type IOPGENR = AFIOENR;
#[doc = "Possible values of the field `ADC1EN`"]
pub type ADC1ENR = AFIOENR;
#[doc = "Possible values of the field `TIM1EN`"]
pub type TIM1ENR = AFIOENR;
#[doc = "Possible values of the field `SPI1EN`"]
pub type SPI1ENR = AFIOENR;
#[doc = "Possible values of the field `USART1EN`"]
pub type USART1ENR = AFIOENR;
#[doc = "Possible values of the field `TIM15EN`"]
pub type TIM15ENR = AFIOENR;
#[doc = "Possible values of the field `TIM16EN`"]
pub type TIM16ENR = AFIOENR;
#[doc = "Possible values of the field `TIM17EN`"]
pub type TIM17ENR = AFIOENR;
#[doc = "Values that can be written to the field `AFIOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIOENW {
    #[doc = "The selected clock is disabled"]
    DISABLED,
    #[doc = "The selected clock is enabled"]
    ENABLED,
}
impl AFIOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AFIOENW::DISABLED => false,
            AFIOENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFIOENW<'a> {
    w: &'a mut W,
}
impl<'a> _AFIOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFIOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPAEN`"]
pub type IOPAENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _IOPAENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPBEN`"]
pub type IOPBENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _IOPBENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPBENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPBENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPCEN`"]
pub type IOPCENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _IOPCENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPDEN`"]
pub type IOPDENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _IOPDENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPEEN`"]
pub type IOPEENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _IOPEENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPFEN`"]
pub type IOPFENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _IOPFENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPGEN`"]
pub type IOPGENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _IOPGENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `ADC1EN`"]
pub type ADC1ENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _ADC1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM1EN`"]
pub type TIM1ENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _TIM1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `SPI1EN`"]
pub type SPI1ENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _SPI1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `USART1EN`"]
pub type USART1ENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _USART1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM15EN`"]
pub type TIM15ENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _TIM15ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM15ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM15ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM16EN`"]
pub type TIM16ENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _TIM16ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM16ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM16ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM17EN`"]
pub type TIM17ENW = AFIOENW;
#[doc = r" Proxy"]
pub struct _TIM17ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM17ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM17ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOENW::ENABLED)
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
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline]
    pub fn afioen(&self) -> AFIOENR {
        AFIOENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline]
    pub fn iopaen(&self) -> IOPAENR {
        IOPAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline]
    pub fn iopben(&self) -> IOPBENR {
        IOPBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline]
    pub fn iopcen(&self) -> IOPCENR {
        IOPCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline]
    pub fn iopden(&self) -> IOPDENR {
        IOPDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline]
    pub fn iopeen(&self) -> IOPEENR {
        IOPEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline]
    pub fn iopfen(&self) -> IOPFENR {
        IOPFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline]
    pub fn iopgen(&self) -> IOPGENR {
        IOPGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline]
    pub fn adc1en(&self) -> ADC1ENR {
        ADC1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline]
    pub fn tim1en(&self) -> TIM1ENR {
        TIM1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline]
    pub fn spi1en(&self) -> SPI1ENR {
        SPI1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline]
    pub fn usart1en(&self) -> USART1ENR {
        USART1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TIM15 Timer clock enable"]
    #[inline]
    pub fn tim15en(&self) -> TIM15ENR {
        TIM15ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TIM16 Timer clock enable"]
    #[inline]
    pub fn tim16en(&self) -> TIM16ENR {
        TIM16ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - TIM17 Timer clock enable"]
    #[inline]
    pub fn tim17en(&self) -> TIM17ENR {
        TIM17ENR::_from({
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
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline]
    pub fn afioen(&mut self) -> _AFIOENW {
        _AFIOENW { w: self }
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline]
    pub fn iopaen(&mut self) -> _IOPAENW {
        _IOPAENW { w: self }
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline]
    pub fn iopben(&mut self) -> _IOPBENW {
        _IOPBENW { w: self }
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline]
    pub fn iopcen(&mut self) -> _IOPCENW {
        _IOPCENW { w: self }
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline]
    pub fn iopden(&mut self) -> _IOPDENW {
        _IOPDENW { w: self }
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline]
    pub fn iopeen(&mut self) -> _IOPEENW {
        _IOPEENW { w: self }
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline]
    pub fn iopfen(&mut self) -> _IOPFENW {
        _IOPFENW { w: self }
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline]
    pub fn iopgen(&mut self) -> _IOPGENW {
        _IOPGENW { w: self }
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline]
    pub fn adc1en(&mut self) -> _ADC1ENW {
        _ADC1ENW { w: self }
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline]
    pub fn tim1en(&mut self) -> _TIM1ENW {
        _TIM1ENW { w: self }
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline]
    pub fn spi1en(&mut self) -> _SPI1ENW {
        _SPI1ENW { w: self }
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline]
    pub fn usart1en(&mut self) -> _USART1ENW {
        _USART1ENW { w: self }
    }
    #[doc = "Bit 16 - TIM15 Timer clock enable"]
    #[inline]
    pub fn tim15en(&mut self) -> _TIM15ENW {
        _TIM15ENW { w: self }
    }
    #[doc = "Bit 17 - TIM16 Timer clock enable"]
    #[inline]
    pub fn tim16en(&mut self) -> _TIM16ENW {
        _TIM16ENW { w: self }
    }
    #[doc = "Bit 18 - TIM17 Timer clock enable"]
    #[inline]
    pub fn tim17en(&mut self) -> _TIM17ENW {
        _TIM17ENW { w: self }
    }
}
