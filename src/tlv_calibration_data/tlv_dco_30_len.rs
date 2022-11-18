#[doc = "Register `TLV_DCO_30_LEN` reader"]
pub struct R(crate::R<TLV_DCO_30_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLV_DCO_30_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLV_DCO_30_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLV_DCO_30_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TLV_DCO_30_LEN` writer"]
pub struct W(crate::W<TLV_DCO_30_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TLV_DCO_30_LEN_SPEC>;
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
impl From<crate::W<TLV_DCO_30_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TLV_DCO_30_LEN_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TLV TAG_DCO30 LEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_dco_30_len](index.html) module"]
pub struct TLV_DCO_30_LEN_SPEC;
impl crate::RegisterSpec for TLV_DCO_30_LEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tlv_dco_30_len::R](R) reader structure"]
impl crate::Readable for TLV_DCO_30_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tlv_dco_30_len::W](W) writer structure"]
impl crate::Writable for TLV_DCO_30_LEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TLV_DCO_30_LEN to value 0"]
impl crate::Resettable for TLV_DCO_30_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
