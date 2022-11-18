#[doc = "Register `WDTCTL` reader"]
pub struct R(crate::R<WDTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCTL` writer"]
pub struct W(crate::W<WDTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCTL_SPEC>;
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
impl From<crate::W<WDTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIS0` reader - WDTIS0"]
pub type WDTIS0_R = crate::BitReader<bool>;
#[doc = "Field `WDTIS0` writer - WDTIS0"]
pub type WDTIS0_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTIS1` reader - WDTIS1"]
pub type WDTIS1_R = crate::BitReader<bool>;
#[doc = "Field `WDTIS1` writer - WDTIS1"]
pub type WDTIS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTSSEL` reader - WDTSSEL"]
pub type WDTSSEL_R = crate::BitReader<bool>;
#[doc = "Field `WDTSSEL` writer - WDTSSEL"]
pub type WDTSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTCNTCL` reader - WDTCNTCL"]
pub type WDTCNTCL_R = crate::BitReader<bool>;
#[doc = "Field `WDTCNTCL` writer - WDTCNTCL"]
pub type WDTCNTCL_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTTMSEL` reader - WDTTMSEL"]
pub type WDTTMSEL_R = crate::BitReader<bool>;
#[doc = "Field `WDTTMSEL` writer - WDTTMSEL"]
pub type WDTTMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTNMI` reader - WDTNMI"]
pub type WDTNMI_R = crate::BitReader<bool>;
#[doc = "Field `WDTNMI` writer - WDTNMI"]
pub type WDTNMI_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTNMIES` reader - WDTNMIES"]
pub type WDTNMIES_R = crate::BitReader<bool>;
#[doc = "Field `WDTNMIES` writer - WDTNMIES"]
pub type WDTNMIES_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTHOLD` reader - WDTHOLD"]
pub type WDTHOLD_R = crate::BitReader<bool>;
#[doc = "Field `WDTHOLD` writer - WDTHOLD"]
pub type WDTHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis0(&self) -> WDTIS0_R {
        WDTIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDTIS1"]
    #[inline(always)]
    pub fn wdtis1(&self) -> WDTIS1_R {
        WDTIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&self) -> WDTNMI_R {
        WDTNMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&self) -> WDTNMIES_R {
        WDTNMIES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTIS0"]
    #[inline(always)]
    #[must_use]
    pub fn wdtis0(&mut self) -> WDTIS0_W<0> {
        WDTIS0_W::new(self)
    }
    #[doc = "Bit 1 - WDTIS1"]
    #[inline(always)]
    #[must_use]
    pub fn wdtis1(&mut self) -> WDTIS1_W<1> {
        WDTIS1_W::new(self)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    #[must_use]
    pub fn wdtssel(&mut self) -> WDTSSEL_W<2> {
        WDTSSEL_W::new(self)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W<3> {
        WDTCNTCL_W::new(self)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W<4> {
        WDTTMSEL_W::new(self)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    #[must_use]
    pub fn wdtnmi(&mut self) -> WDTNMI_W<5> {
        WDTNMI_W::new(self)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    #[must_use]
    pub fn wdtnmies(&mut self) -> WDTNMIES_W<6> {
        WDTNMIES_W::new(self)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn wdthold(&mut self) -> WDTHOLD_W<7> {
        WDTHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtctl](index.html) module"]
pub struct WDTCTL_SPEC;
impl crate::RegisterSpec for WDTCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdtctl::R](R) reader structure"]
impl crate::Readable for WDTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtctl::W](W) writer structure"]
impl crate::Writable for WDTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WDTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
