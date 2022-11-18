#[doc = "Register `FCTL2` reader"]
pub struct R(crate::R<FCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTL2` writer"]
pub struct W(crate::W<FCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTL2_SPEC>;
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
impl From<crate::W<FCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FN0` reader - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
pub type FN0_R = crate::BitReader<bool>;
#[doc = "Field `FN0` writer - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
pub type FN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL2_SPEC, bool, O>;
#[doc = "Field `FN1` reader - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
pub type FN1_R = crate::BitReader<bool>;
#[doc = "Field `FN1` writer - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
pub type FN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL2_SPEC, bool, O>;
#[doc = "Field `FN2` reader - FN2"]
pub type FN2_R = crate::BitReader<bool>;
#[doc = "Field `FN2` writer - FN2"]
pub type FN2_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL2_SPEC, bool, O>;
#[doc = "Field `FN3` reader - FN3"]
pub type FN3_R = crate::BitReader<bool>;
#[doc = "Field `FN3` writer - FN3"]
pub type FN3_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL2_SPEC, bool, O>;
#[doc = "Field `FN4` reader - FN4"]
pub type FN4_R = crate::BitReader<bool>;
#[doc = "Field `FN4` writer - FN4"]
pub type FN4_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL2_SPEC, bool, O>;
#[doc = "Field `FN5` reader - FN5"]
pub type FN5_R = crate::BitReader<bool>;
#[doc = "Field `FN5` writer - FN5"]
pub type FN5_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCTL2_SPEC, bool, O>;
#[doc = "Field `FSSEL` reader - Flash clock select 0 */ /* to distinguish from USART SSELx"]
pub type FSSEL_R = crate::FieldReader<u8, FSSEL_A>;
#[doc = "Flash clock select 0 */ /* to distinguish from USART SSELx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSSEL_A {
    #[doc = "0: Flash clock select: 0 - ACLK"]
    FSSEL_0 = 0,
    #[doc = "1: Flash clock select: 1 - MCLK"]
    FSSEL_1 = 1,
    #[doc = "2: Flash clock select: 2 - SMCLK"]
    FSSEL_2 = 2,
    #[doc = "3: Flash clock select: 3 - SMCLK"]
    FSSEL_3 = 3,
}
impl From<FSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSSEL_A) -> Self {
        variant as _
    }
}
impl FSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSSEL_A {
        match self.bits {
            0 => FSSEL_A::FSSEL_0,
            1 => FSSEL_A::FSSEL_1,
            2 => FSSEL_A::FSSEL_2,
            3 => FSSEL_A::FSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FSSEL_0`"]
    #[inline(always)]
    pub fn is_fssel_0(&self) -> bool {
        *self == FSSEL_A::FSSEL_0
    }
    #[doc = "Checks if the value of the field is `FSSEL_1`"]
    #[inline(always)]
    pub fn is_fssel_1(&self) -> bool {
        *self == FSSEL_A::FSSEL_1
    }
    #[doc = "Checks if the value of the field is `FSSEL_2`"]
    #[inline(always)]
    pub fn is_fssel_2(&self) -> bool {
        *self == FSSEL_A::FSSEL_2
    }
    #[doc = "Checks if the value of the field is `FSSEL_3`"]
    #[inline(always)]
    pub fn is_fssel_3(&self) -> bool {
        *self == FSSEL_A::FSSEL_3
    }
}
#[doc = "Field `FSSEL` writer - Flash clock select 0 */ /* to distinguish from USART SSELx"]
pub type FSSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, FCTL2_SPEC, u8, FSSEL_A, 2, O>;
impl<'a, const O: u8> FSSEL_W<'a, O> {
    #[doc = "Flash clock select: 0 - ACLK"]
    #[inline(always)]
    pub fn fssel_0(self) -> &'a mut W {
        self.variant(FSSEL_A::FSSEL_0)
    }
    #[doc = "Flash clock select: 1 - MCLK"]
    #[inline(always)]
    pub fn fssel_1(self) -> &'a mut W {
        self.variant(FSSEL_A::FSSEL_1)
    }
    #[doc = "Flash clock select: 2 - SMCLK"]
    #[inline(always)]
    pub fn fssel_2(self) -> &'a mut W {
        self.variant(FSSEL_A::FSSEL_2)
    }
    #[doc = "Flash clock select: 3 - SMCLK"]
    #[inline(always)]
    pub fn fssel_3(self) -> &'a mut W {
        self.variant(FSSEL_A::FSSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn0(&self) -> FN0_R {
        FN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
    #[inline(always)]
    pub fn fn1(&self) -> FN1_R {
        FN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FN2"]
    #[inline(always)]
    pub fn fn2(&self) -> FN2_R {
        FN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FN3"]
    #[inline(always)]
    pub fn fn3(&self) -> FN3_R {
        FN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FN4"]
    #[inline(always)]
    pub fn fn4(&self) -> FN4_R {
        FN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FN5"]
    #[inline(always)]
    pub fn fn5(&self) -> FN5_R {
        FN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&self) -> FSSEL_R {
        FSSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    #[must_use]
    pub fn fn0(&mut self) -> FN0_W<0> {
        FN0_W::new(self)
    }
    #[doc = "Bit 1 - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
    #[inline(always)]
    #[must_use]
    pub fn fn1(&mut self) -> FN1_W<1> {
        FN1_W::new(self)
    }
    #[doc = "Bit 2 - FN2"]
    #[inline(always)]
    #[must_use]
    pub fn fn2(&mut self) -> FN2_W<2> {
        FN2_W::new(self)
    }
    #[doc = "Bit 3 - FN3"]
    #[inline(always)]
    #[must_use]
    pub fn fn3(&mut self) -> FN3_W<3> {
        FN3_W::new(self)
    }
    #[doc = "Bit 4 - FN4"]
    #[inline(always)]
    #[must_use]
    pub fn fn4(&mut self) -> FN4_W<4> {
        FN4_W::new(self)
    }
    #[doc = "Bit 5 - FN5"]
    #[inline(always)]
    #[must_use]
    pub fn fn5(&mut self) -> FN5_W<5> {
        FN5_W::new(self)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    #[must_use]
    pub fn fssel(&mut self) -> FSSEL_W<6> {
        FSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl2](index.html) module"]
pub struct FCTL2_SPEC;
impl crate::RegisterSpec for FCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fctl2::R](R) reader structure"]
impl crate::Readable for FCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctl2::W](W) writer structure"]
impl crate::Writable for FCTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTL2 to value 0"]
impl crate::Resettable for FCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
