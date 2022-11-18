#[doc = "Register `DMACTL1` reader"]
pub struct R(crate::R<DMACTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL1` writer"]
pub struct W(crate::W<DMACTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL1_SPEC>;
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
impl From<crate::W<DMACTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENNMI` reader - Enable NMI interruption of DMA"]
pub type ENNMI_R = crate::BitReader<bool>;
#[doc = "Field `ENNMI` writer - Enable NMI interruption of DMA"]
pub type ENNMI_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMACTL1_SPEC, bool, O>;
#[doc = "Field `ROUNDROBIN` reader - Round-Robin DMA channel priorities"]
pub type ROUNDROBIN_R = crate::BitReader<bool>;
#[doc = "Field `ROUNDROBIN` writer - Round-Robin DMA channel priorities"]
pub type ROUNDROBIN_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMACTL1_SPEC, bool, O>;
#[doc = "Field `DMAONFETCH` reader - DMA transfer on instruction fetch"]
pub type DMAONFETCH_R = crate::BitReader<bool>;
#[doc = "Field `DMAONFETCH` writer - DMA transfer on instruction fetch"]
pub type DMAONFETCH_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMACTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable NMI interruption of DMA"]
    #[inline(always)]
    pub fn ennmi(&self) -> ENNMI_R {
        ENNMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Round-Robin DMA channel priorities"]
    #[inline(always)]
    pub fn roundrobin(&self) -> ROUNDROBIN_R {
        ROUNDROBIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA transfer on instruction fetch"]
    #[inline(always)]
    pub fn dmaonfetch(&self) -> DMAONFETCH_R {
        DMAONFETCH_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable NMI interruption of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn ennmi(&mut self) -> ENNMI_W<0> {
        ENNMI_W::new(self)
    }
    #[doc = "Bit 1 - Round-Robin DMA channel priorities"]
    #[inline(always)]
    #[must_use]
    pub fn roundrobin(&mut self) -> ROUNDROBIN_W<1> {
        ROUNDROBIN_W::new(self)
    }
    #[doc = "Bit 2 - DMA transfer on instruction fetch"]
    #[inline(always)]
    #[must_use]
    pub fn dmaonfetch(&mut self) -> DMAONFETCH_W<2> {
        DMAONFETCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Module Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl1](index.html) module"]
pub struct DMACTL1_SPEC;
impl crate::RegisterSpec for DMACTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmactl1::R](R) reader structure"]
impl crate::Readable for DMACTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl1::W](W) writer structure"]
impl crate::Writable for DMACTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTL1 to value 0"]
impl crate::Resettable for DMACTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
