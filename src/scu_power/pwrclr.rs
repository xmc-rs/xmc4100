#[doc = "Register `PWRCLR` writer"]
pub type W = crate::W<PwrclrSpec>;
#[doc = "Clear Disable Hibernate Domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hib {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable Hibernate domain"]
    Value2 = 1,
}
impl From<Hib> for bool {
    #[inline(always)]
    fn from(variant: Hib) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` writer - Clear Disable Hibernate Domain"]
pub type HibW<'a, REG> = crate::BitWriter<'a, REG, Hib>;
impl<'a, REG> HibW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hib::Value1)
    }
    #[doc = "Disable Hibernate domain"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hib::Value2)
    }
}
#[doc = "Clear USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbphypdq {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Power-down"]
    Value2 = 1,
}
impl From<Usbphypdq> for bool {
    #[inline(always)]
    fn from(variant: Usbphypdq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPHYPDQ` writer - Clear USB PHY Transceiver Disable"]
pub type UsbphypdqW<'a, REG> = crate::BitWriter<'a, REG, Usbphypdq>;
impl<'a, REG> UsbphypdqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbphypdq::Value1)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usbphypdq::Value2)
    }
}
#[doc = "Clear USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbpuwq {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Pull-up active"]
    Value2 = 1,
}
impl From<Usbpuwq> for bool {
    #[inline(always)]
    fn from(variant: Usbpuwq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPUWQ` writer - Clear USB Weak Pull-Up at PADN Enable"]
pub type UsbpuwqW<'a, REG> = crate::BitWriter<'a, REG, Usbpuwq>;
impl<'a, REG> UsbpuwqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbpuwq::Value1)
    }
    #[doc = "Pull-up active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usbpuwq::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Disable Hibernate Domain"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HibW<PwrclrSpec> {
        HibW::new(self, 0)
    }
    #[doc = "Bit 16 - Clear USB PHY Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbphypdq(&mut self) -> UsbphypdqW<PwrclrSpec> {
        UsbphypdqW::new(self, 16)
    }
    #[doc = "Bit 18 - Clear USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbpuwq(&mut self) -> UsbpuwqW<PwrclrSpec> {
        UsbpuwqW::new(self, 18)
    }
}
#[doc = "PCU Clear Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrclrSpec;
impl crate::RegisterSpec for PwrclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pwrclr::W`](W) writer structure"]
impl crate::Writable for PwrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCLR to value 0"]
impl crate::Resettable for PwrclrSpec {
    const RESET_VALUE: u32 = 0;
}
