#[doc = "Register `SVSCTL` reader"]
pub struct R(crate::R<SVSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVSCTL` writer"]
pub struct W(crate::W<SVSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVSCTL_SPEC>;
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
impl From<crate::W<SVSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVSFG` reader - SVS Flag"]
pub type SVSFG_R = crate::BitReader<bool>;
#[doc = "Field `SVSFG` writer - SVS Flag"]
pub type SVSFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, SVSCTL_SPEC, bool, O>;
#[doc = "Field `SVSOP` reader - SVS output (read only)"]
pub type SVSOP_R = crate::BitReader<bool>;
#[doc = "Field `SVSOP` writer - SVS output (read only)"]
pub type SVSOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, SVSCTL_SPEC, bool, O>;
#[doc = "Field `SVSON` reader - Switches the SVS on/off"]
pub type SVSON_R = crate::BitReader<bool>;
#[doc = "Field `SVSON` writer - Switches the SVS on/off"]
pub type SVSON_W<'a, const O: u8> = crate::BitWriter<'a, u8, SVSCTL_SPEC, bool, O>;
#[doc = "Field `PORON` reader - Enable POR Generation if Low Voltage"]
pub type PORON_R = crate::BitReader<bool>;
#[doc = "Field `PORON` writer - Enable POR Generation if Low Voltage"]
pub type PORON_W<'a, const O: u8> = crate::BitWriter<'a, u8, SVSCTL_SPEC, bool, O>;
#[doc = "Field `VLD0` reader - VLD0"]
pub type VLD0_R = crate::BitReader<bool>;
#[doc = "Field `VLD0` writer - VLD0"]
pub type VLD0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SVSCTL_SPEC, bool, O>;
#[doc = "Field `VLD1` reader - VLD1"]
pub type VLD1_R = crate::BitReader<bool>;
#[doc = "Field `VLD1` writer - VLD1"]
pub type VLD1_W<'a, const O: u8> = crate::BitWriter<'a, u8, SVSCTL_SPEC, bool, O>;
#[doc = "Field `VLD2` reader - VLD2"]
pub type VLD2_R = crate::BitReader<bool>;
#[doc = "Field `VLD2` writer - VLD2"]
pub type VLD2_W<'a, const O: u8> = crate::BitWriter<'a, u8, SVSCTL_SPEC, bool, O>;
#[doc = "Field `VLD3` reader - VLD3"]
pub type VLD3_R = crate::BitReader<bool>;
#[doc = "Field `VLD3` writer - VLD3"]
pub type VLD3_W<'a, const O: u8> = crate::BitWriter<'a, u8, SVSCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SVS Flag"]
    #[inline(always)]
    pub fn svsfg(&self) -> SVSFG_R {
        SVSFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SVS output (read only)"]
    #[inline(always)]
    pub fn svsop(&self) -> SVSOP_R {
        SVSOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Switches the SVS on/off"]
    #[inline(always)]
    pub fn svson(&self) -> SVSON_R {
        SVSON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable POR Generation if Low Voltage"]
    #[inline(always)]
    pub fn poron(&self) -> PORON_R {
        PORON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VLD0"]
    #[inline(always)]
    pub fn vld0(&self) -> VLD0_R {
        VLD0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VLD1"]
    #[inline(always)]
    pub fn vld1(&self) -> VLD1_R {
        VLD1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VLD2"]
    #[inline(always)]
    pub fn vld2(&self) -> VLD2_R {
        VLD2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VLD3"]
    #[inline(always)]
    pub fn vld3(&self) -> VLD3_R {
        VLD3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SVS Flag"]
    #[inline(always)]
    #[must_use]
    pub fn svsfg(&mut self) -> SVSFG_W<0> {
        SVSFG_W::new(self)
    }
    #[doc = "Bit 1 - SVS output (read only)"]
    #[inline(always)]
    #[must_use]
    pub fn svsop(&mut self) -> SVSOP_W<1> {
        SVSOP_W::new(self)
    }
    #[doc = "Bit 2 - Switches the SVS on/off"]
    #[inline(always)]
    #[must_use]
    pub fn svson(&mut self) -> SVSON_W<2> {
        SVSON_W::new(self)
    }
    #[doc = "Bit 3 - Enable POR Generation if Low Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn poron(&mut self) -> PORON_W<3> {
        PORON_W::new(self)
    }
    #[doc = "Bit 4 - VLD0"]
    #[inline(always)]
    #[must_use]
    pub fn vld0(&mut self) -> VLD0_W<4> {
        VLD0_W::new(self)
    }
    #[doc = "Bit 5 - VLD1"]
    #[inline(always)]
    #[must_use]
    pub fn vld1(&mut self) -> VLD1_W<5> {
        VLD1_W::new(self)
    }
    #[doc = "Bit 6 - VLD2"]
    #[inline(always)]
    #[must_use]
    pub fn vld2(&mut self) -> VLD2_W<6> {
        VLD2_W::new(self)
    }
    #[doc = "Bit 7 - VLD3"]
    #[inline(always)]
    #[must_use]
    pub fn vld3(&mut self) -> VLD3_W<7> {
        VLD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "SVS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svsctl](index.html) module"]
pub struct SVSCTL_SPEC;
impl crate::RegisterSpec for SVSCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [svsctl::R](R) reader structure"]
impl crate::Readable for SVSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svsctl::W](W) writer structure"]
impl crate::Writable for SVSCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVSCTL to value 0"]
impl crate::Resettable for SVSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
