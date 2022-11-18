#[doc = "Register `DAC12_0DAT` reader"]
pub struct R(crate::R<DAC12_0DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC12_0DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC12_0DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC12_0DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC12_0DAT` writer"]
pub struct W(crate::W<DAC12_0DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC12_0DAT_SPEC>;
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
impl From<crate::W<DAC12_0DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC12_0DAT_SPEC>) -> Self {
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
#[doc = "DAC12_0 Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac12_0dat](index.html) module"]
pub struct DAC12_0DAT_SPEC;
impl crate::RegisterSpec for DAC12_0DAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dac12_0dat::R](R) reader structure"]
impl crate::Readable for DAC12_0DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac12_0dat::W](W) writer structure"]
impl crate::Writable for DAC12_0DAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC12_0DAT to value 0"]
impl crate::Resettable for DAC12_0DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
