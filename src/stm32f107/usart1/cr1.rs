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
#[doc = "Possible values of the field `UE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UER {
    #[doc = "USART prescaler and outputs disabled"]
    DISABLED,
    #[doc = "USART enabled"]
    ENABLED,
}
impl UER {
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
            UER::DISABLED => false,
            UER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UER {
        match value {
            false => UER::DISABLED,
            true => UER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UER::ENABLED
    }
}
#[doc = "Possible values of the field `M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR {
    #[doc = "8 data bits"]
    M8,
    #[doc = "9 data bits"]
    M9,
}
impl MR {
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
            MR::M8 => false,
            MR::M9 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR {
        match value {
            false => MR::M8,
            true => MR::M9,
        }
    }
    #[doc = "Checks if the value of the field is `M8`"]
    #[inline]
    pub fn is_m8(&self) -> bool {
        *self == MR::M8
    }
    #[doc = "Checks if the value of the field is `M9`"]
    #[inline]
    pub fn is_m9(&self) -> bool {
        *self == MR::M9
    }
}
#[doc = "Possible values of the field `WAKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKER {
    #[doc = "USART wakeup on idle line"]
    IDLELINE,
    #[doc = "USART wakeup on address mark"]
    ADDRESSMARK,
}
impl WAKER {
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
            WAKER::IDLELINE => false,
            WAKER::ADDRESSMARK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKER {
        match value {
            false => WAKER::IDLELINE,
            true => WAKER::ADDRESSMARK,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELINE`"]
    #[inline]
    pub fn is_idle_line(&self) -> bool {
        *self == WAKER::IDLELINE
    }
    #[doc = "Checks if the value of the field is `ADDRESSMARK`"]
    #[inline]
    pub fn is_address_mark(&self) -> bool {
        *self == WAKER::ADDRESSMARK
    }
}
#[doc = "Possible values of the field `PCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCER {
    #[doc = "Parity control disabled"]
    DISABLED,
    #[doc = "Parity control enabled"]
    ENABLED,
}
impl PCER {
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
            PCER::DISABLED => false,
            PCER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCER {
        match value {
            false => PCER::DISABLED,
            true => PCER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PCER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PCER::ENABLED
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Even parity"]
    EVEN,
    #[doc = "Odd parity"]
    ODD,
}
impl PSR {
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
            PSR::EVEN => false,
            PSR::ODD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSR {
        match value {
            false => PSR::EVEN,
            true => PSR::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == PSR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == PSR::ODD
    }
}
#[doc = "Possible values of the field `PEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIER {
    #[doc = "PE interrupt disabled"]
    DISABLED,
    #[doc = "PE interrupt enabled"]
    ENABLED,
}
impl PEIER {
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
            PEIER::DISABLED => false,
            PEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEIER {
        match value {
            false => PEIER::DISABLED,
            true => PEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PEIER::ENABLED
    }
}
#[doc = "Possible values of the field `TXEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIER {
    #[doc = "TXE interrupt disabled"]
    DISABLED,
    #[doc = "TXE interrupt enabled"]
    ENABLED,
}
impl TXEIER {
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
            TXEIER::DISABLED => false,
            TXEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEIER {
        match value {
            false => TXEIER::DISABLED,
            true => TXEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIER::ENABLED
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "TC interrupt disabled"]
    DISABLED,
    #[doc = "TC interrupt enabled"]
    ENABLED,
}
impl TCIER {
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
            TCIER::DISABLED => false,
            TCIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::DISABLED,
            true => TCIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TCIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TCIER::ENABLED
    }
}
#[doc = "Possible values of the field `RXNEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIER {
    #[doc = "RXNE interrupt disabled"]
    DISABLED,
    #[doc = "RXNE interrupt enabled"]
    ENABLED,
}
impl RXNEIER {
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
            RXNEIER::DISABLED => false,
            RXNEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNEIER {
        match value {
            false => RXNEIER::DISABLED,
            true => RXNEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIER::ENABLED
    }
}
#[doc = "Possible values of the field `IDLEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEIER {
    #[doc = "IDLE interrupt disabled"]
    DISABLED,
    #[doc = "IDLE interrupt enabled"]
    ENABLED,
}
impl IDLEIER {
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
            IDLEIER::DISABLED => false,
            IDLEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLEIER {
        match value {
            false => IDLEIER::DISABLED,
            true => IDLEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIER::ENABLED
    }
}
#[doc = "Possible values of the field `TE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TER {
    #[doc = "Transmitter disabled"]
    DISABLED,
    #[doc = "Transmitted enabled"]
    ENABLED,
}
impl TER {
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
            TER::DISABLED => false,
            TER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TER {
        match value {
            false => TER::DISABLED,
            true => TER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TER::ENABLED
    }
}
#[doc = "Possible values of the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RER {
    #[doc = "Receiver disabled"]
    DISABLED,
    #[doc = "Receiver enabled"]
    ENABLED,
}
impl RER {
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
            RER::DISABLED => false,
            RER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RER {
        match value {
            false => RER::DISABLED,
            true => RER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RER::ENABLED
    }
}
#[doc = "Possible values of the field `RWU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUR {
    #[doc = "Receiver in active mode"]
    ACTIVE,
    #[doc = "Receiver in mute mode"]
    MUTE,
}
impl RWUR {
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
            RWUR::ACTIVE => false,
            RWUR::MUTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWUR {
        match value {
            false => RWUR::ACTIVE,
            true => RWUR::MUTE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == RWUR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `MUTE`"]
    #[inline]
    pub fn is_mute(&self) -> bool {
        *self == RWUR::MUTE
    }
}
#[doc = "Possible values of the field `SBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKR {
    #[doc = "No break character is transmitted"]
    NOBREAK,
    #[doc = "Break character transmitted"]
    BREAK,
}
impl SBKR {
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
            SBKR::NOBREAK => false,
            SBKR::BREAK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBKR {
        match value {
            false => SBKR::NOBREAK,
            true => SBKR::BREAK,
        }
    }
    #[doc = "Checks if the value of the field is `NOBREAK`"]
    #[inline]
    pub fn is_no_break(&self) -> bool {
        *self == SBKR::NOBREAK
    }
    #[doc = "Checks if the value of the field is `BREAK`"]
    #[inline]
    pub fn is_break_(&self) -> bool {
        *self == SBKR::BREAK
    }
}
#[doc = "Values that can be written to the field `UE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UEW {
    #[doc = "USART prescaler and outputs disabled"]
    DISABLED,
    #[doc = "USART enabled"]
    ENABLED,
}
impl UEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UEW::DISABLED => false,
            UEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UEW<'a> {
    w: &'a mut W,
}
impl<'a> _UEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USART prescaler and outputs disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UEW::DISABLED)
    }
    #[doc = "USART enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UEW::ENABLED)
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
#[doc = "Values that can be written to the field `M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MW {
    #[doc = "8 data bits"]
    M8,
    #[doc = "9 data bits"]
    M9,
}
impl MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MW::M8 => false,
            MW::M9 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MW<'a> {
    w: &'a mut W,
}
impl<'a> _MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8 data bits"]
    #[inline]
    pub fn m8(self) -> &'a mut W {
        self.variant(MW::M8)
    }
    #[doc = "9 data bits"]
    #[inline]
    pub fn m9(self) -> &'a mut W {
        self.variant(MW::M9)
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
#[doc = "Values that can be written to the field `WAKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEW {
    #[doc = "USART wakeup on idle line"]
    IDLELINE,
    #[doc = "USART wakeup on address mark"]
    ADDRESSMARK,
}
impl WAKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEW::IDLELINE => false,
            WAKEW::ADDRESSMARK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USART wakeup on idle line"]
    #[inline]
    pub fn idle_line(self) -> &'a mut W {
        self.variant(WAKEW::IDLELINE)
    }
    #[doc = "USART wakeup on address mark"]
    #[inline]
    pub fn address_mark(self) -> &'a mut W {
        self.variant(WAKEW::ADDRESSMARK)
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
#[doc = "Values that can be written to the field `PCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCEW {
    #[doc = "Parity control disabled"]
    DISABLED,
    #[doc = "Parity control enabled"]
    ENABLED,
}
impl PCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCEW::DISABLED => false,
            PCEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCEW<'a> {
    w: &'a mut W,
}
impl<'a> _PCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Parity control disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCEW::DISABLED)
    }
    #[doc = "Parity control enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCEW::ENABLED)
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
#[doc = "Values that can be written to the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSW {
    #[doc = "Even parity"]
    EVEN,
    #[doc = "Odd parity"]
    ODD,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSW::EVEN => false,
            PSW::ODD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(PSW::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(PSW::ODD)
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
#[doc = "Values that can be written to the field `PEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIEW {
    #[doc = "PE interrupt disabled"]
    DISABLED,
    #[doc = "PE interrupt enabled"]
    ENABLED,
}
impl PEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEIEW::DISABLED => false,
            PEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PE interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIEW::DISABLED)
    }
    #[doc = "PE interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TXEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIEW {
    #[doc = "TXE interrupt disabled"]
    DISABLED,
    #[doc = "TXE interrupt enabled"]
    ENABLED,
}
impl TXEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEIEW::DISABLED => false,
            TXEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXE interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIEW::DISABLED)
    }
    #[doc = "TXE interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIEW {
    #[doc = "TC interrupt disabled"]
    DISABLED,
    #[doc = "TC interrupt enabled"]
    ENABLED,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::DISABLED => false,
            TCIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TC interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIEW::DISABLED)
    }
    #[doc = "TC interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIEW::ENABLED)
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
#[doc = "Values that can be written to the field `RXNEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIEW {
    #[doc = "RXNE interrupt disabled"]
    DISABLED,
    #[doc = "RXNE interrupt enabled"]
    ENABLED,
}
impl RXNEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXNEIEW::DISABLED => false,
            RXNEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXNEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXNEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXNE interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIEW::DISABLED)
    }
    #[doc = "RXNE interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXNEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `IDLEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEIEW {
    #[doc = "IDLE interrupt disabled"]
    DISABLED,
    #[doc = "IDLE interrupt enabled"]
    ENABLED,
}
impl IDLEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLEIEW::DISABLED => false,
            IDLEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IDLE interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIEW::DISABLED)
    }
    #[doc = "IDLE interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDLEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEW {
    #[doc = "Transmitter disabled"]
    DISABLED,
    #[doc = "Transmitted enabled"]
    ENABLED,
}
impl TEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEW::DISABLED => false,
            TEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitter disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEW::DISABLED)
    }
    #[doc = "Transmitted enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEW::ENABLED)
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
#[doc = "Values that can be written to the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REW {
    #[doc = "Receiver disabled"]
    DISABLED,
    #[doc = "Receiver enabled"]
    ENABLED,
}
impl REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REW::DISABLED => false,
            REW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REW::DISABLED)
    }
    #[doc = "Receiver enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REW::ENABLED)
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
#[doc = "Values that can be written to the field `RWU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUW {
    #[doc = "Receiver in active mode"]
    ACTIVE,
    #[doc = "Receiver in mute mode"]
    MUTE,
}
impl RWUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWUW::ACTIVE => false,
            RWUW::MUTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWUW<'a> {
    w: &'a mut W,
}
impl<'a> _RWUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver in active mode"]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(RWUW::ACTIVE)
    }
    #[doc = "Receiver in mute mode"]
    #[inline]
    pub fn mute(self) -> &'a mut W {
        self.variant(RWUW::MUTE)
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
#[doc = "Values that can be written to the field `SBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKW {
    #[doc = "No break character is transmitted"]
    NOBREAK,
    #[doc = "Break character transmitted"]
    BREAK,
}
impl SBKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBKW::NOBREAK => false,
            SBKW::BREAK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBKW<'a> {
    w: &'a mut W,
}
impl<'a> _SBKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No break character is transmitted"]
    #[inline]
    pub fn no_break(self) -> &'a mut W {
        self.variant(SBKW::NOBREAK)
    }
    #[doc = "Break character transmitted"]
    #[inline]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKW::BREAK)
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
    #[doc = "Bit 13 - USART enable"]
    #[inline]
    pub fn ue(&self) -> UER {
        UER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Word length"]
    #[inline]
    pub fn m(&self) -> MR {
        MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Wakeup method"]
    #[inline]
    pub fn wake(&self) -> WAKER {
        WAKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline]
    pub fn pce(&self) -> PCER {
        PCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline]
    pub fn peie(&self) -> PEIER {
        PEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - TXE interrupt enable"]
    #[inline]
    pub fn txeie(&self) -> TXEIER {
        TXEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline]
    pub fn rxneie(&self) -> RXNEIER {
        RXNEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline]
    pub fn idleie(&self) -> IDLEIER {
        IDLEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline]
    pub fn te(&self) -> TER {
        TER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline]
    pub fn re(&self) -> RER {
        RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline]
    pub fn rwu(&self) -> RWUR {
        RWUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Send break"]
    #[inline]
    pub fn sbk(&self) -> SBKR {
        SBKR::_from({
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
    #[doc = "Bit 13 - USART enable"]
    #[inline]
    pub fn ue(&mut self) -> _UEW {
        _UEW { w: self }
    }
    #[doc = "Bit 12 - Word length"]
    #[inline]
    pub fn m(&mut self) -> _MW {
        _MW { w: self }
    }
    #[doc = "Bit 11 - Wakeup method"]
    #[inline]
    pub fn wake(&mut self) -> _WAKEW {
        _WAKEW { w: self }
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline]
    pub fn pce(&mut self) -> _PCEW {
        _PCEW { w: self }
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline]
    pub fn peie(&mut self) -> _PEIEW {
        _PEIEW { w: self }
    }
    #[doc = "Bit 7 - TXE interrupt enable"]
    #[inline]
    pub fn txeie(&mut self) -> _TXEIEW {
        _TXEIEW { w: self }
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline]
    pub fn rxneie(&mut self) -> _RXNEIEW {
        _RXNEIEW { w: self }
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline]
    pub fn idleie(&mut self) -> _IDLEIEW {
        _IDLEIEW { w: self }
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline]
    pub fn rwu(&mut self) -> _RWUW {
        _RWUW { w: self }
    }
    #[doc = "Bit 0 - Send break"]
    #[inline]
    pub fn sbk(&mut self) -> _SBKW {
        _SBKW { w: self }
    }
}
