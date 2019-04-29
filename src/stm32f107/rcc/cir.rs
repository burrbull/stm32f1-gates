#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CIR {
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
#[doc = "Possible values of the field `LSIRDYF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYFR {
    #[doc = "No clock ready interrupt"]
    NOTINTERRUPTED,
    #[doc = "Clock ready interrupt"]
    INTERRUPTED,
}
impl LSIRDYFR {
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
            LSIRDYFR::NOTINTERRUPTED => false,
            LSIRDYFR::INTERRUPTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSIRDYFR {
        match value {
            false => LSIRDYFR::NOTINTERRUPTED,
            true => LSIRDYFR::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYFR::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYFR::INTERRUPTED
    }
}
#[doc = "Possible values of the field `LSERDYF`"]
pub type LSERDYFR = LSIRDYFR;
#[doc = "Possible values of the field `HSIRDYF`"]
pub type HSIRDYFR = LSIRDYFR;
#[doc = "Possible values of the field `HSERDYF`"]
pub type HSERDYFR = LSIRDYFR;
#[doc = "Possible values of the field `PLLRDYF`"]
pub type PLLRDYFR = LSIRDYFR;
#[doc = "Possible values of the field `PLL2RDYF`"]
pub type PLL2RDYFR = LSIRDYFR;
#[doc = "Possible values of the field `PLL3RDYF`"]
pub type PLL3RDYFR = LSIRDYFR;
#[doc = "Possible values of the field `CSSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSFR {
    #[doc = "No clock security interrupt caused by HSE clock failure"]
    NOTINTERRUPTED,
    #[doc = "Clock security interrupt caused by HSE clock failure"]
    INTERRUPTED,
}
impl CSSFR {
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
            CSSFR::NOTINTERRUPTED => false,
            CSSFR::INTERRUPTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSSFR {
        match value {
            false => CSSFR::NOTINTERRUPTED,
            true => CSSFR::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSFR::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSFR::INTERRUPTED
    }
}
#[doc = "Possible values of the field `LSIRDYIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYIER {
    #[doc = "Interrupt disabled"]
    DISABLED,
    #[doc = "Interrupt enabled"]
    ENABLED,
}
impl LSIRDYIER {
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
            LSIRDYIER::DISABLED => false,
            LSIRDYIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSIRDYIER {
        match value {
            false => LSIRDYIER::DISABLED,
            true => LSIRDYIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIER::ENABLED
    }
}
#[doc = "Possible values of the field `LSERDYIE`"]
pub type LSERDYIER = LSIRDYIER;
#[doc = "Possible values of the field `HSIRDYIE`"]
pub type HSIRDYIER = LSIRDYIER;
#[doc = "Possible values of the field `HSERDYIE`"]
pub type HSERDYIER = LSIRDYIER;
#[doc = "Possible values of the field `PLLRDYIE`"]
pub type PLLRDYIER = LSIRDYIER;
#[doc = "Possible values of the field `PLL2RDYIE`"]
pub type PLL2RDYIER = LSIRDYIER;
#[doc = "Possible values of the field `PLL3RDYIE`"]
pub type PLL3RDYIER = LSIRDYIER;
#[doc = "Values that can be written to the field `LSIRDYIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYIEW {
    #[doc = "Interrupt disabled"]
    DISABLED,
    #[doc = "Interrupt enabled"]
    ENABLED,
}
impl LSIRDYIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSIRDYIEW::DISABLED => false,
            LSIRDYIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSIRDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LSIRDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSIRDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `LSERDYIE`"]
pub type LSERDYIEW = LSIRDYIEW;
#[doc = r" Proxy"]
pub struct _LSERDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LSERDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSERDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `HSIRDYIE`"]
pub type HSIRDYIEW = LSIRDYIEW;
#[doc = r" Proxy"]
pub struct _HSIRDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIRDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSIRDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `HSERDYIE`"]
pub type HSERDYIEW = LSIRDYIEW;
#[doc = r" Proxy"]
pub struct _HSERDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HSERDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSERDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `PLLRDYIE`"]
pub type PLLRDYIEW = LSIRDYIEW;
#[doc = r" Proxy"]
pub struct _PLLRDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLRDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLRDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `PLL2RDYIE`"]
pub type PLL2RDYIEW = LSIRDYIEW;
#[doc = r" Proxy"]
pub struct _PLL2RDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL2RDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL2RDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `PLL3RDYIE`"]
pub type PLL3RDYIEW = LSIRDYIEW;
#[doc = r" Proxy"]
pub struct _PLL3RDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL3RDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL3RDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `LSIRDYC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYCW {
    #[doc = "Clear interrupt flag"]
    CLEAR,
}
impl LSIRDYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSIRDYCW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSIRDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _LSIRDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSIRDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW::CLEAR)
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
#[doc = "Values that can be written to the field `LSERDYC`"]
pub type LSERDYCW = LSIRDYCW;
#[doc = r" Proxy"]
pub struct _LSERDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _LSERDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSERDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW::CLEAR)
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
#[doc = "Values that can be written to the field `HSIRDYC`"]
pub type HSIRDYCW = LSIRDYCW;
#[doc = r" Proxy"]
pub struct _HSIRDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIRDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSIRDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW::CLEAR)
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
#[doc = "Values that can be written to the field `HSERDYC`"]
pub type HSERDYCW = LSIRDYCW;
#[doc = r" Proxy"]
pub struct _HSERDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _HSERDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSERDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW::CLEAR)
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
#[doc = "Values that can be written to the field `PLLRDYC`"]
pub type PLLRDYCW = LSIRDYCW;
#[doc = r" Proxy"]
pub struct _PLLRDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLRDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLRDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW::CLEAR)
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
#[doc = "Values that can be written to the field `PLL2RDYC`"]
pub type PLL2RDYCW = LSIRDYCW;
#[doc = r" Proxy"]
pub struct _PLL2RDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL2RDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL2RDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW::CLEAR)
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
#[doc = "Values that can be written to the field `PLL3RDYC`"]
pub type PLL3RDYCW = LSIRDYCW;
#[doc = r" Proxy"]
pub struct _PLL3RDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL3RDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL3RDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW::CLEAR)
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
#[doc = "Values that can be written to the field `CSSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSCW {
    #[doc = "Clear CSSF flag"]
    CLEAR,
}
impl CSSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSSCW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear CSSF flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSSCW::CLEAR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - LSI Ready Interrupt flag"]
    #[inline]
    pub fn lsirdyf(&self) -> LSIRDYFR {
        LSIRDYFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - LSE Ready Interrupt flag"]
    #[inline]
    pub fn lserdyf(&self) -> LSERDYFR {
        LSERDYFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - HSI Ready Interrupt flag"]
    #[inline]
    pub fn hsirdyf(&self) -> HSIRDYFR {
        HSIRDYFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - HSE Ready Interrupt flag"]
    #[inline]
    pub fn hserdyf(&self) -> HSERDYFR {
        HSERDYFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PLL Ready Interrupt flag"]
    #[inline]
    pub fn pllrdyf(&self) -> PLLRDYFR {
        PLLRDYFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PLL2 Ready Interrupt flag"]
    #[inline]
    pub fn pll2rdyf(&self) -> PLL2RDYFR {
        PLL2RDYFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PLL3 Ready Interrupt flag"]
    #[inline]
    pub fn pll3rdyf(&self) -> PLL3RDYFR {
        PLL3RDYFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Clock Security System Interrupt flag"]
    #[inline]
    pub fn cssf(&self) -> CSSFR {
        CSSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline]
    pub fn lsirdyie(&self) -> LSIRDYIER {
        LSIRDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline]
    pub fn lserdyie(&self) -> LSERDYIER {
        LSERDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline]
    pub fn hsirdyie(&self) -> HSIRDYIER {
        HSIRDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline]
    pub fn hserdyie(&self) -> HSERDYIER {
        HSERDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline]
    pub fn pllrdyie(&self) -> PLLRDYIER {
        PLLRDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - PLL2 Ready Interrupt Enable"]
    #[inline]
    pub fn pll2rdyie(&self) -> PLL2RDYIER {
        PLL2RDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - PLL3 Ready Interrupt Enable"]
    #[inline]
    pub fn pll3rdyie(&self) -> PLL3RDYIER {
        PLL3RDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
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
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline]
    pub fn lsirdyie(&mut self) -> _LSIRDYIEW {
        _LSIRDYIEW { w: self }
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline]
    pub fn lserdyie(&mut self) -> _LSERDYIEW {
        _LSERDYIEW { w: self }
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline]
    pub fn hsirdyie(&mut self) -> _HSIRDYIEW {
        _HSIRDYIEW { w: self }
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline]
    pub fn hserdyie(&mut self) -> _HSERDYIEW {
        _HSERDYIEW { w: self }
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline]
    pub fn pllrdyie(&mut self) -> _PLLRDYIEW {
        _PLLRDYIEW { w: self }
    }
    #[doc = "Bit 13 - PLL2 Ready Interrupt Enable"]
    #[inline]
    pub fn pll2rdyie(&mut self) -> _PLL2RDYIEW {
        _PLL2RDYIEW { w: self }
    }
    #[doc = "Bit 14 - PLL3 Ready Interrupt Enable"]
    #[inline]
    pub fn pll3rdyie(&mut self) -> _PLL3RDYIEW {
        _PLL3RDYIEW { w: self }
    }
    #[doc = "Bit 16 - LSI Ready Interrupt Clear"]
    #[inline]
    pub fn lsirdyc(&mut self) -> _LSIRDYCW {
        _LSIRDYCW { w: self }
    }
    #[doc = "Bit 17 - LSE Ready Interrupt Clear"]
    #[inline]
    pub fn lserdyc(&mut self) -> _LSERDYCW {
        _LSERDYCW { w: self }
    }
    #[doc = "Bit 18 - HSI Ready Interrupt Clear"]
    #[inline]
    pub fn hsirdyc(&mut self) -> _HSIRDYCW {
        _HSIRDYCW { w: self }
    }
    #[doc = "Bit 19 - HSE Ready Interrupt Clear"]
    #[inline]
    pub fn hserdyc(&mut self) -> _HSERDYCW {
        _HSERDYCW { w: self }
    }
    #[doc = "Bit 20 - PLL Ready Interrupt Clear"]
    #[inline]
    pub fn pllrdyc(&mut self) -> _PLLRDYCW {
        _PLLRDYCW { w: self }
    }
    #[doc = "Bit 21 - PLL2 Ready Interrupt Clear"]
    #[inline]
    pub fn pll2rdyc(&mut self) -> _PLL2RDYCW {
        _PLL2RDYCW { w: self }
    }
    #[doc = "Bit 22 - PLL3 Ready Interrupt Clear"]
    #[inline]
    pub fn pll3rdyc(&mut self) -> _PLL3RDYCW {
        _PLL3RDYCW { w: self }
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline]
    pub fn cssc(&mut self) -> _CSSCW {
        _CSSCW { w: self }
    }
}
