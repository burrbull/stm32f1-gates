#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
pub struct LPDSR {
    bits: bool,
}
impl LPDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PDDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDDSR {
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    STOP_MODE,
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    STANDBY_MODE,
}
impl PDDSR {
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
            PDDSR::STOP_MODE => false,
            PDDSR::STANDBY_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDDSR {
        match value {
            false => PDDSR::STOP_MODE,
            true => PDDSR::STANDBY_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_MODE`"]
    #[inline]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDSR::STOP_MODE
    }
    #[doc = "Checks if the value of the field is `STANDBY_MODE`"]
    #[inline]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDSR::STANDBY_MODE
    }
}
#[doc = r" Value of the field"]
pub struct CWUFR {
    bits: bool,
}
impl CWUFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CSBFR {
    bits: bool,
}
impl CSBFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PVDER {
    bits: bool,
}
impl PVDER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PLSR {
    bits: u8,
}
impl PLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DBPR {
    bits: bool,
}
impl DBPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _LPDSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPDSW<'a> {
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
#[doc = "Values that can be written to the field `PDDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDDSW {
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    STOP_MODE,
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    STANDBY_MODE,
}
impl PDDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDDSW::STOP_MODE => false,
            PDDSW::STANDBY_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDDSW<'a> {
    w: &'a mut W,
}
impl<'a> _PDDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline]
    pub fn stop_mode(self) -> &'a mut W {
        self.variant(PDDSW::STOP_MODE)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline]
    pub fn standby_mode(self) -> &'a mut W {
        self.variant(PDDSW::STANDBY_MODE)
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
#[doc = r" Proxy"]
pub struct _CWUFW<'a> {
    w: &'a mut W,
}
impl<'a> _CWUFW<'a> {
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
#[doc = r" Proxy"]
pub struct _CSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _CSBFW<'a> {
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
#[doc = r" Proxy"]
pub struct _PVDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PVDEW<'a> {
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
#[doc = r" Proxy"]
pub struct _PLSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DBPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBPW<'a> {
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
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline]
    pub fn lpds(&self) -> LPDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPDSR { bits }
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline]
    pub fn pdds(&self) -> PDDSR {
        PDDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline]
    pub fn cwuf(&self) -> CWUFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CWUFR { bits }
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline]
    pub fn csbf(&self) -> CSBFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSBFR { bits }
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline]
    pub fn pvde(&self) -> PVDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PVDER { bits }
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline]
    pub fn pls(&self) -> PLSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLSR { bits }
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline]
    pub fn dbp(&self) -> DBPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBPR { bits }
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
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline]
    pub fn lpds(&mut self) -> _LPDSW {
        _LPDSW { w: self }
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline]
    pub fn pdds(&mut self) -> _PDDSW {
        _PDDSW { w: self }
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline]
    pub fn cwuf(&mut self) -> _CWUFW {
        _CWUFW { w: self }
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline]
    pub fn csbf(&mut self) -> _CSBFW {
        _CSBFW { w: self }
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline]
    pub fn pvde(&mut self) -> _PVDEW {
        _PVDEW { w: self }
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline]
    pub fn pls(&mut self) -> _PLSW {
        _PLSW { w: self }
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline]
    pub fn dbp(&mut self) -> _DBPW {
        _DBPW { w: self }
    }
}
