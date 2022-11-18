#[doc = "Register `UCA1IRRCTL` reader"]
pub struct R(crate::R<UCA1IRRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1IRRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1IRRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1IRRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1IRRCTL` writer"]
pub struct W(crate::W<UCA1IRRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1IRRCTL_SPEC>;
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
impl From<crate::W<UCA1IRRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1IRRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIRRXFE` reader - IRDA Receive Filter enable"]
pub type UCIRRXFE_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFE` writer - IRDA Receive Filter enable"]
pub type UCIRRXFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXPL` reader - IRDA Receive Input Polarity"]
pub type UCIRRXPL_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXPL` writer - IRDA Receive Input Polarity"]
pub type UCIRRXPL_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXFL0` reader - IRDA Receive Filter Length 0"]
pub type UCIRRXFL0_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFL0` writer - IRDA Receive Filter Length 0"]
pub type UCIRRXFL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXFL1` reader - IRDA Receive Filter Length 1"]
pub type UCIRRXFL1_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFL1` writer - IRDA Receive Filter Length 1"]
pub type UCIRRXFL1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXFL2` reader - IRDA Receive Filter Length 2"]
pub type UCIRRXFL2_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFL2` writer - IRDA Receive Filter Length 2"]
pub type UCIRRXFL2_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXFL3` reader - IRDA Receive Filter Length 3"]
pub type UCIRRXFL3_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFL3` writer - IRDA Receive Filter Length 3"]
pub type UCIRRXFL3_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXFL4` reader - IRDA Receive Filter Length 4"]
pub type UCIRRXFL4_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFL4` writer - IRDA Receive Filter Length 4"]
pub type UCIRRXFL4_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRRCTL_SPEC, bool, O>;
#[doc = "Field `UCIRRXFL5` reader - IRDA Receive Filter Length 5"]
pub type UCIRRXFL5_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFL5` writer - IRDA Receive Filter Length 5"]
pub type UCIRRXFL5_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IRRCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl0(&self) -> UCIRRXFL0_R {
        UCIRRXFL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    pub fn ucirrxfl1(&self) -> UCIRRXFL1_R {
        UCIRRXFL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    pub fn ucirrxfl2(&self) -> UCIRRXFL2_R {
        UCIRRXFL2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    pub fn ucirrxfl3(&self) -> UCIRRXFL3_R {
        UCIRRXFL3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    pub fn ucirrxfl4(&self) -> UCIRRXFL4_R {
        UCIRRXFL4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    pub fn ucirrxfl5(&self) -> UCIRRXFL5_R {
        UCIRRXFL5_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W<0> {
        UCIRRXFE_W::new(self)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W<1> {
        UCIRRXPL_W::new(self)
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl0(&mut self) -> UCIRRXFL0_W<2> {
        UCIRRXFL0_W::new(self)
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl1(&mut self) -> UCIRRXFL1_W<3> {
        UCIRRXFL1_W::new(self)
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl2(&mut self) -> UCIRRXFL2_W<4> {
        UCIRRXFL2_W::new(self)
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl3(&mut self) -> UCIRRXFL3_W<5> {
        UCIRRXFL3_W::new(self)
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl4(&mut self) -> UCIRRXFL4_W<6> {
        UCIRRXFL4_W::new(self)
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl5(&mut self) -> UCIRRXFL5_W<7> {
        UCIRRXFL5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USCI A1 IrDA Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irrctl](index.html) module"]
pub struct UCA1IRRCTL_SPEC;
impl crate::RegisterSpec for UCA1IRRCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca1irrctl::R](R) reader structure"]
impl crate::Readable for UCA1IRRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1irrctl::W](W) writer structure"]
impl crate::Writable for UCA1IRRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1IRRCTL to value 0"]
impl crate::Resettable for UCA1IRRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
