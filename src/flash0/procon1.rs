#[doc = "Register `PROCON1` reader"]
pub type R = crate::R<Procon1Spec>;
#[doc = "Sector 0 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S0l> for bool {
    #[inline(always)]
    fn from(variant: S0l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0L` reader - Sector 0 Locked for Write Protection by User 1"]
pub type S0lR = crate::BitReader<S0l>;
impl S0lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0l {
        match self.bits {
            false => S0l::Value1,
            true => S0l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0l::Value2
    }
}
#[doc = "Sector 1 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S1l> for bool {
    #[inline(always)]
    fn from(variant: S1l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1L` reader - Sector 1 Locked for Write Protection by User 1"]
pub type S1lR = crate::BitReader<S1l>;
impl S1lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1l {
        match self.bits {
            false => S1l::Value1,
            true => S1l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1l::Value2
    }
}
#[doc = "Sector 2 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S2l> for bool {
    #[inline(always)]
    fn from(variant: S2l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2L` reader - Sector 2 Locked for Write Protection by User 1"]
pub type S2lR = crate::BitReader<S2l>;
impl S2lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2l {
        match self.bits {
            false => S2l::Value1,
            true => S2l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2l::Value2
    }
}
#[doc = "Sector 3 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S3l> for bool {
    #[inline(always)]
    fn from(variant: S3l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3L` reader - Sector 3 Locked for Write Protection by User 1"]
pub type S3lR = crate::BitReader<S3l>;
impl S3lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3l {
        match self.bits {
            false => S3l::Value1,
            true => S3l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3l::Value2
    }
}
#[doc = "Sector 4 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S4l> for bool {
    #[inline(always)]
    fn from(variant: S4l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4L` reader - Sector 4 Locked for Write Protection by User 1"]
pub type S4lR = crate::BitReader<S4l>;
impl S4lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S4l {
        match self.bits {
            false => S4l::Value1,
            true => S4l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S4l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S4l::Value2
    }
}
#[doc = "Sector 5 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S5l> for bool {
    #[inline(always)]
    fn from(variant: S5l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5L` reader - Sector 5 Locked for Write Protection by User 1"]
pub type S5lR = crate::BitReader<S5l>;
impl S5lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S5l {
        match self.bits {
            false => S5l::Value1,
            true => S5l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S5l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S5l::Value2
    }
}
#[doc = "Sector 6 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S6l> for bool {
    #[inline(always)]
    fn from(variant: S6l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6L` reader - Sector 6 Locked for Write Protection by User 1"]
pub type S6lR = crate::BitReader<S6l>;
impl S6lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S6l {
        match self.bits {
            false => S6l::Value1,
            true => S6l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S6l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S6l::Value2
    }
}
#[doc = "Sector 7 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S7l> for bool {
    #[inline(always)]
    fn from(variant: S7l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7L` reader - Sector 7 Locked for Write Protection by User 1"]
pub type S7lR = crate::BitReader<S7l>;
impl S7lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S7l {
        match self.bits {
            false => S7l::Value1,
            true => S7l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S7l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S7l::Value2
    }
}
#[doc = "Sector 8 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S8l> for bool {
    #[inline(always)]
    fn from(variant: S8l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8L` reader - Sector 8 Locked for Write Protection by User 1"]
pub type S8lR = crate::BitReader<S8l>;
impl S8lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S8l {
        match self.bits {
            false => S8l::Value1,
            true => S8l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S8l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S8l::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s0l(&self) -> S0lR {
        S0lR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s1l(&self) -> S1lR {
        S1lR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s2l(&self) -> S2lR {
        S2lR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s3l(&self) -> S3lR {
        S3lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s4l(&self) -> S4lR {
        S4lR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s5l(&self) -> S5lR {
        S5lR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s6l(&self) -> S6lR {
        S6lR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s7l(&self) -> S7lR {
        S7lR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s8l(&self) -> S8lR {
        S8lR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Flash Protection Configuration Register User 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`procon1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Procon1Spec;
impl crate::RegisterSpec for Procon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procon1::R`](R) reader structure"]
impl crate::Readable for Procon1Spec {}
#[doc = "`reset()` method sets PROCON1 to value 0"]
impl crate::Resettable for Procon1Spec {
    const RESET_VALUE: u32 = 0;
}
