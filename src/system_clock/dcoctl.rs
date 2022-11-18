#[doc = "Register `DCOCTL` reader"]
pub struct R(crate::R<DCOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCOCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCOCTL` writer"]
pub struct W(crate::W<DCOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCOCTL_SPEC>;
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
impl From<crate::W<DCOCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD0` reader - Modulation Bit 0"]
pub type MOD0_R = crate::BitReader<bool>;
#[doc = "Field `MOD0` writer - Modulation Bit 0"]
pub type MOD0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCOCTL_SPEC, bool, O>;
#[doc = "Field `MOD1` reader - Modulation Bit 1"]
pub type MOD1_R = crate::BitReader<bool>;
#[doc = "Field `MOD1` writer - Modulation Bit 1"]
pub type MOD1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCOCTL_SPEC, bool, O>;
#[doc = "Field `MOD2` reader - Modulation Bit 2"]
pub type MOD2_R = crate::BitReader<bool>;
#[doc = "Field `MOD2` writer - Modulation Bit 2"]
pub type MOD2_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCOCTL_SPEC, bool, O>;
#[doc = "Field `MOD3` reader - Modulation Bit 3"]
pub type MOD3_R = crate::BitReader<bool>;
#[doc = "Field `MOD3` writer - Modulation Bit 3"]
pub type MOD3_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCOCTL_SPEC, bool, O>;
#[doc = "Field `MOD4` reader - Modulation Bit 4"]
pub type MOD4_R = crate::BitReader<bool>;
#[doc = "Field `MOD4` writer - Modulation Bit 4"]
pub type MOD4_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCOCTL_SPEC, bool, O>;
#[doc = "Field `DCO0` reader - DCO Select Bit 0"]
pub type DCO0_R = crate::BitReader<bool>;
#[doc = "Field `DCO0` writer - DCO Select Bit 0"]
pub type DCO0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCOCTL_SPEC, bool, O>;
#[doc = "Field `DCO1` reader - DCO Select Bit 1"]
pub type DCO1_R = crate::BitReader<bool>;
#[doc = "Field `DCO1` writer - DCO Select Bit 1"]
pub type DCO1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCOCTL_SPEC, bool, O>;
#[doc = "Field `DCO2` reader - DCO Select Bit 2"]
pub type DCO2_R = crate::BitReader<bool>;
#[doc = "Field `DCO2` writer - DCO Select Bit 2"]
pub type DCO2_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCOCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod0(&self) -> MOD0_R {
        MOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Modulation Bit 1"]
    #[inline(always)]
    pub fn mod1(&self) -> MOD1_R {
        MOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Modulation Bit 2"]
    #[inline(always)]
    pub fn mod2(&self) -> MOD2_R {
        MOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Modulation Bit 3"]
    #[inline(always)]
    pub fn mod3(&self) -> MOD3_R {
        MOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Modulation Bit 4"]
    #[inline(always)]
    pub fn mod4(&self) -> MOD4_R {
        MOD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco0(&self) -> DCO0_R {
        DCO0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCO Select Bit 1"]
    #[inline(always)]
    pub fn dco1(&self) -> DCO1_R {
        DCO1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCO Select Bit 2"]
    #[inline(always)]
    pub fn dco2(&self) -> DCO2_R {
        DCO2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Modulation Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mod0(&mut self) -> MOD0_W<0> {
        MOD0_W::new(self)
    }
    #[doc = "Bit 1 - Modulation Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mod1(&mut self) -> MOD1_W<1> {
        MOD1_W::new(self)
    }
    #[doc = "Bit 2 - Modulation Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn mod2(&mut self) -> MOD2_W<2> {
        MOD2_W::new(self)
    }
    #[doc = "Bit 3 - Modulation Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn mod3(&mut self) -> MOD3_W<3> {
        MOD3_W::new(self)
    }
    #[doc = "Bit 4 - Modulation Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn mod4(&mut self) -> MOD4_W<4> {
        MOD4_W::new(self)
    }
    #[doc = "Bit 5 - DCO Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dco0(&mut self) -> DCO0_W<5> {
        DCO0_W::new(self)
    }
    #[doc = "Bit 6 - DCO Select Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dco1(&mut self) -> DCO1_W<6> {
        DCO1_W::new(self)
    }
    #[doc = "Bit 7 - DCO Select Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dco2(&mut self) -> DCO2_W<7> {
        DCO2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "DCO Clock Frequency Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoctl](index.html) module"]
pub struct DCOCTL_SPEC;
impl crate::RegisterSpec for DCOCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dcoctl::R](R) reader structure"]
impl crate::Readable for DCOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcoctl::W](W) writer structure"]
impl crate::Writable for DCOCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCOCTL to value 0"]
impl crate::Resettable for DCOCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
