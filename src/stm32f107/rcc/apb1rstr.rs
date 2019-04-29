#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB1RSTR {
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
#[doc = "Possible values of the field `TIM2RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2RSTR {
    #[doc = "Reset the selected module"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TIM2RSTR {
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
            TIM2RSTR::RESET => true,
            TIM2RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM2RSTR {
        match value {
            true => TIM2RSTR::RESET,
            i => TIM2RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM3RST`"]
pub type TIM3RSTR = TIM2RSTR;
#[doc = "Possible values of the field `TIM4RST`"]
pub type TIM4RSTR = TIM2RSTR;
#[doc = "Possible values of the field `TIM5RST`"]
pub type TIM5RSTR = TIM2RSTR;
#[doc = "Possible values of the field `TIM6RST`"]
pub type TIM6RSTR = TIM2RSTR;
#[doc = "Possible values of the field `TIM7RST`"]
pub type TIM7RSTR = TIM2RSTR;
#[doc = "Possible values of the field `WWDGRST`"]
pub type WWDGRSTR = TIM2RSTR;
#[doc = "Possible values of the field `SPI2RST`"]
pub type SPI2RSTR = TIM2RSTR;
#[doc = "Possible values of the field `SPI3RST`"]
pub type SPI3RSTR = TIM2RSTR;
#[doc = "Possible values of the field `USART2RST`"]
pub type USART2RSTR = TIM2RSTR;
#[doc = "Possible values of the field `USART3RST`"]
pub type USART3RSTR = TIM2RSTR;
#[doc = "Possible values of the field `UART4RST`"]
pub type UART4RSTR = TIM2RSTR;
#[doc = "Possible values of the field `UART5RST`"]
pub type UART5RSTR = TIM2RSTR;
#[doc = "Possible values of the field `I2C1RST`"]
pub type I2C1RSTR = TIM2RSTR;
#[doc = "Possible values of the field `I2C2RST`"]
pub type I2C2RSTR = TIM2RSTR;
#[doc = "Possible values of the field `CAN1RST`"]
pub type CAN1RSTR = TIM2RSTR;
#[doc = "Possible values of the field `CAN2RST`"]
pub type CAN2RSTR = TIM2RSTR;
#[doc = "Possible values of the field `BKPRST`"]
pub type BKPRSTR = TIM2RSTR;
#[doc = "Possible values of the field `PWRRST`"]
pub type PWRRSTR = TIM2RSTR;
#[doc = "Possible values of the field `DACRST`"]
pub type DACRSTR = TIM2RSTR;
#[doc = "Values that can be written to the field `TIM2RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2RSTW {
    #[doc = "Reset the selected module"]
    RESET,
}
impl TIM2RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM2RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM3RST`"]
pub type TIM3RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _TIM3RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM3RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM3RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM4RST`"]
pub type TIM4RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _TIM4RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM4RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM4RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM5RST`"]
pub type TIM5RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _TIM5RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM5RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM5RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM6RST`"]
pub type TIM6RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _TIM6RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM6RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM7RST`"]
pub type TIM7RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _TIM7RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM7RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM7RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `WWDGRST`"]
pub type WWDGRSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _WWDGRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDGRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDGRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `SPI2RST`"]
pub type SPI2RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _SPI2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `SPI3RST`"]
pub type SPI3RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _SPI3RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI3RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI3RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `USART2RST`"]
pub type USART2RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _USART2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `USART3RST`"]
pub type USART3RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _USART3RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART3RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `UART4RST`"]
pub type UART4RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _UART4RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _UART4RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART4RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `UART5RST`"]
pub type UART5RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _UART5RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _UART5RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART5RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `I2C1RST`"]
pub type I2C1RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _I2C1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `I2C2RST`"]
pub type I2C2RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _I2C2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAN1RST`"]
pub type CAN1RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _CAN1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAN2RST`"]
pub type CAN2RSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _CAN2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `BKPRST`"]
pub type BKPRSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _BKPRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BKPRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BKPRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `PWRRST`"]
pub type PWRRSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _PWRRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `DACRST`"]
pub type DACRSTW = TIM2RSTW;
#[doc = r" Proxy"]
pub struct _DACRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DACRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline]
    pub fn tim2rst(&self) -> TIM2RSTR {
        TIM2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline]
    pub fn tim3rst(&self) -> TIM3RSTR {
        TIM3RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline]
    pub fn tim4rst(&self) -> TIM4RSTR {
        TIM4RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline]
    pub fn tim5rst(&self) -> TIM5RSTR {
        TIM5RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline]
    pub fn tim6rst(&self) -> TIM6RSTR {
        TIM6RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline]
    pub fn tim7rst(&self) -> TIM7RSTR {
        TIM7RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline]
    pub fn wwdgrst(&self) -> WWDGRSTR {
        WWDGRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline]
    pub fn spi2rst(&self) -> SPI2RSTR {
        SPI2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline]
    pub fn spi3rst(&self) -> SPI3RSTR {
        SPI3RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline]
    pub fn usart2rst(&self) -> USART2RSTR {
        USART2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline]
    pub fn usart3rst(&self) -> USART3RSTR {
        USART3RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - USART 4 reset"]
    #[inline]
    pub fn uart4rst(&self) -> UART4RSTR {
        UART4RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - USART 5 reset"]
    #[inline]
    pub fn uart5rst(&self) -> UART5RSTR {
        UART5RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline]
    pub fn i2c1rst(&self) -> I2C1RSTR {
        I2C1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline]
    pub fn i2c2rst(&self) -> I2C2RSTR {
        I2C2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline]
    pub fn can1rst(&self) -> CAN1RSTR {
        CAN1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline]
    pub fn can2rst(&self) -> CAN2RSTR {
        CAN2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline]
    pub fn bkprst(&self) -> BKPRSTR {
        BKPRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline]
    pub fn pwrrst(&self) -> PWRRSTR {
        PWRRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline]
    pub fn dacrst(&self) -> DACRSTR {
        DACRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline]
    pub fn tim2rst(&mut self) -> _TIM2RSTW {
        _TIM2RSTW { w: self }
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline]
    pub fn tim3rst(&mut self) -> _TIM3RSTW {
        _TIM3RSTW { w: self }
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline]
    pub fn tim4rst(&mut self) -> _TIM4RSTW {
        _TIM4RSTW { w: self }
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline]
    pub fn tim5rst(&mut self) -> _TIM5RSTW {
        _TIM5RSTW { w: self }
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline]
    pub fn tim6rst(&mut self) -> _TIM6RSTW {
        _TIM6RSTW { w: self }
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline]
    pub fn tim7rst(&mut self) -> _TIM7RSTW {
        _TIM7RSTW { w: self }
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline]
    pub fn wwdgrst(&mut self) -> _WWDGRSTW {
        _WWDGRSTW { w: self }
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline]
    pub fn spi2rst(&mut self) -> _SPI2RSTW {
        _SPI2RSTW { w: self }
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline]
    pub fn spi3rst(&mut self) -> _SPI3RSTW {
        _SPI3RSTW { w: self }
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline]
    pub fn usart2rst(&mut self) -> _USART2RSTW {
        _USART2RSTW { w: self }
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline]
    pub fn usart3rst(&mut self) -> _USART3RSTW {
        _USART3RSTW { w: self }
    }
    #[doc = "Bit 19 - USART 4 reset"]
    #[inline]
    pub fn uart4rst(&mut self) -> _UART4RSTW {
        _UART4RSTW { w: self }
    }
    #[doc = "Bit 20 - USART 5 reset"]
    #[inline]
    pub fn uart5rst(&mut self) -> _UART5RSTW {
        _UART5RSTW { w: self }
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline]
    pub fn i2c1rst(&mut self) -> _I2C1RSTW {
        _I2C1RSTW { w: self }
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline]
    pub fn i2c2rst(&mut self) -> _I2C2RSTW {
        _I2C2RSTW { w: self }
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline]
    pub fn can1rst(&mut self) -> _CAN1RSTW {
        _CAN1RSTW { w: self }
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline]
    pub fn can2rst(&mut self) -> _CAN2RSTW {
        _CAN2RSTW { w: self }
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline]
    pub fn bkprst(&mut self) -> _BKPRSTW {
        _BKPRSTW { w: self }
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline]
    pub fn pwrrst(&mut self) -> _PWRRSTW {
        _PWRRSTW { w: self }
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline]
    pub fn dacrst(&mut self) -> _DACRSTW {
        _DACRSTW { w: self }
    }
}
