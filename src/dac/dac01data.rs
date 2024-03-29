#[doc = "Register `DAC01DATA` reader"]
pub type R = crate::R<Dac01dataSpec>;
#[doc = "Register `DAC01DATA` writer"]
pub type W = crate::W<Dac01dataSpec>;
#[doc = "Field `DATA0` reader - DAC0 Data Bits"]
pub type Data0R = crate::FieldReader<u16>;
#[doc = "Field `DATA0` writer - DAC0 Data Bits"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DATA1` reader - DAC1 Data Bits"]
pub type Data1R = crate::FieldReader<u16>;
#[doc = "Field `DATA1` writer - DAC1 Data Bits"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC1 Data Bits"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<Dac01dataSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC1 Data Bits"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<Dac01dataSpec> {
        Data1W::new(self, 16)
    }
}
#[doc = "DAC01 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac01data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac01data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac01dataSpec;
impl crate::RegisterSpec for Dac01dataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac01data::R`](R) reader structure"]
impl crate::Readable for Dac01dataSpec {}
#[doc = "`write(|w| ..)` method takes [`dac01data::W`](W) writer structure"]
impl crate::Writable for Dac01dataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC01DATA to value 0"]
impl crate::Resettable for Dac01dataSpec {
    const RESET_VALUE: u32 = 0;
}
