#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
}
#[doc = "Values that can be written to the field `RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETW {
    #[doc = "Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"]
    RESET,
}
impl RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETW::RESET)
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
    #[doc = "Bit 0 - Reset bit"]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
}
