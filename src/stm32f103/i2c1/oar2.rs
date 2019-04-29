#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OAR2 {
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
pub struct ADD2R {
    bits: u8,
}
impl ADD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENDUAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDUALR {
    #[doc = "Single addressing mode"]
    SINGLE,
    #[doc = "Dual addressing mode"]
    DUAL,
}
impl ENDUALR {
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
            ENDUALR::SINGLE => false,
            ENDUALR::DUAL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDUALR {
        match value {
            false => ENDUALR::SINGLE,
            true => ENDUALR::DUAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == ENDUALR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == ENDUALR::DUAL
    }
}
#[doc = r" Proxy"]
pub struct _ADD2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADD2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENDUAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDUALW {
    #[doc = "Single addressing mode"]
    SINGLE,
    #[doc = "Dual addressing mode"]
    DUAL,
}
impl ENDUALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDUALW::SINGLE => false,
            ENDUALW::DUAL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDUALW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDUALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDUALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single addressing mode"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(ENDUALW::SINGLE)
    }
    #[doc = "Dual addressing mode"]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(ENDUALW::DUAL)
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
    #[doc = "Bits 1:7 - Interface address"]
    #[inline]
    pub fn add2(&self) -> ADD2R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADD2R { bits }
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline]
    pub fn endual(&self) -> ENDUALR {
        ENDUALR::_from({
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
    #[doc = "Bits 1:7 - Interface address"]
    #[inline]
    pub fn add2(&mut self) -> _ADD2W {
        _ADD2W { w: self }
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline]
    pub fn endual(&mut self) -> _ENDUALW {
        _ENDUALW { w: self }
    }
}
