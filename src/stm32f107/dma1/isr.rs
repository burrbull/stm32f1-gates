#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ISR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `GIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF1R {
    #[doc = "No transfer error, half event, complete event"]
    NOEVENT,
    #[doc = "A transfer error, half event or complete event has occured"]
    EVENT,
}
impl GIF1R {
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
            GIF1R::NOEVENT => false,
            GIF1R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GIF1R {
        match value {
            false => GIF1R::NOEVENT,
            true => GIF1R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline]
    pub fn is_no_event(&self) -> bool {
        *self == GIF1R::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == GIF1R::EVENT
    }
}
#[doc = "Possible values of the field `TCIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF1R {
    #[doc = "No transfer complete event"]
    NOTCOMPLETE,
    #[doc = "A transfer complete event has occured"]
    COMPLETE,
}
impl TCIF1R {
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
            TCIF1R::NOTCOMPLETE => false,
            TCIF1R::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIF1R {
        match value {
            false => TCIF1R::NOTCOMPLETE,
            true => TCIF1R::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF1R::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == TCIF1R::COMPLETE
    }
}
#[doc = "Possible values of the field `HTIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF1R {
    #[doc = "No half transfer event"]
    NOTHALT,
    #[doc = "A half transfer event has occured"]
    HALF,
}
impl HTIF1R {
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
            HTIF1R::NOTHALT => false,
            HTIF1R::HALF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTIF1R {
        match value {
            false => HTIF1R::NOTHALT,
            true => HTIF1R::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALT`"]
    #[inline]
    pub fn is_not_halt(&self) -> bool {
        *self == HTIF1R::NOTHALT
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == HTIF1R::HALF
    }
}
#[doc = "Possible values of the field `TEIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF1R {
    #[doc = "No transfer error"]
    NOERROR,
    #[doc = "A transfer error has occured"]
    ERROR,
}
impl TEIF1R {
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
            TEIF1R::NOERROR => false,
            TEIF1R::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIF1R {
        match value {
            false => TEIF1R::NOERROR,
            true => TEIF1R::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF1R::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == TEIF1R::ERROR
    }
}
#[doc = "Possible values of the field `GIF2`"]
pub type GIF2R = GIF1R;
#[doc = "Possible values of the field `TCIF2`"]
pub type TCIF2R = TCIF1R;
#[doc = "Possible values of the field `HTIF2`"]
pub type HTIF2R = HTIF1R;
#[doc = "Possible values of the field `TEIF2`"]
pub type TEIF2R = TEIF1R;
#[doc = "Possible values of the field `GIF3`"]
pub type GIF3R = GIF1R;
#[doc = "Possible values of the field `TCIF3`"]
pub type TCIF3R = TCIF1R;
#[doc = "Possible values of the field `HTIF3`"]
pub type HTIF3R = HTIF1R;
#[doc = "Possible values of the field `TEIF3`"]
pub type TEIF3R = TEIF1R;
#[doc = "Possible values of the field `GIF4`"]
pub type GIF4R = GIF1R;
#[doc = "Possible values of the field `TCIF4`"]
pub type TCIF4R = TCIF1R;
#[doc = "Possible values of the field `HTIF4`"]
pub type HTIF4R = HTIF1R;
#[doc = "Possible values of the field `TEIF4`"]
pub type TEIF4R = TEIF1R;
#[doc = "Possible values of the field `GIF5`"]
pub type GIF5R = GIF1R;
#[doc = "Possible values of the field `TCIF5`"]
pub type TCIF5R = TCIF1R;
#[doc = "Possible values of the field `HTIF5`"]
pub type HTIF5R = HTIF1R;
#[doc = "Possible values of the field `TEIF5`"]
pub type TEIF5R = TEIF1R;
#[doc = "Possible values of the field `GIF6`"]
pub type GIF6R = GIF1R;
#[doc = "Possible values of the field `TCIF6`"]
pub type TCIF6R = TCIF1R;
#[doc = "Possible values of the field `HTIF6`"]
pub type HTIF6R = HTIF1R;
#[doc = "Possible values of the field `TEIF6`"]
pub type TEIF6R = TEIF1R;
#[doc = "Possible values of the field `GIF7`"]
pub type GIF7R = GIF1R;
#[doc = "Possible values of the field `TCIF7`"]
pub type TCIF7R = TCIF1R;
#[doc = "Possible values of the field `HTIF7`"]
pub type HTIF7R = HTIF1R;
#[doc = "Possible values of the field `TEIF7`"]
pub type TEIF7R = TEIF1R;
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 1 Global interrupt flag"]
    #[inline]
    pub fn gif1(&self) -> GIF1R {
        GIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete flag"]
    #[inline]
    pub fn tcif1(&self) -> TCIF1R {
        TCIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer Complete flag"]
    #[inline]
    pub fn htif1(&self) -> HTIF1R {
        HTIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error flag"]
    #[inline]
    pub fn teif1(&self) -> TEIF1R {
        TEIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt flag"]
    #[inline]
    pub fn gif2(&self) -> GIF2R {
        GIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete flag"]
    #[inline]
    pub fn tcif2(&self) -> TCIF2R {
        TCIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer Complete flag"]
    #[inline]
    pub fn htif2(&self) -> HTIF2R {
        HTIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error flag"]
    #[inline]
    pub fn teif2(&self) -> TEIF2R {
        TEIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt flag"]
    #[inline]
    pub fn gif3(&self) -> GIF3R {
        GIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete flag"]
    #[inline]
    pub fn tcif3(&self) -> TCIF3R {
        TCIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer Complete flag"]
    #[inline]
    pub fn htif3(&self) -> HTIF3R {
        HTIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error flag"]
    #[inline]
    pub fn teif3(&self) -> TEIF3R {
        TEIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt flag"]
    #[inline]
    pub fn gif4(&self) -> GIF4R {
        GIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete flag"]
    #[inline]
    pub fn tcif4(&self) -> TCIF4R {
        TCIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer Complete flag"]
    #[inline]
    pub fn htif4(&self) -> HTIF4R {
        HTIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error flag"]
    #[inline]
    pub fn teif4(&self) -> TEIF4R {
        TEIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt flag"]
    #[inline]
    pub fn gif5(&self) -> GIF5R {
        GIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete flag"]
    #[inline]
    pub fn tcif5(&self) -> TCIF5R {
        TCIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer Complete flag"]
    #[inline]
    pub fn htif5(&self) -> HTIF5R {
        HTIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error flag"]
    #[inline]
    pub fn teif5(&self) -> TEIF5R {
        TEIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt flag"]
    #[inline]
    pub fn gif6(&self) -> GIF6R {
        GIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete flag"]
    #[inline]
    pub fn tcif6(&self) -> TCIF6R {
        TCIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer Complete flag"]
    #[inline]
    pub fn htif6(&self) -> HTIF6R {
        HTIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error flag"]
    #[inline]
    pub fn teif6(&self) -> TEIF6R {
        TEIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt flag"]
    #[inline]
    pub fn gif7(&self) -> GIF7R {
        GIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete flag"]
    #[inline]
    pub fn tcif7(&self) -> TCIF7R {
        TCIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer Complete flag"]
    #[inline]
    pub fn htif7(&self) -> HTIF7R {
        HTIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error flag"]
    #[inline]
    pub fn teif7(&self) -> TEIF7R {
        TEIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
