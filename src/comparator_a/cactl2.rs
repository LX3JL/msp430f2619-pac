#[doc = "Register `CACTL2` reader"]
pub struct R(crate::R<CACTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACTL2` writer"]
pub struct W(crate::W<CACTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACTL2_SPEC>;
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
impl From<crate::W<CACTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAOUT` reader - Comp. A Output"]
pub type CAOUT_R = crate::BitReader<bool>;
#[doc = "Field `CAOUT` writer - Comp. A Output"]
pub type CAOUT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACTL2_SPEC, bool, O>;
#[doc = "Field `CAF` reader - Comp. A Enable Output Filter"]
pub type CAF_R = crate::BitReader<bool>;
#[doc = "Field `CAF` writer - Comp. A Enable Output Filter"]
pub type CAF_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACTL2_SPEC, bool, O>;
#[doc = "Field `P2CA0` reader - Comp. A +Terminal Multiplexer"]
pub type P2CA0_R = crate::BitReader<bool>;
#[doc = "Field `P2CA0` writer - Comp. A +Terminal Multiplexer"]
pub type P2CA0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACTL2_SPEC, bool, O>;
#[doc = "Field `P2CA1` reader - Comp. A -Terminal Multiplexer"]
pub type P2CA1_R = crate::BitReader<bool>;
#[doc = "Field `P2CA1` writer - Comp. A -Terminal Multiplexer"]
pub type P2CA1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACTL2_SPEC, bool, O>;
#[doc = "Field `P2CA2` reader - Comp. A -Terminal Multiplexer"]
pub type P2CA2_R = crate::BitReader<bool>;
#[doc = "Field `P2CA2` writer - Comp. A -Terminal Multiplexer"]
pub type P2CA2_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACTL2_SPEC, bool, O>;
#[doc = "Field `P2CA3` reader - Comp. A -Terminal Multiplexer"]
pub type P2CA3_R = crate::BitReader<bool>;
#[doc = "Field `P2CA3` writer - Comp. A -Terminal Multiplexer"]
pub type P2CA3_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACTL2_SPEC, bool, O>;
#[doc = "Field `P2CA4` reader - Comp. A +Terminal Multiplexer"]
pub type P2CA4_R = crate::BitReader<bool>;
#[doc = "Field `P2CA4` writer - Comp. A +Terminal Multiplexer"]
pub type P2CA4_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACTL2_SPEC, bool, O>;
#[doc = "Field `CASHORT` reader - Comp. A Short + and - Terminals"]
pub type CASHORT_R = crate::BitReader<bool>;
#[doc = "Field `CASHORT` writer - Comp. A Short + and - Terminals"]
pub type CASHORT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&self) -> CAOUT_R {
        CAOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca0(&self) -> P2CA0_R {
        P2CA0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca1(&self) -> P2CA1_R {
        P2CA1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca2(&self) -> P2CA2_R {
        P2CA2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca3(&self) -> P2CA3_R {
        P2CA3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca4(&self) -> P2CA4_R {
        P2CA4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&self) -> CASHORT_R {
        CASHORT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    #[must_use]
    pub fn caout(&mut self) -> CAOUT_W<0> {
        CAOUT_W::new(self)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    #[must_use]
    pub fn caf(&mut self) -> CAF_W<1> {
        CAF_W::new(self)
    }
    #[doc = "Bit 2 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn p2ca0(&mut self) -> P2CA0_W<2> {
        P2CA0_W::new(self)
    }
    #[doc = "Bit 3 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn p2ca1(&mut self) -> P2CA1_W<3> {
        P2CA1_W::new(self)
    }
    #[doc = "Bit 4 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn p2ca2(&mut self) -> P2CA2_W<4> {
        P2CA2_W::new(self)
    }
    #[doc = "Bit 5 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn p2ca3(&mut self) -> P2CA3_W<5> {
        P2CA3_W::new(self)
    }
    #[doc = "Bit 6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn p2ca4(&mut self) -> P2CA4_W<6> {
        P2CA4_W::new(self)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    #[must_use]
    pub fn cashort(&mut self) -> CASHORT_W<7> {
        CASHORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Comparator A Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cactl2](index.html) module"]
pub struct CACTL2_SPEC;
impl crate::RegisterSpec for CACTL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cactl2::R](R) reader structure"]
impl crate::Readable for CACTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cactl2::W](W) writer structure"]
impl crate::Writable for CACTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACTL2 to value 0"]
impl crate::Resettable for CACTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
