#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB2RSTR {
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
#[doc = "Possible values of the field `AFIORST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIORSTR {
    #[doc = "Reset the selected module"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl AFIORSTR {
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
            AFIORSTR::RESET => true,
            AFIORSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AFIORSTR {
        match value {
            true => AFIORSTR::RESET,
            i => AFIORSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == AFIORSTR::RESET
    }
}
#[doc = "Possible values of the field `IOPARST`"]
pub type IOPARSTR = AFIORSTR;
#[doc = "Possible values of the field `IOPBRST`"]
pub type IOPBRSTR = AFIORSTR;
#[doc = "Possible values of the field `IOPCRST`"]
pub type IOPCRSTR = AFIORSTR;
#[doc = "Possible values of the field `IOPDRST`"]
pub type IOPDRSTR = AFIORSTR;
#[doc = "Possible values of the field `IOPERST`"]
pub type IOPERSTR = AFIORSTR;
#[doc = "Possible values of the field `ADC1RST`"]
pub type ADC1RSTR = AFIORSTR;
#[doc = "Possible values of the field `SPI1RST`"]
pub type SPI1RSTR = AFIORSTR;
#[doc = "Possible values of the field `USART1RST`"]
pub type USART1RSTR = AFIORSTR;
#[doc = "Possible values of the field `TIM9RST`"]
pub type TIM9RSTR = AFIORSTR;
#[doc = "Possible values of the field `TIM10RST`"]
pub type TIM10RSTR = AFIORSTR;
#[doc = "Possible values of the field `TIM11RST`"]
pub type TIM11RSTR = AFIORSTR;
#[doc = "Possible values of the field `IOPFRST`"]
pub type IOPFRSTR = AFIORSTR;
#[doc = "Possible values of the field `IOPGRST`"]
pub type IOPGRSTR = AFIORSTR;
#[doc = "Values that can be written to the field `AFIORST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIORSTW {
    #[doc = "Reset the selected module"]
    RESET,
}
impl AFIORSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AFIORSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFIORSTW<'a> {
    w: &'a mut W,
}
impl<'a> _AFIORSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFIORSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPARST`"]
pub type IOPARSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _IOPARSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPARSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPARSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPBRST`"]
pub type IOPBRSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _IOPBRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPBRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPBRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPCRST`"]
pub type IOPCRSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _IOPCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPDRST`"]
pub type IOPDRSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _IOPDRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPDRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPDRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPERST`"]
pub type IOPERSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _IOPERSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPERSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPERSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `ADC1RST`"]
pub type ADC1RSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _ADC1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `SPI1RST`"]
pub type SPI1RSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _SPI1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `USART1RST`"]
pub type USART1RSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _USART1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM9RST`"]
pub type TIM9RSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _TIM9RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM9RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM9RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM10RST`"]
pub type TIM10RSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _TIM10RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM10RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM10RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM11RST`"]
pub type TIM11RSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _TIM11RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM11RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM11RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IOPFRST`"]
pub type IOPFRSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _IOPFRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPFRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPFRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPGRST`"]
pub type IOPGRSTW = AFIORSTW;
#[doc = r" Proxy"]
pub struct _IOPGRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPGRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPGRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORSTW::RESET)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline]
    pub fn afiorst(&self) -> AFIORSTR {
        AFIORSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline]
    pub fn ioparst(&self) -> IOPARSTR {
        IOPARSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline]
    pub fn iopbrst(&self) -> IOPBRSTR {
        IOPBRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline]
    pub fn iopcrst(&self) -> IOPCRSTR {
        IOPCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline]
    pub fn iopdrst(&self) -> IOPDRSTR {
        IOPDRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline]
    pub fn ioperst(&self) -> IOPERSTR {
        IOPERSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ADC 1 interface reset"]
    #[inline]
    pub fn adc1rst(&self) -> ADC1RSTR {
        ADC1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline]
    pub fn spi1rst(&self) -> SPI1RSTR {
        SPI1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline]
    pub fn usart1rst(&self) -> USART1RSTR {
        USART1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - TIM9 timer reset"]
    #[inline]
    pub fn tim9rst(&self) -> TIM9RSTR {
        TIM9RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - TIM10 timer reset"]
    #[inline]
    pub fn tim10rst(&self) -> TIM10RSTR {
        TIM10RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - TIM11 timer reset"]
    #[inline]
    pub fn tim11rst(&self) -> TIM11RSTR {
        TIM11RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - IO port F reset"]
    #[inline]
    pub fn iopfrst(&self) -> IOPFRSTR {
        IOPFRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - IO port G reset"]
    #[inline]
    pub fn iopgrst(&self) -> IOPGRSTR {
        IOPGRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline]
    pub fn afiorst(&mut self) -> _AFIORSTW {
        _AFIORSTW { w: self }
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline]
    pub fn ioparst(&mut self) -> _IOPARSTW {
        _IOPARSTW { w: self }
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline]
    pub fn iopbrst(&mut self) -> _IOPBRSTW {
        _IOPBRSTW { w: self }
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline]
    pub fn iopcrst(&mut self) -> _IOPCRSTW {
        _IOPCRSTW { w: self }
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline]
    pub fn iopdrst(&mut self) -> _IOPDRSTW {
        _IOPDRSTW { w: self }
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline]
    pub fn ioperst(&mut self) -> _IOPERSTW {
        _IOPERSTW { w: self }
    }
    #[doc = "Bit 9 - ADC 1 interface reset"]
    #[inline]
    pub fn adc1rst(&mut self) -> _ADC1RSTW {
        _ADC1RSTW { w: self }
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline]
    pub fn spi1rst(&mut self) -> _SPI1RSTW {
        _SPI1RSTW { w: self }
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline]
    pub fn usart1rst(&mut self) -> _USART1RSTW {
        _USART1RSTW { w: self }
    }
    #[doc = "Bit 19 - TIM9 timer reset"]
    #[inline]
    pub fn tim9rst(&mut self) -> _TIM9RSTW {
        _TIM9RSTW { w: self }
    }
    #[doc = "Bit 20 - TIM10 timer reset"]
    #[inline]
    pub fn tim10rst(&mut self) -> _TIM10RSTW {
        _TIM10RSTW { w: self }
    }
    #[doc = "Bit 21 - TIM11 timer reset"]
    #[inline]
    pub fn tim11rst(&mut self) -> _TIM11RSTW {
        _TIM11RSTW { w: self }
    }
    #[doc = "Bit 7 - IO port F reset"]
    #[inline]
    pub fn iopfrst(&mut self) -> _IOPFRSTW {
        _IOPFRSTW { w: self }
    }
    #[doc = "Bit 8 - IO port G reset"]
    #[inline]
    pub fn iopgrst(&mut self) -> _IOPGRSTW {
        _IOPGRSTW { w: self }
    }
}
