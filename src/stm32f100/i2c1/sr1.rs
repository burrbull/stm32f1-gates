#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR1 {
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
#[doc = "Possible values of the field `SMBALERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBALERTR {
    #[doc = "No SMBALERT occured"]
    NOALERT,
    #[doc = "SMBALERT occurred"]
    ALERT,
}
impl SMBALERTR {
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
            SMBALERTR::NOALERT => false,
            SMBALERTR::ALERT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMBALERTR {
        match value {
            false => SMBALERTR::NOALERT,
            true => SMBALERTR::ALERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOALERT`"]
    #[inline]
    pub fn is_no_alert(&self) -> bool {
        *self == SMBALERTR::NOALERT
    }
    #[doc = "Checks if the value of the field is `ALERT`"]
    #[inline]
    pub fn is_alert(&self) -> bool {
        *self == SMBALERTR::ALERT
    }
}
#[doc = "Possible values of the field `TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTR {
    #[doc = "No Timeout error"]
    NOTIMEOUT,
    #[doc = "SCL remained LOW for 25 ms"]
    TIMEOUT,
}
impl TIMEOUTR {
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
            TIMEOUTR::NOTIMEOUT => false,
            TIMEOUTR::TIMEOUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMEOUTR {
        match value {
            false => TIMEOUTR::NOTIMEOUT,
            true => TIMEOUTR::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTIMEOUT`"]
    #[inline]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUTR::NOTIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUTR::TIMEOUT
    }
}
#[doc = "Possible values of the field `PECERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERRR {
    #[doc = "no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    NOERROR,
    #[doc = "PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    ERROR,
}
impl PECERRR {
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
            PECERRR::NOERROR => false,
            PECERRR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PECERRR {
        match value {
            false => PECERRR::NOERROR,
            true => PECERRR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == PECERRR::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == PECERRR::ERROR
    }
}
#[doc = "Possible values of the field `OVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRR {
    #[doc = "No overrun/underrun occured"]
    NOOVERRUN,
    #[doc = "Overrun/underrun occured"]
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
#[doc = "Possible values of the field `AF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFR {
    #[doc = "No acknowledge failure"]
    NOFAILURE,
    #[doc = "Acknowledge failure"]
    FAILURE,
}
impl AFR {
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
            AFR::NOFAILURE => false,
            AFR::FAILURE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AFR {
        match value {
            false => AFR::NOFAILURE,
            true => AFR::FAILURE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAILURE`"]
    #[inline]
    pub fn is_no_failure(&self) -> bool {
        *self == AFR::NOFAILURE
    }
    #[doc = "Checks if the value of the field is `FAILURE`"]
    #[inline]
    pub fn is_failure(&self) -> bool {
        *self == AFR::FAILURE
    }
}
#[doc = "Possible values of the field `ARLO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLOR {
    #[doc = "No Arbitration Lost detected"]
    NOLOST,
    #[doc = "Arbitration Lost detected"]
    LOST,
}
impl ARLOR {
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
            ARLOR::NOLOST => false,
            ARLOR::LOST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARLOR {
        match value {
            false => ARLOR::NOLOST,
            true => ARLOR::LOST,
        }
    }
    #[doc = "Checks if the value of the field is `NOLOST`"]
    #[inline]
    pub fn is_no_lost(&self) -> bool {
        *self == ARLOR::NOLOST
    }
    #[doc = "Checks if the value of the field is `LOST`"]
    #[inline]
    pub fn is_lost(&self) -> bool {
        *self == ARLOR::LOST
    }
}
#[doc = "Possible values of the field `BERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERRR {
    #[doc = "No misplaced Start or Stop condition"]
    NOERROR,
    #[doc = "Misplaced Start or Stop condition"]
    ERROR,
}
impl BERRR {
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
            BERRR::NOERROR => false,
            BERRR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BERRR {
        match value {
            false => BERRR::NOERROR,
            true => BERRR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == BERRR::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == BERRR::ERROR
    }
}
#[doc = "Possible values of the field `TxE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXER {
    #[doc = "Data register not empty"]
    NOTEMPTY,
    #[doc = "Data register empty"]
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
#[doc = "Possible values of the field `RxNE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNER {
    #[doc = "Data register empty"]
    EMPTY,
    #[doc = "Data register not empty"]
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
#[doc = "Possible values of the field `STOPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPFR {
    #[doc = "No Stop condition detected"]
    NOSTOP,
    #[doc = "Stop condition detected"]
    STOP,
}
impl STOPFR {
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
            STOPFR::NOSTOP => false,
            STOPFR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPFR {
        match value {
            false => STOPFR::NOSTOP,
            true => STOPFR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPFR::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STOPFR::STOP
    }
}
#[doc = r" Value of the field"]
pub struct ADD10R {
    bits: bool,
}
impl ADD10R {
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
#[doc = "Possible values of the field `BTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTFR {
    #[doc = "Data byte transfer not done"]
    NOTFINISHED,
    #[doc = "Data byte transfer successful"]
    FINISHED,
}
impl BTFR {
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
            BTFR::NOTFINISHED => false,
            BTFR::FINISHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BTFR {
        match value {
            false => BTFR::NOTFINISHED,
            true => BTFR::FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFINISHED`"]
    #[inline]
    pub fn is_not_finished(&self) -> bool {
        *self == BTFR::NOTFINISHED
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline]
    pub fn is_finished(&self) -> bool {
        *self == BTFR::FINISHED
    }
}
#[doc = "Possible values of the field `ADDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRR {
    #[doc = "Adress mismatched or not received"]
    NOTMATCH,
    #[doc = "Received slave address matched with one of the enabled slave addresses"]
    MATCH,
}
impl ADDRR {
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
            ADDRR::NOTMATCH => false,
            ADDRR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRR {
        match value {
            false => ADDRR::NOTMATCH,
            true => ADDRR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NOTMATCH`"]
    #[inline]
    pub fn is_not_match(&self) -> bool {
        *self == ADDRR::NOTMATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == ADDRR::MATCH
    }
}
#[doc = "Possible values of the field `SB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBR {
    #[doc = "No Start condition"]
    NOSTART,
    #[doc = "Start condition generated"]
    START,
}
impl SBR {
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
            SBR::NOSTART => false,
            SBR::START => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBR {
        match value {
            false => SBR::NOSTART,
            true => SBR::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTART`"]
    #[inline]
    pub fn is_no_start(&self) -> bool {
        *self == SBR::NOSTART
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == SBR::START
    }
}
#[doc = "Values that can be written to the field `SMBALERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBALERTW {
    #[doc = "No SMBALERT occured"]
    NOALERT,
    #[doc = "SMBALERT occurred"]
    ALERT,
}
impl SMBALERTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMBALERTW::NOALERT => false,
            SMBALERTW::ALERT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMBALERTW<'a> {
    w: &'a mut W,
}
impl<'a> _SMBALERTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMBALERTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SMBALERT occured"]
    #[inline]
    pub fn no_alert(self) -> &'a mut W {
        self.variant(SMBALERTW::NOALERT)
    }
    #[doc = "SMBALERT occurred"]
    #[inline]
    pub fn alert(self) -> &'a mut W {
        self.variant(SMBALERTW::ALERT)
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
#[doc = "Values that can be written to the field `TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTW {
    #[doc = "No Timeout error"]
    NOTIMEOUT,
    #[doc = "SCL remained LOW for 25 ms"]
    TIMEOUT,
}
impl TIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMEOUTW::NOTIMEOUT => false,
            TIMEOUTW::TIMEOUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Timeout error"]
    #[inline]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(TIMEOUTW::NOTIMEOUT)
    }
    #[doc = "SCL remained LOW for 25 ms"]
    #[inline]
    pub fn timeout(self) -> &'a mut W {
        self.variant(TIMEOUTW::TIMEOUT)
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
#[doc = "Values that can be written to the field `PECERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERRW {
    #[doc = "no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    NOERROR,
    #[doc = "PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    ERROR,
}
impl PECERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECERRW::NOERROR => false,
            PECERRW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PECERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(PECERRW::NOERROR)
    }
    #[doc = "PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(PECERRW::ERROR)
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
#[doc = "Values that can be written to the field `OVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRW {
    #[doc = "No overrun/underrun occured"]
    NOOVERRUN,
    #[doc = "Overrun/underrun occured"]
    OVERRUN,
}
impl OVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVRW::NOOVERRUN => false,
            OVRW::OVERRUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _OVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun/underrun occured"]
    #[inline]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVRW::NOOVERRUN)
    }
    #[doc = "Overrun/underrun occured"]
    #[inline]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVRW::OVERRUN)
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
#[doc = "Values that can be written to the field `AF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFW {
    #[doc = "No acknowledge failure"]
    NOFAILURE,
    #[doc = "Acknowledge failure"]
    FAILURE,
}
impl AFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AFW::NOFAILURE => false,
            AFW::FAILURE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFW<'a> {
    w: &'a mut W,
}
impl<'a> _AFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No acknowledge failure"]
    #[inline]
    pub fn no_failure(self) -> &'a mut W {
        self.variant(AFW::NOFAILURE)
    }
    #[doc = "Acknowledge failure"]
    #[inline]
    pub fn failure(self) -> &'a mut W {
        self.variant(AFW::FAILURE)
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
#[doc = "Values that can be written to the field `ARLO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLOW {
    #[doc = "No Arbitration Lost detected"]
    NOLOST,
    #[doc = "Arbitration Lost detected"]
    LOST,
}
impl ARLOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARLOW::NOLOST => false,
            ARLOW::LOST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARLOW<'a> {
    w: &'a mut W,
}
impl<'a> _ARLOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARLOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Arbitration Lost detected"]
    #[inline]
    pub fn no_lost(self) -> &'a mut W {
        self.variant(ARLOW::NOLOST)
    }
    #[doc = "Arbitration Lost detected"]
    #[inline]
    pub fn lost(self) -> &'a mut W {
        self.variant(ARLOW::LOST)
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
#[doc = "Values that can be written to the field `BERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERRW {
    #[doc = "No misplaced Start or Stop condition"]
    NOERROR,
    #[doc = "Misplaced Start or Stop condition"]
    ERROR,
}
impl BERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BERRW::NOERROR => false,
            BERRW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BERRW<'a> {
    w: &'a mut W,
}
impl<'a> _BERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No misplaced Start or Stop condition"]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(BERRW::NOERROR)
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(BERRW::ERROR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline]
    pub fn smbalert(&self) -> SMBALERTR {
        SMBALERTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline]
    pub fn timeout(&self) -> TIMEOUTR {
        TIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline]
    pub fn pecerr(&self) -> PECERRR {
        PECERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline]
    pub fn ovr(&self) -> OVRR {
        OVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline]
    pub fn af(&self) -> AFR {
        AFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline]
    pub fn arlo(&self) -> ARLOR {
        ARLOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline]
    pub fn berr(&self) -> BERRR {
        BERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline]
    pub fn tx_e(&self) -> TXER {
        TXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline]
    pub fn rx_ne(&self) -> RXNER {
        RXNER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline]
    pub fn stopf(&self) -> STOPFR {
        STOPFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 10-bit header sent (Master mode)"]
    #[inline]
    pub fn add10(&self) -> ADD10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADD10R { bits }
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline]
    pub fn btf(&self) -> BTFR {
        BTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline]
    pub fn addr(&self) -> ADDRR {
        ADDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline]
    pub fn sb(&self) -> SBR {
        SBR::_from({
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
    #[doc = "Bit 15 - SMBus alert"]
    #[inline]
    pub fn smbalert(&mut self) -> _SMBALERTW {
        _SMBALERTW { w: self }
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline]
    pub fn timeout(&mut self) -> _TIMEOUTW {
        _TIMEOUTW { w: self }
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline]
    pub fn pecerr(&mut self) -> _PECERRW {
        _PECERRW { w: self }
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline]
    pub fn ovr(&mut self) -> _OVRW {
        _OVRW { w: self }
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline]
    pub fn af(&mut self) -> _AFW {
        _AFW { w: self }
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline]
    pub fn arlo(&mut self) -> _ARLOW {
        _ARLOW { w: self }
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline]
    pub fn berr(&mut self) -> _BERRW {
        _BERRW { w: self }
    }
}
