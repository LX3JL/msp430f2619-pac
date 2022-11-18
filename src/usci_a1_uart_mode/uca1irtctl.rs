#[doc = "Register `UCA1IRTCTL` reader"]
pub struct R(crate::R<UCA1IRTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1IRTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1IRTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1IRTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1IRTCTL` writer"]
pub struct W(crate::W<UCA1IRTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1IRTCTL_SPEC>;
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
impl From<crate::W<UCA1IRTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1IRTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIREN` reader - IRDA Encoder/Decoder enable"]
pub type UCIREN_R = crate::BitReader<bool>;
#[doc = "Field `UCIREN` writer - IRDA Encoder/Decoder enable"]
pub type UCIREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRTCTL_SPEC, bool, O>;
#[doc = "Field `UCIRTXCLK` reader - IRDA Transmit Pulse Clock Select"]
pub type UCIRTXCLK_R = crate::BitReader<bool>;
#[doc = "Field `UCIRTXCLK` writer - IRDA Transmit Pulse Clock Select"]
pub type UCIRTXCLK_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRTCTL_SPEC, bool, O>;
#[doc = "Field `UCIRTXPL0` reader - IRDA Transmit Pulse Length 0"]
pub type UCIRTXPL0_R = crate::BitReader<bool>;
#[doc = "Field `UCIRTXPL0` writer - IRDA Transmit Pulse Length 0"]
pub type UCIRTXPL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRTCTL_SPEC, bool, O>;
#[doc = "Field `UCIRTXPL1` reader - IRDA Transmit Pulse Length 1"]
pub type UCIRTXPL1_R = crate::BitReader<bool>;
#[doc = "Field `UCIRTXPL1` writer - IRDA Transmit Pulse Length 1"]
pub type UCIRTXPL1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRTCTL_SPEC, bool, O>;
#[doc = "Field `UCIRTXPL2` reader - IRDA Transmit Pulse Length 2"]
pub type UCIRTXPL2_R = crate::BitReader<bool>;
#[doc = "Field `UCIRTXPL2` writer - IRDA Transmit Pulse Length 2"]
pub type UCIRTXPL2_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRTCTL_SPEC, bool, O>;
#[doc = "Field `UCIRTXPL3` reader - IRDA Transmit Pulse Length 3"]
pub type UCIRTXPL3_R = crate::BitReader<bool>;
#[doc = "Field `UCIRTXPL3` writer - IRDA Transmit Pulse Length 3"]
pub type UCIRTXPL3_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRTCTL_SPEC, bool, O>;
#[doc = "Field `UCIRTXPL4` reader - IRDA Transmit Pulse Length 4"]
pub type UCIRTXPL4_R = crate::BitReader<bool>;
#[doc = "Field `UCIRTXPL4` writer - IRDA Transmit Pulse Length 4"]
pub type UCIRTXPL4_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRTCTL_SPEC, bool, O>;
#[doc = "Field `UCIRTXPL5` reader - IRDA Transmit Pulse Length 5"]
pub type UCIRTXPL5_R = crate::BitReader<bool>;
#[doc = "Field `UCIRTXPL5` writer - IRDA Transmit Pulse Length 5"]
pub type UCIRTXPL5_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRTCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl0(&self) -> UCIRTXPL0_R {
        UCIRTXPL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRDA Transmit Pulse Length 1"]
    #[inline(always)]
    pub fn ucirtxpl1(&self) -> UCIRTXPL1_R {
        UCIRTXPL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRDA Transmit Pulse Length 2"]
    #[inline(always)]
    pub fn ucirtxpl2(&self) -> UCIRTXPL2_R {
        UCIRTXPL2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRDA Transmit Pulse Length 3"]
    #[inline(always)]
    pub fn ucirtxpl3(&self) -> UCIRTXPL3_R {
        UCIRTXPL3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRDA Transmit Pulse Length 4"]
    #[inline(always)]
    pub fn ucirtxpl4(&self) -> UCIRTXPL4_R {
        UCIRTXPL4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRDA Transmit Pulse Length 5"]
    #[inline(always)]
    pub fn ucirtxpl5(&self) -> UCIRTXPL5_R {
        UCIRTXPL5_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    #[must_use]
    pub fn uciren(&mut self) -> UCIREN_W<0> {
        UCIREN_W::new(self)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W<1> {
        UCIRTXCLK_W::new(self)
    }
    #[doc = "Bit 2 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxpl0(&mut self) -> UCIRTXPL0_W<2> {
        UCIRTXPL0_W::new(self)
    }
    #[doc = "Bit 3 - IRDA Transmit Pulse Length 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxpl1(&mut self) -> UCIRTXPL1_W<3> {
        UCIRTXPL1_W::new(self)
    }
    #[doc = "Bit 4 - IRDA Transmit Pulse Length 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxpl2(&mut self) -> UCIRTXPL2_W<4> {
        UCIRTXPL2_W::new(self)
    }
    #[doc = "Bit 5 - IRDA Transmit Pulse Length 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxpl3(&mut self) -> UCIRTXPL3_W<5> {
        UCIRTXPL3_W::new(self)
    }
    #[doc = "Bit 6 - IRDA Transmit Pulse Length 4"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxpl4(&mut self) -> UCIRTXPL4_W<6> {
        UCIRTXPL4_W::new(self)
    }
    #[doc = "Bit 7 - IRDA Transmit Pulse Length 5"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxpl5(&mut self) -> UCIRTXPL5_W<7> {
        UCIRTXPL5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USCI A1 IrDA Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irtctl](index.html) module"]
pub struct UCA1IRTCTL_SPEC;
impl crate::RegisterSpec for UCA1IRTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca1irtctl::R](R) reader structure"]
impl crate::Readable for UCA1IRTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1irtctl::W](W) writer structure"]
impl crate::Writable for UCA1IRTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1IRTCTL to value 0"]
impl crate::Resettable for UCA1IRTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
