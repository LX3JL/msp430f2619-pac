#[doc = "Register `FCTL1` reader"]
pub struct R(crate::R<FCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTL1` writer"]
pub struct W(crate::W<FCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASE` reader - Enable bit for Flash segment erase"]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `ERASE` writer - Enable bit for Flash segment erase"]
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, O>;
#[doc = "Field `MERAS` reader - Enable bit for Flash mass erase"]
pub type MERAS_R = crate::BitReader<bool>;
#[doc = "Field `MERAS` writer - Enable bit for Flash mass erase"]
pub type MERAS_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, O>;
#[doc = "Field `EEI` reader - Enable Erase Interrupts"]
pub type EEI_R = crate::BitReader<bool>;
#[doc = "Field `EEI` writer - Enable Erase Interrupts"]
pub type EEI_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, O>;
#[doc = "Field `EEIEX` reader - Enable Emergency Interrupt Exit"]
pub type EEIEX_R = crate::BitReader<bool>;
#[doc = "Field `EEIEX` writer - Enable Emergency Interrupt Exit"]
pub type EEIEX_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, O>;
#[doc = "Field `WRT` reader - Enable bit for Flash write"]
pub type WRT_R = crate::BitReader<bool>;
#[doc = "Field `WRT` writer - Enable bit for Flash write"]
pub type WRT_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, O>;
#[doc = "Field `BLKWRT` reader - Enable bit for Flash segment write"]
pub type BLKWRT_R = crate::BitReader<bool>;
#[doc = "Field `BLKWRT` writer - Enable bit for Flash segment write"]
pub type BLKWRT_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&self) -> MERAS_R {
        MERAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Erase Interrupts"]
    #[inline(always)]
    pub fn eei(&self) -> EEI_R {
        EEI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Emergency Interrupt Exit"]
    #[inline(always)]
    pub fn eeiex(&self) -> EEIEX_R {
        EEIEX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&self) -> WRT_R {
        WRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&self) -> BLKWRT_R {
        BLKWRT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<1> {
        ERASE_W::new(self)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn meras(&mut self) -> MERAS_W<2> {
        MERAS_W::new(self)
    }
    #[doc = "Bit 3 - Enable Erase Interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn eei(&mut self) -> EEI_W<3> {
        EEI_W::new(self)
    }
    #[doc = "Bit 4 - Enable Emergency Interrupt Exit"]
    #[inline(always)]
    #[must_use]
    pub fn eeiex(&mut self) -> EEIEX_W<4> {
        EEIEX_W::new(self)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    #[must_use]
    pub fn wrt(&mut self) -> WRT_W<6> {
        WRT_W::new(self)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    #[must_use]
    pub fn blkwrt(&mut self) -> BLKWRT_W<7> {
        BLKWRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl1](index.html) module"]
pub struct FCTL1_SPEC;
impl crate::RegisterSpec for FCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fctl1::R](R) reader structure"]
impl crate::Readable for FCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctl1::W](W) writer structure"]
impl crate::Writable for FCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTL1 to value 0"]
impl crate::Resettable for FCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
