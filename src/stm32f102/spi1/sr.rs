#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
#[doc = "Possible values of the field `BSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSYR {
    #[doc = "SPI not busy"]
    NOTBUSY,
    #[doc = "SPI busy"]
    BUSY,
}
impl BSYR {
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
            BSYR::NOTBUSY => false,
            BSYR::BUSY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSYR {
        match value {
            false => BSYR::NOTBUSY,
            true => BSYR::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSY`"]
    #[inline]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR::NOTBUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == BSYR::BUSY
    }
}
#[doc = "Possible values of the field `OVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRR {
    #[doc = "No overrun occurred"]
    NOOVERRUN,
    #[doc = "Overrun occurred"]
    OVERRUN,
}
impl OVRR {
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
            OVRR::NOOVERRUN => false,
            OVRR::OVERRUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVRR {
        match value {
            false => OVRR::NOOVERRUN,
            true => OVRR::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::OVERRUN
    }
}
#[doc = "Possible values of the field `MODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFR {
    #[doc = "No mode fault occurred"]
    NOFAULT,
    #[doc = "Mode fault occurred"]
    FAULT,
}
impl MODFR {
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
            MODFR::NOFAULT => false,
            MODFR::FAULT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODFR {
        match value {
            false => MODFR::NOFAULT,
            true => MODFR::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline]
    pub fn is_no_fault(&self) -> bool {
        *self == MODFR::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline]
    pub fn is_fault(&self) -> bool {
        *self == MODFR::FAULT
    }
}
#[doc = "Possible values of the field `CRCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERRR {
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    MATCH,
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    NOMATCH,
}
impl CRCERRR {
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
            CRCERRR::MATCH => false,
            CRCERRR::NOMATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCERRR {
        match value {
            false => CRCERRR::MATCH,
            true => CRCERRR::NOMATCH,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == CRCERRR::MATCH
    }
    #[doc = "Checks if the value of the field is `NOMATCH`"]
    #[inline]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERRR::NOMATCH
    }
}
#[doc = r" Value of the field"]
pub struct UDRR {
    bits: bool,
}
impl UDRR {
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
pub struct CHSIDER {
    bits: bool,
}
impl CHSIDER {
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
#[doc = "Possible values of the field `TXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXER {
    #[doc = "Tx buffer not empty"]
    NOTEMPTY,
    #[doc = "Tx buffer empty"]
    EMPTY,
}
impl TXER {
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
            TXER::NOTEMPTY => false,
            TXER::EMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXER {
        match value {
            false => TXER::NOTEMPTY,
            true => TXER::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline]
    pub fn is_not_empty(&self) -> bool {
        *self == TXER::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == TXER::EMPTY
    }
}
#[doc = "Possible values of the field `RXNE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNER {
    #[doc = "Rx buffer empty"]
    EMPTY,
    #[doc = "Rx buffer not empty"]
    NOTEMPTY,
}
impl RXNER {
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
            RXNER::EMPTY => false,
            RXNER::NOTEMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNER {
        match value {
            false => RXNER::EMPTY,
            true => RXNER::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == RXNER::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNER::NOTEMPTY
    }
}
#[doc = "Values that can be written to the field `CRCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERRW {
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    MATCH,
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    NOMATCH,
}
impl CRCERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCERRW::MATCH => false,
            CRCERRW::NOMATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(CRCERRW::MATCH)
    }
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    #[inline]
    pub fn no_match(self) -> &'a mut W {
        self.variant(CRCERRW::NOMATCH)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline]
    pub fn bsy(&self) -> BSYR {
        BSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline]
    pub fn ovr(&self) -> OVRR {
        OVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline]
    pub fn modf(&self) -> MODFR {
        MODFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline]
    pub fn crcerr(&self) -> CRCERRR {
        CRCERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Underrun flag"]
    #[inline]
    pub fn udr(&self) -> UDRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UDRR { bits }
    }
    #[doc = "Bit 2 - Channel side"]
    #[inline]
    pub fn chside(&self) -> CHSIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHSIDER { bits }
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline]
    pub fn txe(&self) -> TXER {
        TXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline]
    pub fn rxne(&self) -> RXNER {
        RXNER::_from({
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
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline]
    pub fn crcerr(&mut self) -> _CRCERRW {
        _CRCERRW { w: self }
    }
}
