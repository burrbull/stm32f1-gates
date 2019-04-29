#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OAR1 {
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
#[doc = "Possible values of the field `ADDMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDMODER {
    #[doc = "7-bit slave address"]
    ADD7,
    #[doc = "10-bit slave address"]
    ADD10,
}
impl ADDMODER {
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
            ADDMODER::ADD7 => false,
            ADDMODER::ADD10 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDMODER {
        match value {
            false => ADDMODER::ADD7,
            true => ADDMODER::ADD10,
        }
    }
    #[doc = "Checks if the value of the field is `ADD7`"]
    #[inline]
    pub fn is_add7(&self) -> bool {
        *self == ADDMODER::ADD7
    }
    #[doc = "Checks if the value of the field is `ADD10`"]
    #[inline]
    pub fn is_add10(&self) -> bool {
        *self == ADDMODER::ADD10
    }
}
#[doc = r" Value of the field"]
pub struct ADDR {
    bits: u16,
}
impl ADDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ADDMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDMODEW {
    #[doc = "7-bit slave address"]
    ADD7,
    #[doc = "10-bit slave address"]
    ADD10,
}
impl ADDMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDMODEW::ADD7 => false,
            ADDMODEW::ADD10 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "7-bit slave address"]
    #[inline]
    pub fn add7(self) -> &'a mut W {
        self.variant(ADDMODEW::ADD7)
    }
    #[doc = "10-bit slave address"]
    #[inline]
    pub fn add10(self) -> &'a mut W {
        self.variant(ADDMODEW::ADD10)
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
#[doc = r" Proxy"]
pub struct _ADDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
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
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline]
    pub fn addmode(&self) -> ADDMODER {
        ADDMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:9 - Interface address"]
    #[inline]
    pub fn add(&self) -> ADDR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ADDR { bits }
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
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline]
    pub fn addmode(&mut self) -> _ADDMODEW {
        _ADDMODEW { w: self }
    }
    #[doc = "Bits 0:9 - Interface address"]
    #[inline]
    pub fn add(&mut self) -> _ADDW {
        _ADDW { w: self }
    }
}
