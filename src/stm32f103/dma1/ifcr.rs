#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFCR {
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
#[doc = "Values that can be written to the field `CGIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGIF1W {
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    CLEAR,
}
impl CGIF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CGIF1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF2`"]
pub type CGIF2W = CGIF1W;
#[doc = r" Proxy"]
pub struct _CGIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF3`"]
pub type CGIF3W = CGIF1W;
#[doc = r" Proxy"]
pub struct _CGIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF4`"]
pub type CGIF4W = CGIF1W;
#[doc = r" Proxy"]
pub struct _CGIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF5`"]
pub type CGIF5W = CGIF1W;
#[doc = r" Proxy"]
pub struct _CGIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF6`"]
pub type CGIF6W = CGIF1W;
#[doc = r" Proxy"]
pub struct _CGIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF7`"]
pub type CGIF7W = CGIF1W;
#[doc = r" Proxy"]
pub struct _CGIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1W::CLEAR)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF1W {
    #[doc = "Clears the TCIF flag in the ISR register"]
    CLEAR,
}
impl CTCIF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTCIF1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTCIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF2`"]
pub type CTCIF2W = CTCIF1W;
#[doc = r" Proxy"]
pub struct _CTCIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF3`"]
pub type CTCIF3W = CTCIF1W;
#[doc = r" Proxy"]
pub struct _CTCIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF4`"]
pub type CTCIF4W = CTCIF1W;
#[doc = r" Proxy"]
pub struct _CTCIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1W::CLEAR)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF5`"]
pub type CTCIF5W = CTCIF1W;
#[doc = r" Proxy"]
pub struct _CTCIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF6`"]
pub type CTCIF6W = CTCIF1W;
#[doc = r" Proxy"]
pub struct _CTCIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF7`"]
pub type CTCIF7W = CTCIF1W;
#[doc = r" Proxy"]
pub struct _CTCIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHTIF1W {
    #[doc = "Clears the HTIF flag in the ISR register"]
    CLEAR,
}
impl CHTIF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHTIF1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHTIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF2`"]
pub type CHTIF2W = CHTIF1W;
#[doc = r" Proxy"]
pub struct _CHTIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF3`"]
pub type CHTIF3W = CHTIF1W;
#[doc = r" Proxy"]
pub struct _CHTIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF4`"]
pub type CHTIF4W = CHTIF1W;
#[doc = r" Proxy"]
pub struct _CHTIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF5`"]
pub type CHTIF5W = CHTIF1W;
#[doc = r" Proxy"]
pub struct _CHTIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF6`"]
pub type CHTIF6W = CHTIF1W;
#[doc = r" Proxy"]
pub struct _CHTIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF7`"]
pub type CHTIF7W = CHTIF1W;
#[doc = r" Proxy"]
pub struct _CHTIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF1W {
    #[doc = "Clears the TEIF flag in the ISR register"]
    CLEAR,
}
impl CTEIF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTEIF1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTEIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF2`"]
pub type CTEIF2W = CTEIF1W;
#[doc = r" Proxy"]
pub struct _CTEIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF3`"]
pub type CTEIF3W = CTEIF1W;
#[doc = r" Proxy"]
pub struct _CTEIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF4`"]
pub type CTEIF4W = CTEIF1W;
#[doc = r" Proxy"]
pub struct _CTEIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF5`"]
pub type CTEIF5W = CTEIF1W;
#[doc = r" Proxy"]
pub struct _CTEIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF6`"]
pub type CTEIF6W = CTEIF1W;
#[doc = r" Proxy"]
pub struct _CTEIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1W::CLEAR)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTEIF7`"]
pub type CTEIF7W = CTEIF1W;
#[doc = r" Proxy"]
pub struct _CTEIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1W::CLEAR)
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
    #[doc = "Bit 0 - Channel 1 Global interrupt clear"]
    #[inline]
    pub fn cgif1(&mut self) -> _CGIF1W {
        _CGIF1W { w: self }
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt clear"]
    #[inline]
    pub fn cgif2(&mut self) -> _CGIF2W {
        _CGIF2W { w: self }
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt clear"]
    #[inline]
    pub fn cgif3(&mut self) -> _CGIF3W {
        _CGIF3W { w: self }
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt clear"]
    #[inline]
    pub fn cgif4(&mut self) -> _CGIF4W {
        _CGIF4W { w: self }
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt clear"]
    #[inline]
    pub fn cgif5(&mut self) -> _CGIF5W {
        _CGIF5W { w: self }
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt clear"]
    #[inline]
    pub fn cgif6(&mut self) -> _CGIF6W {
        _CGIF6W { w: self }
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt clear"]
    #[inline]
    pub fn cgif7(&mut self) -> _CGIF7W {
        _CGIF7W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete clear"]
    #[inline]
    pub fn ctcif1(&mut self) -> _CTCIF1W {
        _CTCIF1W { w: self }
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete clear"]
    #[inline]
    pub fn ctcif2(&mut self) -> _CTCIF2W {
        _CTCIF2W { w: self }
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete clear"]
    #[inline]
    pub fn ctcif3(&mut self) -> _CTCIF3W {
        _CTCIF3W { w: self }
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete clear"]
    #[inline]
    pub fn ctcif4(&mut self) -> _CTCIF4W {
        _CTCIF4W { w: self }
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete clear"]
    #[inline]
    pub fn ctcif5(&mut self) -> _CTCIF5W {
        _CTCIF5W { w: self }
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete clear"]
    #[inline]
    pub fn ctcif6(&mut self) -> _CTCIF6W {
        _CTCIF6W { w: self }
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete clear"]
    #[inline]
    pub fn ctcif7(&mut self) -> _CTCIF7W {
        _CTCIF7W { w: self }
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer clear"]
    #[inline]
    pub fn chtif1(&mut self) -> _CHTIF1W {
        _CHTIF1W { w: self }
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer clear"]
    #[inline]
    pub fn chtif2(&mut self) -> _CHTIF2W {
        _CHTIF2W { w: self }
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer clear"]
    #[inline]
    pub fn chtif3(&mut self) -> _CHTIF3W {
        _CHTIF3W { w: self }
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer clear"]
    #[inline]
    pub fn chtif4(&mut self) -> _CHTIF4W {
        _CHTIF4W { w: self }
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer clear"]
    #[inline]
    pub fn chtif5(&mut self) -> _CHTIF5W {
        _CHTIF5W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer clear"]
    #[inline]
    pub fn chtif6(&mut self) -> _CHTIF6W {
        _CHTIF6W { w: self }
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer clear"]
    #[inline]
    pub fn chtif7(&mut self) -> _CHTIF7W {
        _CHTIF7W { w: self }
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error clear"]
    #[inline]
    pub fn cteif1(&mut self) -> _CTEIF1W {
        _CTEIF1W { w: self }
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error clear"]
    #[inline]
    pub fn cteif2(&mut self) -> _CTEIF2W {
        _CTEIF2W { w: self }
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error clear"]
    #[inline]
    pub fn cteif3(&mut self) -> _CTEIF3W {
        _CTEIF3W { w: self }
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error clear"]
    #[inline]
    pub fn cteif4(&mut self) -> _CTEIF4W {
        _CTEIF4W { w: self }
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error clear"]
    #[inline]
    pub fn cteif5(&mut self) -> _CTEIF5W {
        _CTEIF5W { w: self }
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error clear"]
    #[inline]
    pub fn cteif6(&mut self) -> _CTEIF6W {
        _CTEIF6W { w: self }
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error clear"]
    #[inline]
    pub fn cteif7(&mut self) -> _CTEIF7W {
        _CTEIF7W { w: self }
    }
}
