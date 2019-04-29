#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `SWRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTR {
    #[doc = "I2C peripheral not under reset"]
    NOTRESET,
    #[doc = "I2C peripheral under reset"]
    RESET,
}
impl SWRSTR {
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
            SWRSTR::NOTRESET => false,
            SWRSTR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTR {
        match value {
            false => SWRSTR::NOTRESET,
            true => SWRSTR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRESET`"]
    #[inline]
    pub fn is_not_reset(&self) -> bool {
        *self == SWRSTR::NOTRESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTR::RESET
    }
}
#[doc = "Possible values of the field `ALERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTR {
    #[doc = "SMBA pin released high"]
    RELEASE,
    #[doc = "SMBA pin driven low"]
    DRIVE,
}
impl ALERTR {
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
            ALERTR::RELEASE => false,
            ALERTR::DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALERTR {
        match value {
            false => ALERTR::RELEASE,
            true => ALERTR::DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASE`"]
    #[inline]
    pub fn is_release(&self) -> bool {
        *self == ALERTR::RELEASE
    }
    #[doc = "Checks if the value of the field is `DRIVE`"]
    #[inline]
    pub fn is_drive(&self) -> bool {
        *self == ALERTR::DRIVE
    }
}
#[doc = "Possible values of the field `PEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECR {
    #[doc = "No PEC transfer"]
    DISABLED,
    #[doc = "PEC transfer"]
    ENABLED,
}
impl PECR {
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
            PECR::DISABLED => false,
            PECR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PECR {
        match value {
            false => PECR::DISABLED,
            true => PECR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PECR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PECR::ENABLED
    }
}
#[doc = "Possible values of the field `POS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR {
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    CURRENT,
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    NEXT,
}
impl POSR {
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
            POSR::CURRENT => false,
            POSR::NEXT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POSR {
        match value {
            false => POSR::CURRENT,
            true => POSR::NEXT,
        }
    }
    #[doc = "Checks if the value of the field is `CURRENT`"]
    #[inline]
    pub fn is_current(&self) -> bool {
        *self == POSR::CURRENT
    }
    #[doc = "Checks if the value of the field is `NEXT`"]
    #[inline]
    pub fn is_next(&self) -> bool {
        *self == POSR::NEXT
    }
}
#[doc = "Possible values of the field `ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKR {
    #[doc = "No acknowledge returned"]
    NAK,
    #[doc = "Acknowledge returned after a byte is received"]
    ACK,
}
impl ACKR {
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
            ACKR::NAK => false,
            ACKR::ACK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKR {
        match value {
            false => ACKR::NAK,
            true => ACKR::ACK,
        }
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline]
    pub fn is_nak(&self) -> bool {
        *self == ACKR::NAK
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline]
    pub fn is_ack(&self) -> bool {
        *self == ACKR::ACK
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "No Stop generation"]
    NOSTOP,
    #[doc = "In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    STOP,
}
impl STOPR {
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
            STOPR::NOSTOP => false,
            STOPR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPR {
        match value {
            false => STOPR::NOSTOP,
            true => STOPR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPR::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STOPR::STOP
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "No Start generation"]
    NOSTART,
    #[doc = "In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    START,
}
impl STARTR {
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
            STARTR::NOSTART => false,
            STARTR::START => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTR {
        match value {
            false => STARTR::NOSTART,
            true => STARTR::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTART`"]
    #[inline]
    pub fn is_no_start(&self) -> bool {
        *self == STARTR::NOSTART
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == STARTR::START
    }
}
#[doc = "Possible values of the field `NOSTRETCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTRETCHR {
    #[doc = "Clock stretching enabled"]
    ENABLED,
    #[doc = "Clock stretching disabled"]
    DISABLED,
}
impl NOSTRETCHR {
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
            NOSTRETCHR::ENABLED => false,
            NOSTRETCHR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOSTRETCHR {
        match value {
            false => NOSTRETCHR::ENABLED,
            true => NOSTRETCHR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCHR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCHR::DISABLED
    }
}
#[doc = "Possible values of the field `ENGC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENGCR {
    #[doc = "General call disabled"]
    DISABLED,
    #[doc = "General call enabled"]
    ENABLED,
}
impl ENGCR {
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
            ENGCR::DISABLED => false,
            ENGCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENGCR {
        match value {
            false => ENGCR::DISABLED,
            true => ENGCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENGCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENGCR::ENABLED
    }
}
#[doc = "Possible values of the field `ENPEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENPECR {
    #[doc = "PEC calculation disabled"]
    DISABLED,
    #[doc = "PEC calculation enabled"]
    ENABLED,
}
impl ENPECR {
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
            ENPECR::DISABLED => false,
            ENPECR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENPECR {
        match value {
            false => ENPECR::DISABLED,
            true => ENPECR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENPECR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENPECR::ENABLED
    }
}
#[doc = "Possible values of the field `ENARP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENARPR {
    #[doc = "ARP disabled"]
    DISABLED,
    #[doc = "ARP enabled"]
    ENABLED,
}
impl ENARPR {
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
            ENARPR::DISABLED => false,
            ENARPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENARPR {
        match value {
            false => ENARPR::DISABLED,
            true => ENARPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENARPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENARPR::ENABLED
    }
}
#[doc = "Possible values of the field `SMBTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBTYPER {
    #[doc = "SMBus Device"]
    DEVICE,
    #[doc = "SMBus Host"]
    HOST,
}
impl SMBTYPER {
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
            SMBTYPER::DEVICE => false,
            SMBTYPER::HOST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMBTYPER {
        match value {
            false => SMBTYPER::DEVICE,
            true => SMBTYPER::HOST,
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline]
    pub fn is_device(&self) -> bool {
        *self == SMBTYPER::DEVICE
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline]
    pub fn is_host(&self) -> bool {
        *self == SMBTYPER::HOST
    }
}
#[doc = "Possible values of the field `SMBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBUSR {
    #[doc = "I2C Mode"]
    I2C,
    #[doc = "SMBus"]
    SMBUS,
}
impl SMBUSR {
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
            SMBUSR::I2C => false,
            SMBUSR::SMBUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMBUSR {
        match value {
            false => SMBUSR::I2C,
            true => SMBUSR::SMBUS,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline]
    pub fn is_i2c(&self) -> bool {
        *self == SMBUSR::I2C
    }
    #[doc = "Checks if the value of the field is `SMBUS`"]
    #[inline]
    pub fn is_smbus(&self) -> bool {
        *self == SMBUSR::SMBUS
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Peripheral disabled"]
    DISABLED,
    #[doc = "Peripheral enabled"]
    ENABLED,
}
impl PER {
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
            PER::DISABLED => false,
            PER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::DISABLED,
            true => PER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PER::ENABLED
    }
}
#[doc = "Values that can be written to the field `SWRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTW {
    #[doc = "I2C peripheral not under reset"]
    NOTRESET,
    #[doc = "I2C peripheral under reset"]
    RESET,
}
impl SWRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTW::NOTRESET => false,
            SWRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C peripheral not under reset"]
    #[inline]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(SWRSTW::NOTRESET)
    }
    #[doc = "I2C peripheral under reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTW::RESET)
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
#[doc = "Values that can be written to the field `ALERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTW {
    #[doc = "SMBA pin released high"]
    RELEASE,
    #[doc = "SMBA pin driven low"]
    DRIVE,
}
impl ALERTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALERTW::RELEASE => false,
            ALERTW::DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALERTW<'a> {
    w: &'a mut W,
}
impl<'a> _ALERTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALERTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SMBA pin released high"]
    #[inline]
    pub fn release(self) -> &'a mut W {
        self.variant(ALERTW::RELEASE)
    }
    #[doc = "SMBA pin driven low"]
    #[inline]
    pub fn drive(self) -> &'a mut W {
        self.variant(ALERTW::DRIVE)
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
#[doc = "Values that can be written to the field `PEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECW {
    #[doc = "No PEC transfer"]
    DISABLED,
    #[doc = "PEC transfer"]
    ENABLED,
}
impl PECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECW::DISABLED => false,
            PECW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECW<'a> {
    w: &'a mut W,
}
impl<'a> _PECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No PEC transfer"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PECW::DISABLED)
    }
    #[doc = "PEC transfer"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PECW::ENABLED)
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
#[doc = "Values that can be written to the field `POS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSW {
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    CURRENT,
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    NEXT,
}
impl POSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POSW::CURRENT => false,
            POSW::NEXT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POSW<'a> {
    w: &'a mut W,
}
impl<'a> _POSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline]
    pub fn current(self) -> &'a mut W {
        self.variant(POSW::CURRENT)
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline]
    pub fn next(self) -> &'a mut W {
        self.variant(POSW::NEXT)
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
#[doc = "Values that can be written to the field `ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKW {
    #[doc = "No acknowledge returned"]
    NAK,
    #[doc = "Acknowledge returned after a byte is received"]
    ACK,
}
impl ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACKW::NAK => false,
            ACKW::ACK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No acknowledge returned"]
    #[inline]
    pub fn nak(self) -> &'a mut W {
        self.variant(ACKW::NAK)
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline]
    pub fn ack(self) -> &'a mut W {
        self.variant(ACKW::ACK)
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
#[doc = "Values that can be written to the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPW {
    #[doc = "No Stop generation"]
    NOSTOP,
    #[doc = "In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    STOP,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPW::NOSTOP => false,
            STOPW::STOP => true,
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
            self.bit(variant._bits())
        }
    }
    #[doc = "No Stop generation"]
    #[inline]
    pub fn no_stop(self) -> &'a mut W {
        self.variant(STOPW::NOSTOP)
    }
    #[doc = "In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPW::STOP)
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
#[doc = "Values that can be written to the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTW {
    #[doc = "No Start generation"]
    NOSTART,
    #[doc = "In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    START,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::NOSTART => false,
            STARTW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Start generation"]
    #[inline]
    pub fn no_start(self) -> &'a mut W {
        self.variant(STARTW::NOSTART)
    }
    #[doc = "In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW::START)
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
#[doc = "Values that can be written to the field `NOSTRETCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTRETCHW {
    #[doc = "Clock stretching enabled"]
    ENABLED,
    #[doc = "Clock stretching disabled"]
    DISABLED,
}
impl NOSTRETCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOSTRETCHW::ENABLED => false,
            NOSTRETCHW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOSTRETCHW<'a> {
    w: &'a mut W,
}
impl<'a> _NOSTRETCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOSTRETCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOSTRETCHW::ENABLED)
    }
    #[doc = "Clock stretching disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOSTRETCHW::DISABLED)
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
#[doc = "Values that can be written to the field `ENGC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENGCW {
    #[doc = "General call disabled"]
    DISABLED,
    #[doc = "General call enabled"]
    ENABLED,
}
impl ENGCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENGCW::DISABLED => false,
            ENGCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENGCW<'a> {
    w: &'a mut W,
}
impl<'a> _ENGCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENGCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "General call disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENGCW::DISABLED)
    }
    #[doc = "General call enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENGCW::ENABLED)
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
#[doc = "Values that can be written to the field `ENPEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENPECW {
    #[doc = "PEC calculation disabled"]
    DISABLED,
    #[doc = "PEC calculation enabled"]
    ENABLED,
}
impl ENPECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENPECW::DISABLED => false,
            ENPECW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENPECW<'a> {
    w: &'a mut W,
}
impl<'a> _ENPECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENPECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENPECW::DISABLED)
    }
    #[doc = "PEC calculation enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENPECW::ENABLED)
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
#[doc = "Values that can be written to the field `ENARP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENARPW {
    #[doc = "ARP disabled"]
    DISABLED,
    #[doc = "ARP enabled"]
    ENABLED,
}
impl ENARPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENARPW::DISABLED => false,
            ENARPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENARPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENARPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENARPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ARP disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENARPW::DISABLED)
    }
    #[doc = "ARP enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENARPW::ENABLED)
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
#[doc = "Values that can be written to the field `SMBTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBTYPEW {
    #[doc = "SMBus Device"]
    DEVICE,
    #[doc = "SMBus Host"]
    HOST,
}
impl SMBTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMBTYPEW::DEVICE => false,
            SMBTYPEW::HOST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMBTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _SMBTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMBTYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SMBus Device"]
    #[inline]
    pub fn device(self) -> &'a mut W {
        self.variant(SMBTYPEW::DEVICE)
    }
    #[doc = "SMBus Host"]
    #[inline]
    pub fn host(self) -> &'a mut W {
        self.variant(SMBTYPEW::HOST)
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
#[doc = "Values that can be written to the field `SMBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBUSW {
    #[doc = "I2C Mode"]
    I2C,
    #[doc = "SMBus"]
    SMBUS,
}
impl SMBUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMBUSW::I2C => false,
            SMBUSW::SMBUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SMBUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMBUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C Mode"]
    #[inline]
    pub fn i2c(self) -> &'a mut W {
        self.variant(SMBUSW::I2C)
    }
    #[doc = "SMBus"]
    #[inline]
    pub fn smbus(self) -> &'a mut W {
        self.variant(SMBUSW::SMBUS)
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
#[doc = "Values that can be written to the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEW {
    #[doc = "Peripheral disabled"]
    DISABLED,
    #[doc = "Peripheral enabled"]
    ENABLED,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::DISABLED => false,
            PEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEW::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEW::ENABLED)
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
    #[doc = "Bit 15 - Software reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        SWRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline]
    pub fn alert(&self) -> ALERTR {
        ALERTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline]
    pub fn pec(&self) -> PECR {
        PECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline]
    pub fn pos(&self) -> POSR {
        POSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline]
    pub fn ack(&self) -> ACKR {
        ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline]
    pub fn nostretch(&self) -> NOSTRETCHR {
        NOSTRETCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline]
    pub fn engc(&self) -> ENGCR {
        ENGCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline]
    pub fn enpec(&self) -> ENPECR {
        ENPECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline]
    pub fn enarp(&self) -> ENARPR {
        ENARPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline]
    pub fn smbtype(&self) -> SMBTYPER {
        SMBTYPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline]
    pub fn smbus(&self) -> SMBUSR {
        SMBUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
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
    #[doc = "Bit 15 - Software reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline]
    pub fn alert(&mut self) -> _ALERTW {
        _ALERTW { w: self }
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline]
    pub fn pec(&mut self) -> _PECW {
        _PECW { w: self }
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline]
    pub fn pos(&mut self) -> _POSW {
        _POSW { w: self }
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline]
    pub fn ack(&mut self) -> _ACKW {
        _ACKW { w: self }
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline]
    pub fn nostretch(&mut self) -> _NOSTRETCHW {
        _NOSTRETCHW { w: self }
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline]
    pub fn engc(&mut self) -> _ENGCW {
        _ENGCW { w: self }
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline]
    pub fn enpec(&mut self) -> _ENPECW {
        _ENPECW { w: self }
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline]
    pub fn enarp(&mut self) -> _ENARPW {
        _ENARPW { w: self }
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline]
    pub fn smbtype(&mut self) -> _SMBTYPEW {
        _SMBTYPEW { w: self }
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline]
    pub fn smbus(&mut self) -> _SMBUSW {
        _SMBUSW { w: self }
    }
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
}
