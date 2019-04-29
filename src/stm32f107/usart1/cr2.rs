#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
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
#[doc = "Possible values of the field `LINEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINENR {
    #[doc = "LIN mode disabled"]
    DISABLED,
    #[doc = "LIN mode enabled"]
    ENABLED,
}
impl LINENR {
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
            LINENR::DISABLED => false,
            LINENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINENR {
        match value {
            false => LINENR::DISABLED,
            true => LINENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LINENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LINENR::ENABLED
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "1 stop bit"]
    STOP1,
    #[doc = "0.5 stop bits"]
    STOP0P5,
    #[doc = "2 stop bits"]
    STOP2,
    #[doc = "1.5 stop bits"]
    STOP1P5,
}
impl STOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOPR::STOP1 => 0,
            STOPR::STOP0P5 => 1,
            STOPR::STOP2 => 2,
            STOPR::STOP1P5 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOPR {
        match value {
            0 => STOPR::STOP1,
            1 => STOPR::STOP0P5,
            2 => STOPR::STOP2,
            3 => STOPR::STOP1P5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline]
    pub fn is_stop1(&self) -> bool {
        *self == STOPR::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP0P5`"]
    #[inline]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOPR::STOP0P5
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline]
    pub fn is_stop2(&self) -> bool {
        *self == STOPR::STOP2
    }
    #[doc = "Checks if the value of the field is `STOP1P5`"]
    #[inline]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOPR::STOP1P5
    }
}
#[doc = "Possible values of the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKENR {
    #[doc = "CK pin disabled"]
    DISABLED,
    #[doc = "CK pin enabled"]
    ENABLED,
}
impl CLKENR {
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
            CLKENR::DISABLED => false,
            CLKENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKENR {
        match value {
            false => CLKENR::DISABLED,
            true => CLKENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CLKENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CLKENR::ENABLED
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "Steady low value on CK pin outside transmission window"]
    LOW,
    #[doc = "Steady high value on CK pin outside transmission window"]
    HIGH,
}
impl CPOLR {
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
            CPOLR::LOW => false,
            CPOLR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::LOW,
            true => CPOLR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CPOLR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CPOLR::HIGH
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "The first clock transition is the first data capture edge"]
    FIRST,
    #[doc = "The second clock transition is the first data capture edge"]
    SECOND,
}
impl CPHAR {
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
            CPHAR::FIRST => false,
            CPHAR::SECOND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::FIRST,
            true => CPHAR::SECOND,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST`"]
    #[inline]
    pub fn is_first(&self) -> bool {
        *self == CPHAR::FIRST
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline]
    pub fn is_second(&self) -> bool {
        *self == CPHAR::SECOND
    }
}
#[doc = r" Value of the field"]
pub struct LBCLR {
    bits: bool,
}
impl LBCLR {
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
#[doc = "Possible values of the field `LBDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDIER {
    #[doc = "LIN break detection interrupt disabled"]
    DISABLED,
    #[doc = "LIN break detection interrupt enabled"]
    ENABLED,
}
impl LBDIER {
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
            LBDIER::DISABLED => false,
            LBDIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBDIER {
        match value {
            false => LBDIER::DISABLED,
            true => LBDIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIER::ENABLED
    }
}
#[doc = "Possible values of the field `LBDL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDLR {
    #[doc = "10-bit break detection"]
    LBDL10,
    #[doc = "11-bit break detection"]
    LBDL11,
}
impl LBDLR {
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
            LBDLR::LBDL10 => false,
            LBDLR::LBDL11 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBDLR {
        match value {
            false => LBDLR::LBDL10,
            true => LBDLR::LBDL11,
        }
    }
    #[doc = "Checks if the value of the field is `LBDL10`"]
    #[inline]
    pub fn is_lbdl10(&self) -> bool {
        *self == LBDLR::LBDL10
    }
    #[doc = "Checks if the value of the field is `LBDL11`"]
    #[inline]
    pub fn is_lbdl11(&self) -> bool {
        *self == LBDLR::LBDL11
    }
}
#[doc = r" Value of the field"]
pub struct ADDR {
    bits: u8,
}
impl ADDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `LINEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINENW {
    #[doc = "LIN mode disabled"]
    DISABLED,
    #[doc = "LIN mode enabled"]
    ENABLED,
}
impl LINENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINENW::DISABLED => false,
            LINENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINENW<'a> {
    w: &'a mut W,
}
impl<'a> _LINENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINENW::DISABLED)
    }
    #[doc = "LIN mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINENW::ENABLED)
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
#[doc = "Values that can be written to the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPW {
    #[doc = "1 stop bit"]
    STOP1,
    #[doc = "0.5 stop bits"]
    STOP0P5,
    #[doc = "2 stop bits"]
    STOP2,
    #[doc = "1.5 stop bits"]
    STOP1P5,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOPW::STOP1 => 0,
            STOPW::STOP0P5 => 1,
            STOPW::STOP2 => 2,
            STOPW::STOP1P5 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 stop bit"]
    #[inline]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STOPW::STOP1)
    }
    #[doc = "0.5 stop bits"]
    #[inline]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STOPW::STOP0P5)
    }
    #[doc = "2 stop bits"]
    #[inline]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STOPW::STOP2)
    }
    #[doc = "1.5 stop bits"]
    #[inline]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STOPW::STOP1P5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKENW {
    #[doc = "CK pin disabled"]
    DISABLED,
    #[doc = "CK pin enabled"]
    ENABLED,
}
impl CLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKENW::DISABLED => false,
            CLKENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CK pin disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKENW::DISABLED)
    }
    #[doc = "CK pin enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKENW::ENABLED)
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
#[doc = "Values that can be written to the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLW {
    #[doc = "Steady low value on CK pin outside transmission window"]
    LOW,
    #[doc = "Steady high value on CK pin outside transmission window"]
    HIGH,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::LOW => false,
            CPOLW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Steady low value on CK pin outside transmission window"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOLW::LOW)
    }
    #[doc = "Steady high value on CK pin outside transmission window"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOLW::HIGH)
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
#[doc = "Values that can be written to the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAW {
    #[doc = "The first clock transition is the first data capture edge"]
    FIRST,
    #[doc = "The second clock transition is the first data capture edge"]
    SECOND,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::FIRST => false,
            CPHAW::SECOND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline]
    pub fn first(self) -> &'a mut W {
        self.variant(CPHAW::FIRST)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline]
    pub fn second(self) -> &'a mut W {
        self.variant(CPHAW::SECOND)
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
#[doc = r" Proxy"]
pub struct _LBCLW<'a> {
    w: &'a mut W,
}
impl<'a> _LBCLW<'a> {
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
#[doc = "Values that can be written to the field `LBDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDIEW {
    #[doc = "LIN break detection interrupt disabled"]
    DISABLED,
    #[doc = "LIN break detection interrupt enabled"]
    ENABLED,
}
impl LBDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBDIEW::DISABLED => false,
            LBDIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LIN break detection interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBDIEW::DISABLED)
    }
    #[doc = "LIN break detection interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBDIEW::ENABLED)
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
#[doc = "Values that can be written to the field `LBDL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDLW {
    #[doc = "10-bit break detection"]
    LBDL10,
    #[doc = "11-bit break detection"]
    LBDL11,
}
impl LBDLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBDLW::LBDL10 => false,
            LBDLW::LBDL11 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBDLW<'a> {
    w: &'a mut W,
}
impl<'a> _LBDLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBDLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "10-bit break detection"]
    #[inline]
    pub fn lbdl10(self) -> &'a mut W {
        self.variant(LBDLW::LBDL10)
    }
    #[doc = "11-bit break detection"]
    #[inline]
    pub fn lbdl11(self) -> &'a mut W {
        self.variant(LBDLW::LBDL11)
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
#[doc = r" Proxy"]
pub struct _ADDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline]
    pub fn linen(&self) -> LINENR {
        LINENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline]
    pub fn clken(&self) -> CLKENR {
        CLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline]
    pub fn lbcl(&self) -> LBCLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBCLR { bits }
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline]
    pub fn lbdie(&self) -> LBDIER {
        LBDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline]
    pub fn lbdl(&self) -> LBDLR {
        LBDLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline]
    pub fn add(&self) -> ADDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline]
    pub fn linen(&mut self) -> _LINENW {
        _LINENW { w: self }
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline]
    pub fn lbcl(&mut self) -> _LBCLW {
        _LBCLW { w: self }
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline]
    pub fn lbdie(&mut self) -> _LBDIEW {
        _LBDIEW { w: self }
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline]
    pub fn lbdl(&mut self) -> _LBDLW {
        _LBDLW { w: self }
    }
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline]
    pub fn add(&mut self) -> _ADDW {
        _ADDW { w: self }
    }
}
