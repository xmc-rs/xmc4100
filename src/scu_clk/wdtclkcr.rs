#[doc = "Register `WDTCLKCR` reader"]
pub type R = crate::R<WdtclkcrSpec>;
#[doc = "Register `WDTCLKCR` writer"]
pub type W = crate::W<WdtclkcrSpec>;
#[doc = "Field `WDTDIV` reader - WDT Clock Divider Value"]
pub type WdtdivR = crate::FieldReader;
#[doc = "Field `WDTDIV` writer - WDT Clock Divider Value"]
pub type WdtdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtsel {
    #[doc = "0: fOFI clock"]
    Value1 = 0,
    #[doc = "1: fSTDBY clock"]
    Value2 = 1,
    #[doc = "2: fPLL clock"]
    Value3 = 2,
}
impl From<Wdtsel> for u8 {
    #[inline(always)]
    fn from(variant: Wdtsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtsel {
    type Ux = u8;
}
#[doc = "Field `WDTSEL` reader - WDT Clock Selection Value"]
pub type WdtselR = crate::FieldReader<Wdtsel>;
impl WdtselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wdtsel> {
        match self.bits {
            0 => Some(Wdtsel::Value1),
            1 => Some(Wdtsel::Value2),
            2 => Some(Wdtsel::Value3),
            _ => None,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wdtsel::Value1
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wdtsel::Value2
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Wdtsel::Value3
    }
}
#[doc = "Field `WDTSEL` writer - WDT Clock Selection Value"]
pub type WdtselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wdtsel>;
impl<'a, REG> WdtselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtsel::Value1)
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtsel::Value2)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtsel::Value3)
    }
}
impl R {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WdtdivR {
        WdtdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WdtselR {
        WdtselR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtdiv(&mut self) -> WdtdivW<WdtclkcrSpec> {
        WdtdivW::new(self, 0)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtsel(&mut self) -> WdtselW<WdtclkcrSpec> {
        WdtselW::new(self, 16)
    }
}
#[doc = "WDT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtclkcrSpec;
impl crate::RegisterSpec for WdtclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtclkcr::R`](R) reader structure"]
impl crate::Readable for WdtclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtclkcr::W`](W) writer structure"]
impl crate::Writable for WdtclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCLKCR to value 0"]
impl crate::Resettable for WdtclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
