#[doc = "Register `UCA0MCTL_SPI` reader"]
pub struct R(crate::R<UCA0MCTL_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0MCTL_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0MCTL_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0MCTL_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0MCTL_SPI` writer"]
pub struct W(crate::W<UCA0MCTL_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0MCTL_SPI_SPEC>;
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
impl From<crate::W<UCA0MCTL_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0MCTL_SPI_SPEC>) -> Self {
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
#[doc = "USCI A0 Modulation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0mctl_spi](index.html) module"]
pub struct UCA0MCTL_SPI_SPEC;
impl crate::RegisterSpec for UCA0MCTL_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0mctl_spi::R](R) reader structure"]
impl crate::Readable for UCA0MCTL_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0mctl_spi::W](W) writer structure"]
impl crate::Writable for UCA0MCTL_SPI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0MCTL_SPI to value 0"]
impl crate::Resettable for UCA0MCTL_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
