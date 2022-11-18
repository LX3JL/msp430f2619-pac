#[doc = "Register `UCA1STAT_SPI` reader"]
pub struct R(crate::R<UCA1STAT_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1STAT_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1STAT_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1STAT_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1STAT_SPI` writer"]
pub struct W(crate::W<UCA1STAT_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1STAT_SPI_SPEC>;
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
impl From<crate::W<UCA1STAT_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1STAT_SPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBUSY` reader - USCI Busy Flag"]
pub type UCBUSY_R = crate::BitReader<bool>;
#[doc = "Field `UCBUSY` writer - USCI Busy Flag"]
pub type UCBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1STAT_SPI_SPEC, bool, O>;
#[doc = "Field `UCOE` reader - USCI Overrun Error Flag"]
pub type UCOE_R = crate::BitReader<bool>;
#[doc = "Field `UCOE` writer - USCI Overrun Error Flag"]
pub type UCOE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1STAT_SPI_SPEC, bool, O>;
#[doc = "Field `UCFE` reader - USCI Frame Error Flag"]
pub type UCFE_R = crate::BitReader<bool>;
#[doc = "Field `UCFE` writer - USCI Frame Error Flag"]
pub type UCFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1STAT_SPI_SPEC, bool, O>;
#[doc = "Field `UCLISTEN` reader - USCI Listen mode"]
pub type UCLISTEN_R = crate::BitReader<bool>;
#[doc = "Field `UCLISTEN` writer - USCI Listen mode"]
pub type UCLISTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1STAT_SPI_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UCBUSY_R {
        UCBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UCOE_R {
        UCOE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UCFE_R {
        UCFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucbusy(&mut self) -> UCBUSY_W<0> {
        UCBUSY_W::new(self)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucoe(&mut self) -> UCOE_W<5> {
        UCOE_W::new(self)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucfe(&mut self) -> UCFE_W<6> {
        UCFE_W::new(self)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn uclisten(&mut self) -> UCLISTEN_W<7> {
        UCLISTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1stat_spi](index.html) module"]
pub struct UCA1STAT_SPI_SPEC;
impl crate::RegisterSpec for UCA1STAT_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca1stat_spi::R](R) reader structure"]
impl crate::Readable for UCA1STAT_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1stat_spi::W](W) writer structure"]
impl crate::Writable for UCA1STAT_SPI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1STAT_SPI to value 0"]
impl crate::Resettable for UCA1STAT_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
