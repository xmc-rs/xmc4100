#[doc = "Reader of register CLKSTAT"]
pub type R = crate::R<u32, super::CLKSTAT>;
#[doc = "USB Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCST_A {
    #[doc = "0: Clock disabled"]
    VALUE1,
    #[doc = "1: Clock enabled"]
    VALUE2,
}
impl From<USBCST_A> for bool {
    #[inline(always)]
    fn from(variant: USBCST_A) -> Self {
        match variant {
            USBCST_A::VALUE1 => false,
            USBCST_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `USBCST`"]
pub type USBCST_R = crate::R<bool, USBCST_A>;
impl USBCST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCST_A {
        match self.bits {
            false => USBCST_A::VALUE1,
            true => USBCST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBCST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBCST_A::VALUE2
    }
}
#[doc = "CCU Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCST_A {
    #[doc = "0: Clock disabled"]
    VALUE1,
    #[doc = "1: Clock enabled"]
    VALUE2,
}
impl From<CCUCST_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCST_A) -> Self {
        match variant {
            CCUCST_A::VALUE1 => false,
            CCUCST_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCUCST`"]
pub type CCUCST_R = crate::R<bool, CCUCST_A>;
impl CCUCST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUCST_A {
        match self.bits {
            false => CCUCST_A::VALUE1,
            true => CCUCST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCUCST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCUCST_A::VALUE2
    }
}
#[doc = "WDT Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCST_A {
    #[doc = "0: Clock disabled"]
    VALUE1,
    #[doc = "1: Clock enabled"]
    VALUE2,
}
impl From<WDTCST_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCST_A) -> Self {
        match variant {
            WDTCST_A::VALUE1 => false,
            WDTCST_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WDTCST`"]
pub type WDTCST_R = crate::R<bool, WDTCST_A>;
impl WDTCST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCST_A {
        match self.bits {
            false => WDTCST_A::VALUE1,
            true => WDTCST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDTCST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDTCST_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Status"]
    #[inline(always)]
    pub fn usbcst(&self) -> USBCST_R {
        USBCST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CCU Clock Status"]
    #[inline(always)]
    pub fn ccucst(&self) -> CCUCST_R {
        CCUCST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WDT Clock Status"]
    #[inline(always)]
    pub fn wdtcst(&self) -> WDTCST_R {
        WDTCST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}