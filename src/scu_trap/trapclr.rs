#[doc = "Register `TRAPCLR` writer"]
pub type W = crate::W<TRAPCLR_SPEC>;
#[doc = "System OSC WDT Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSCWDGT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` writer - System OSC WDT Trap Clear"]
pub type SOSCWDGT_W<'a, REG> = crate::BitWriter<'a, REG, SOSCWDGT_A>;
impl<'a, REG> SOSCWDGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_A::VALUE2)
    }
}
#[doc = "System VCO Lock Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCOLCKT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Clear"]
pub type SVCOLCKT_W<'a, REG> = crate::BitWriter<'a, REG, SVCOLCKT_A>;
impl<'a, REG> SVCOLCKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_A::VALUE2)
    }
}
#[doc = "USB VCO Lock Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVCOLCKT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Clear"]
pub type UVCOLCKT_W<'a, REG> = crate::BitWriter<'a, REG, UVCOLCKT_A>;
impl<'a, REG> UVCOLCKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PET_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Clear"]
pub type PET_W<'a, REG> = crate::BitWriter<'a, REG, PET_A>;
impl<'a, REG> PET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PET_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PET_A::VALUE2)
    }
}
#[doc = "Brown Out Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRWNT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Clear"]
pub type BRWNT_W<'a, REG> = crate::BitWriter<'a, REG, BRWNT_A>;
impl<'a, REG> BRWNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_A::VALUE2)
    }
}
#[doc = "OSC_ULP WDG Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDGT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` writer - OSC_ULP WDG Trap Clear"]
pub type ULPWDGT_W<'a, REG> = crate::BitWriter<'a, REG, ULPWDGT_A>;
impl<'a, REG> ULPWDGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDGT_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDGT_A::VALUE2)
    }
}
#[doc = "Peripheral Bridge 0 Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR0T_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Clear"]
pub type BWERR0T_W<'a, REG> = crate::BitWriter<'a, REG, BWERR0T_A>;
impl<'a, REG> BWERR0T_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_A::VALUE2)
    }
}
#[doc = "Peripheral Bridge 1 Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR1T_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Clear"]
pub type BWERR1T_W<'a, REG> = crate::BitWriter<'a, REG, BWERR1T_A>;
impl<'a, REG> BWERR1T_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_A::VALUE2)
    }
}
#[doc = "Die Temperature Too High Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPHIT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<TEMPHIT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPHIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPHIT` writer - Die Temperature Too High Trap Clear"]
pub type TEMPHIT_W<'a, REG> = crate::BitWriter<'a, REG, TEMPHIT_A>;
impl<'a, REG> TEMPHIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TEMPHIT_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TEMPHIT_A::VALUE2)
    }
}
#[doc = "Die Temperature Too Low Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPLOT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear trap request"]
    VALUE2 = 1,
}
impl From<TEMPLOT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPLOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPLOT` writer - Die Temperature Too Low Trap Clear"]
pub type TEMPLOT_W<'a, REG> = crate::BitWriter<'a, REG, TEMPLOT_A>;
impl<'a, REG> TEMPLOT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TEMPLOT_A::VALUE1)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TEMPLOT_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - System OSC WDT Trap Clear"]
    #[inline(always)]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W<TRAPCLR_SPEC> {
        SOSCWDGT_W::new(self, 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Clear"]
    #[inline(always)]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W<TRAPCLR_SPEC> {
        SVCOLCKT_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Clear"]
    #[inline(always)]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W<TRAPCLR_SPEC> {
        UVCOLCKT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Parity Error Trap Clear"]
    #[inline(always)]
    pub fn pet(&mut self) -> PET_W<TRAPCLR_SPEC> {
        PET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Brown Out Trap Clear"]
    #[inline(always)]
    pub fn brwnt(&mut self) -> BRWNT_W<TRAPCLR_SPEC> {
        BRWNT_W::new(self, 5)
    }
    #[doc = "Bit 6 - OSC_ULP WDG Trap Clear"]
    #[inline(always)]
    pub fn ulpwdgt(&mut self) -> ULPWDGT_W<TRAPCLR_SPEC> {
        ULPWDGT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Clear"]
    #[inline(always)]
    pub fn bwerr0t(&mut self) -> BWERR0T_W<TRAPCLR_SPEC> {
        BWERR0T_W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Clear"]
    #[inline(always)]
    pub fn bwerr1t(&mut self) -> BWERR1T_W<TRAPCLR_SPEC> {
        BWERR1T_W::new(self, 8)
    }
    #[doc = "Bit 12 - Die Temperature Too High Trap Clear"]
    #[inline(always)]
    pub fn temphit(&mut self) -> TEMPHIT_W<TRAPCLR_SPEC> {
        TEMPHIT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Die Temperature Too Low Trap Clear"]
    #[inline(always)]
    pub fn templot(&mut self) -> TEMPLOT_W<TRAPCLR_SPEC> {
        TEMPLOT_W::new(self, 13)
    }
}
#[doc = "Trap Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trapclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRAPCLR_SPEC;
impl crate::RegisterSpec for TRAPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trapclr::W`](W) writer structure"]
impl crate::Writable for TRAPCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRAPCLR to value 0"]
impl crate::Resettable for TRAPCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
