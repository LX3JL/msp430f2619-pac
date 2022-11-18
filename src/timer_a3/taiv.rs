#[doc = "Register `TAIV` reader"]
pub struct R(crate::R<TAIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAIV` writer"]
pub struct W(crate::W<TAIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAIV_SPEC>;
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
impl From<crate::W<TAIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAIV_SPEC>) -> Self {
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
#[doc = "Timer A Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taiv](index.html) module"]
pub struct TAIV_SPEC;
impl crate::RegisterSpec for TAIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [taiv::R](R) reader structure"]
impl crate::Readable for TAIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taiv::W](W) writer structure"]
impl crate::Writable for TAIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAIV to value 0"]
impl crate::Resettable for TAIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
