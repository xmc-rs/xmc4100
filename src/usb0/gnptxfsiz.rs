#[doc = "Register `GNPTXFSIZ` reader"]
pub type R = crate::R<GnptxfsizSpec>;
#[doc = "Register `GNPTXFSIZ` writer"]
pub type W = crate::W<GnptxfsizSpec>;
#[doc = "Field `INEPTxF0StAddr` reader - IN Endpoint FIFO0 Transmit RAM Start Address"]
pub type IneptxF0stAddrR = crate::FieldReader<u16>;
#[doc = "Field `INEPTxF0StAddr` writer - IN Endpoint FIFO0 Transmit RAM Start Address"]
pub type IneptxF0stAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTxF0Dep` reader - IN Endpoint TxFIFO 0 Depth"]
pub type IneptxF0depR = crate::FieldReader<u16>;
#[doc = "Field `INEPTxF0Dep` writer - IN Endpoint TxFIFO 0 Depth"]
pub type IneptxF0depW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn ineptx_f0st_addr(&self) -> IneptxF0stAddrR {
        IneptxF0stAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn ineptx_f0dep(&self) -> IneptxF0depR {
        IneptxF0depR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptx_f0st_addr(&mut self) -> IneptxF0stAddrW<GnptxfsizSpec> {
        IneptxF0stAddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptx_f0dep(&mut self) -> IneptxF0depW<GnptxfsizSpec> {
        IneptxF0depW::new(self, 16)
    }
}
#[doc = "Non-Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnptxfsizSpec;
impl crate::RegisterSpec for GnptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz::R`](R) reader structure"]
impl crate::Readable for GnptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz::W`](W) writer structure"]
impl crate::Writable for GnptxfsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ to value 0x0010_011a"]
impl crate::Resettable for GnptxfsizSpec {
    const RESET_VALUE: u32 = 0x0010_011a;
}
