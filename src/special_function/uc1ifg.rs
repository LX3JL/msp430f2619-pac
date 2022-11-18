#[doc = "Register `UC1IFG` reader"]
pub struct R(crate::R<UC1IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UC1IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UC1IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UC1IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UC1IFG` writer"]
pub struct W(crate::W<UC1IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UC1IFG_SPEC>;
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
impl From<crate::W<UC1IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UC1IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA1RXIFG` reader - UCA1RXIFG"]
pub type UCA1RXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCA1RXIFG` writer - UCA1RXIFG"]
pub type UCA1RXIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, UC1IFG_SPEC, bool, O>;
#[doc = "Field `UCA1TXIFG` reader - UCA1TXIFG"]
pub type UCA1TXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCA1TXIFG` writer - UCA1TXIFG"]
pub type UCA1TXIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, UC1IFG_SPEC, bool, O>;
#[doc = "Field `UCB1RXIFG` reader - UCB1RXIFG"]
pub type UCB1RXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCB1RXIFG` writer - UCB1RXIFG"]
pub type UCB1RXIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, UC1IFG_SPEC, bool, O>;
#[doc = "Field `UCB1TXIFG` reader - UCB1TXIFG"]
pub type UCB1TXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCB1TXIFG` writer - UCB1TXIFG"]
pub type UCB1TXIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, UC1IFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UCA1RXIFG"]
    #[inline(always)]
    pub fn uca1rxifg(&self) -> UCA1RXIFG_R {
        UCA1RXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA1TXIFG"]
    #[inline(always)]
    pub fn uca1txifg(&self) -> UCA1TXIFG_R {
        UCA1TXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB1RXIFG"]
    #[inline(always)]
    pub fn ucb1rxifg(&self) -> UCB1RXIFG_R {
        UCB1RXIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB1TXIFG"]
    #[inline(always)]
    pub fn ucb1txifg(&self) -> UCB1TXIFG_R {
        UCB1TXIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA1RXIFG"]
    #[inline(always)]
    #[must_use]
    pub fn uca1rxifg(&mut self) -> UCA1RXIFG_W<0> {
        UCA1RXIFG_W::new(self)
    }
    #[doc = "Bit 1 - UCA1TXIFG"]
    #[inline(always)]
    #[must_use]
    pub fn uca1txifg(&mut self) -> UCA1TXIFG_W<1> {
        UCA1TXIFG_W::new(self)
    }
    #[doc = "Bit 2 - UCB1RXIFG"]
    #[inline(always)]
    #[must_use]
    pub fn ucb1rxifg(&mut self) -> UCB1RXIFG_W<2> {
        UCB1RXIFG_W::new(self)
    }
    #[doc = "Bit 3 - UCB1TXIFG"]
    #[inline(always)]
    #[must_use]
    pub fn ucb1txifg(&mut self) -> UCB1TXIFG_W<3> {
        UCB1TXIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISCI 1 Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uc1ifg](index.html) module"]
pub struct UC1IFG_SPEC;
impl crate::RegisterSpec for UC1IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uc1ifg::R](R) reader structure"]
impl crate::Readable for UC1IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uc1ifg::W](W) writer structure"]
impl crate::Writable for UC1IFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UC1IFG to value 0"]
impl crate::Resettable for UC1IFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
