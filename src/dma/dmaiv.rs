#[doc = "Register `DMAIV` reader"]
pub struct R(crate::R<DMAIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAIV` writer"]
pub struct W(crate::W<DMAIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAIV_SPEC>;
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
impl From<crate::W<DMAIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAIV_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaiv](index.html) module"]
pub struct DMAIV_SPEC;
impl crate::RegisterSpec for DMAIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmaiv::R](R) reader structure"]
impl crate::Readable for DMAIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaiv::W](W) writer structure"]
impl crate::Writable for DMAIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAIV to value 0"]
impl crate::Resettable for DMAIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
