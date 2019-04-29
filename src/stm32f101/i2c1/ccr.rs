#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `F_S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F_SR {
    #[doc = "Standard mode I2C"]
    STANDARD,
    #[doc = "Fast mode I2C"]
    FAST,
}
impl F_SR {
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
            F_SR::STANDARD => false,
            F_SR::FAST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> F_SR {
        match value {
            false => F_SR::STANDARD,
            true => F_SR::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == F_SR::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline]
    pub fn is_fast(&self) -> bool {
        *self == F_SR::FAST
    }
}
#[doc = "Possible values of the field `DUTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUTYR {
    #[doc = "Duty cycle t_low/t_high = 2/1"]
    DUTY2_1,
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    DUTY16_9,
}
impl DUTYR {
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
            DUTYR::DUTY2_1 => false,
            DUTYR::DUTY16_9 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DUTYR {
        match value {
            false => DUTYR::DUTY2_1,
            true => DUTYR::DUTY16_9,
        }
    }
    #[doc = "Checks if the value of the field is `DUTY2_1`"]
    #[inline]
    pub fn is_duty2_1(&self) -> bool {
        *self == DUTYR::DUTY2_1
    }
    #[doc = "Checks if the value of the field is `DUTY16_9`"]
    #[inline]
    pub fn is_duty16_9(&self) -> bool {
        *self == DUTYR::DUTY16_9
    }
}
#[doc = r" Value of the field"]
pub struct CCRR {
    bits: u16,
}
impl CCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `F_S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F_SW {
    #[doc = "Standard mode I2C"]
    STANDARD,
    #[doc = "Fast mode I2C"]
    FAST,
}
impl F_SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            F_SW::STANDARD => false,
            F_SW::FAST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _F_SW<'a> {
    w: &'a mut W,
}
impl<'a> _F_SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: F_SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard mode I2C"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(F_SW::STANDARD)
    }
    #[doc = "Fast mode I2C"]
    #[inline]
    pub fn fast(self) -> &'a mut W {
        self.variant(F_SW::FAST)
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
#[doc = "Values that can be written to the field `DUTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUTYW {
    #[doc = "Duty cycle t_low/t_high = 2/1"]
    DUTY2_1,
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    DUTY16_9,
}
impl DUTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DUTYW::DUTY2_1 => false,
            DUTYW::DUTY16_9 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DUTYW<'a> {
    w: &'a mut W,
}
impl<'a> _DUTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DUTYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Duty cycle t_low/t_high = 2/1"]
    #[inline]
    pub fn duty2_1(self) -> &'a mut W {
        self.variant(DUTYW::DUTY2_1)
    }
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    #[inline]
    pub fn duty16_9(self) -> &'a mut W {
        self.variant(DUTYW::DUTY16_9)
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
#[doc = r" Proxy"]
pub struct _CCRW<'a> {
    w: &'a mut W,
}
impl<'a> _CCRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
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
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline]
    pub fn f_s(&self) -> F_SR {
        F_SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline]
    pub fn duty(&self) -> DUTYR {
        DUTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline]
    pub fn ccr(&self) -> CCRR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CCRR { bits }
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
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline]
    pub fn f_s(&mut self) -> _F_SW {
        _F_SW { w: self }
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline]
    pub fn duty(&mut self) -> _DUTYW {
        _DUTYW { w: self }
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline]
    pub fn ccr(&mut self) -> _CCRW {
        _CCRW { w: self }
    }
}
