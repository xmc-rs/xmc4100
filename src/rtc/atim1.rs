#[doc = "Register `ATIM1` reader"]
pub type R = crate::R<Atim1Spec>;
#[doc = "Register `ATIM1` writer"]
pub type W = crate::W<Atim1Spec>;
#[doc = "Field `AMO` reader - Alarm Month Compare Value"]
pub type AmoR = crate::FieldReader;
#[doc = "Field `AMO` writer - Alarm Month Compare Value"]
pub type AmoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AYE` reader - Alarm Year Compare Value"]
pub type AyeR = crate::FieldReader<u16>;
#[doc = "Field `AYE` writer - Alarm Year Compare Value"]
pub type AyeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:11 - Alarm Month Compare Value"]
    #[inline(always)]
    pub fn amo(&self) -> AmoR {
        AmoR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Alarm Year Compare Value"]
    #[inline(always)]
    pub fn aye(&self) -> AyeR {
        AyeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:11 - Alarm Month Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn amo(&mut self) -> AmoW<Atim1Spec> {
        AmoW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Alarm Year Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn aye(&mut self) -> AyeW<Atim1Spec> {
        AyeW::new(self, 16)
    }
}
#[doc = "RTC Alarm Time Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atim1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atim1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Atim1Spec;
impl crate::RegisterSpec for Atim1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atim1::R`](R) reader structure"]
impl crate::Readable for Atim1Spec {}
#[doc = "`write(|w| ..)` method takes [`atim1::W`](W) writer structure"]
impl crate::Writable for Atim1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATIM1 to value 0"]
impl crate::Resettable for Atim1Spec {
    const RESET_VALUE: u32 = 0;
}
