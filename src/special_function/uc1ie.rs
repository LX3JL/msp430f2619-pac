#[doc = "Register `UC1IE` reader"]
pub struct R(crate::R<UC1IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UC1IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UC1IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UC1IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UC1IE` writer"]
pub struct W(crate::W<UC1IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UC1IE_SPEC>;
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
impl From<crate::W<UC1IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UC1IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA1RXIE` reader - UCA1RXIE"]
pub type UCA1RXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCA1RXIE` writer - UCA1RXIE"]
pub type UCA1RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UC1IE_SPEC, bool, O>;
#[doc = "Field `UCA1TXIE` reader - UCA1TXIE"]
pub type UCA1TXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCA1TXIE` writer - UCA1TXIE"]
pub type UCA1TXIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UC1IE_SPEC, bool, O>;
#[doc = "Field `UCB1RXIE` reader - UCB1RXIE"]
pub type UCB1RXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCB1RXIE` writer - UCB1RXIE"]
pub type UCB1RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UC1IE_SPEC, bool, O>;
#[doc = "Field `UCB1TXIE` reader - UCB1TXIE"]
pub type UCB1TXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCB1TXIE` writer - UCB1TXIE"]
pub type UCB1TXIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UC1IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UCA1RXIE"]
    #[inline(always)]
    pub fn uca1rxie(&self) -> UCA1RXIE_R {
        UCA1RXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA1TXIE"]
    #[inline(always)]
    pub fn uca1txie(&self) -> UCA1TXIE_R {
        UCA1TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB1RXIE"]
    #[inline(always)]
    pub fn ucb1rxie(&self) -> UCB1RXIE_R {
        UCB1RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB1TXIE"]
    #[inline(always)]
    pub fn ucb1txie(&self) -> UCB1TXIE_R {
        UCB1TXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA1RXIE"]
    #[inline(always)]
    #[must_use]
    pub fn uca1rxie(&mut self) -> UCA1RXIE_W<0> {
        UCA1RXIE_W::new(self)
    }
    #[doc = "Bit 1 - UCA1TXIE"]
    #[inline(always)]
    #[must_use]
    pub fn uca1txie(&mut self) -> UCA1TXIE_W<1> {
        UCA1TXIE_W::new(self)
    }
    #[doc = "Bit 2 - UCB1RXIE"]
    #[inline(always)]
    #[must_use]
    pub fn ucb1rxie(&mut self) -> UCB1RXIE_W<2> {
        UCB1RXIE_W::new(self)
    }
    #[doc = "Bit 3 - UCB1TXIE"]
    #[inline(always)]
    #[must_use]
    pub fn ucb1txie(&mut self) -> UCB1TXIE_W<3> {
        UCB1TXIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI 1 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uc1ie](index.html) module"]
pub struct UC1IE_SPEC;
impl crate::RegisterSpec for UC1IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uc1ie::R](R) reader structure"]
impl crate::Readable for UC1IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uc1ie::W](W) writer structure"]
impl crate::Writable for UC1IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UC1IE to value 0"]
impl crate::Resettable for UC1IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
