#[doc = "Reader of register OSCHPCTRL"]
pub type R = crate::R<u32, super::OSCHPCTRL>;
#[doc = "Writer for register OSCHPCTRL"]
pub type W = crate::W<u32, super::OSCHPCTRL>;
#[doc = "Register OSCHPCTRL `reset()`'s with value 0x33"]
impl crate::ResetValue for super::OSCHPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x33
    }
}
#[doc = "XTAL1 Data Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X1DEN_A {
    #[doc = "0: Bit X1D is not updated"]
    VALUE1 = 0,
    #[doc = "1: Bit X1D can be updated"]
    VALUE2 = 1,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `X1DEN`"]
pub type X1DEN_R = crate::R<bool, X1DEN_A>;
impl X1DEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X1DEN_A {
        match self.bits {
            false => X1DEN_A::VALUE1,
            true => X1DEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == X1DEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == X1DEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `X1DEN`"]
pub struct X1DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> X1DEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: X1DEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit X1D is not updated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(X1DEN_A::VALUE1)
    }
    #[doc = "Bit X1D can be updated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(X1DEN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Shaper Bypass\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHBY_A {
    #[doc = "0: The shaper is not bypassed"]
    VALUE1 = 0,
    #[doc = "1: The shaper is bypassed"]
    VALUE2 = 1,
}
impl From<SHBY_A> for bool {
    #[inline(always)]
    fn from(variant: SHBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SHBY`"]
pub type SHBY_R = crate::R<bool, SHBY_A>;
impl SHBY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHBY_A {
        match self.bits {
            false => SHBY_A::VALUE1,
            true => SHBY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SHBY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SHBY_A::VALUE2
    }
}
#[doc = "Write proxy for field `SHBY`"]
pub struct SHBY_W<'a> {
    w: &'a mut W,
}
impl<'a> SHBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The shaper is not bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SHBY_A::VALUE1)
    }
    #[doc = "The shaper is bypassed"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SHBY_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Oscillator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    VALUE1 = 0,
    #[doc = "1: OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    VALUE2 = 1,
    #[doc = "2: External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    VALUE3 = 2,
    #[doc = "3: OSC is disabled. The oscillator Power-Saving Mode is entered."]
    VALUE4 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::VALUE1,
            1 => MODE_A::VALUE2,
            2 => MODE_A::VALUE3,
            3 => MODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MODE_A::VALUE4
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MODE_A::VALUE3)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `OSCVAL`"]
pub type OSCVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSCVAL`"]
pub struct OSCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1DEN_R {
        X1DEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&self) -> SHBY_R {
        SHBY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&self) -> OSCVAL_R {
        OSCVAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&mut self) -> X1DEN_W {
        X1DEN_W { w: self }
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&mut self) -> SHBY_W {
        SHBY_W { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&mut self) -> OSCVAL_W {
        OSCVAL_W { w: self }
    }
}
