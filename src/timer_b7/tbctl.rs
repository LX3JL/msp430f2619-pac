#[doc = "Register `TBCTL` reader"]
pub struct R(crate::R<TBCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBCTL` writer"]
pub struct W(crate::W<TBCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBCTL_SPEC>;
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
impl From<crate::W<TBCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBIFG` reader - Timer B interrupt flag"]
pub type TBIFG_R = crate::BitReader<bool>;
#[doc = "Field `TBIFG` writer - Timer B interrupt flag"]
pub type TBIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TBCTL_SPEC, bool, O>;
#[doc = "Field `TBIE` reader - Timer B interrupt enable"]
pub type TBIE_R = crate::BitReader<bool>;
#[doc = "Field `TBIE` writer - Timer B interrupt enable"]
pub type TBIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TBCTL_SPEC, bool, O>;
#[doc = "Field `TBCLR` reader - Timer B counter clear"]
pub type TBCLR_R = crate::BitReader<bool>;
#[doc = "Field `TBCLR` writer - Timer B counter clear"]
pub type TBCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, TBCTL_SPEC, bool, O>;
#[doc = "Field `MC` reader - Timer B mode control 1"]
pub type MC_R = crate::FieldReader<u8, MC_A>;
#[doc = "Timer B mode control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Timer A mode control: 0 - Stop"]
    MC_0 = 0,
    #[doc = "1: Timer A mode control: 1 - Up to CCR0"]
    MC_1 = 1,
    #[doc = "2: Timer A mode control: 2 - Continous up"]
    MC_2 = 2,
    #[doc = "3: Timer A mode control: 3 - Up/Down"]
    MC_3 = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
impl MC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::MC_0,
            1 => MC_A::MC_1,
            2 => MC_A::MC_2,
            3 => MC_A::MC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MC_0`"]
    #[inline(always)]
    pub fn is_mc_0(&self) -> bool {
        *self == MC_A::MC_0
    }
    #[doc = "Checks if the value of the field is `MC_1`"]
    #[inline(always)]
    pub fn is_mc_1(&self) -> bool {
        *self == MC_A::MC_1
    }
    #[doc = "Checks if the value of the field is `MC_2`"]
    #[inline(always)]
    pub fn is_mc_2(&self) -> bool {
        *self == MC_A::MC_2
    }
    #[doc = "Checks if the value of the field is `MC_3`"]
    #[inline(always)]
    pub fn is_mc_3(&self) -> bool {
        *self == MC_A::MC_3
    }
}
#[doc = "Field `MC` writer - Timer B mode control 1"]
pub type MC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TBCTL_SPEC, u8, MC_A, 2, O>;
impl<'a, const O: u8> MC_W<'a, O> {
    #[doc = "Timer A mode control: 0 - Stop"]
    #[inline(always)]
    pub fn mc_0(self) -> &'a mut W {
        self.variant(MC_A::MC_0)
    }
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    #[inline(always)]
    pub fn mc_1(self) -> &'a mut W {
        self.variant(MC_A::MC_1)
    }
    #[doc = "Timer A mode control: 2 - Continous up"]
    #[inline(always)]
    pub fn mc_2(self) -> &'a mut W {
        self.variant(MC_A::MC_2)
    }
    #[doc = "Timer A mode control: 3 - Up/Down"]
    #[inline(always)]
    pub fn mc_3(self) -> &'a mut W {
        self.variant(MC_A::MC_3)
    }
}
#[doc = "Field `ID` reader - Timer B clock input divider 1"]
pub type ID_R = crate::FieldReader<u8, ID_A>;
#[doc = "Timer B clock input divider 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: Timer A input divider: 0 - /1"]
    ID_0 = 0,
    #[doc = "1: Timer A input divider: 1 - /2"]
    ID_1 = 1,
    #[doc = "2: Timer A input divider: 2 - /4"]
    ID_2 = 2,
    #[doc = "3: Timer A input divider: 3 - /8"]
    ID_3 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::ID_0,
            1 => ID_A::ID_1,
            2 => ID_A::ID_2,
            3 => ID_A::ID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ID_0`"]
    #[inline(always)]
    pub fn is_id_0(&self) -> bool {
        *self == ID_A::ID_0
    }
    #[doc = "Checks if the value of the field is `ID_1`"]
    #[inline(always)]
    pub fn is_id_1(&self) -> bool {
        *self == ID_A::ID_1
    }
    #[doc = "Checks if the value of the field is `ID_2`"]
    #[inline(always)]
    pub fn is_id_2(&self) -> bool {
        *self == ID_A::ID_2
    }
    #[doc = "Checks if the value of the field is `ID_3`"]
    #[inline(always)]
    pub fn is_id_3(&self) -> bool {
        *self == ID_A::ID_3
    }
}
#[doc = "Field `ID` writer - Timer B clock input divider 1"]
pub type ID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TBCTL_SPEC, u8, ID_A, 2, O>;
impl<'a, const O: u8> ID_W<'a, O> {
    #[doc = "Timer A input divider: 0 - /1"]
    #[inline(always)]
    pub fn id_0(self) -> &'a mut W {
        self.variant(ID_A::ID_0)
    }
    #[doc = "Timer A input divider: 1 - /2"]
    #[inline(always)]
    pub fn id_1(self) -> &'a mut W {
        self.variant(ID_A::ID_1)
    }
    #[doc = "Timer A input divider: 2 - /4"]
    #[inline(always)]
    pub fn id_2(self) -> &'a mut W {
        self.variant(ID_A::ID_2)
    }
    #[doc = "Timer A input divider: 3 - /8"]
    #[inline(always)]
    pub fn id_3(self) -> &'a mut W {
        self.variant(ID_A::ID_3)
    }
}
#[doc = "Field `TBSSEL` reader - Clock source 1"]
pub type TBSSEL_R = crate::FieldReader<u8, TBSSEL_A>;
#[doc = "Clock source 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TBSSEL_A {
    #[doc = "0: Clock Source: TBCLK"]
    TBSSEL_0 = 0,
    #[doc = "1: Clock Source: ACLK"]
    TBSSEL_1 = 1,
    #[doc = "2: Clock Source: SMCLK"]
    TBSSEL_2 = 2,
    #[doc = "3: Clock Source: INCLK"]
    TBSSEL_3 = 3,
}
impl From<TBSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TBSSEL_A) -> Self {
        variant as _
    }
}
impl TBSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBSSEL_A {
        match self.bits {
            0 => TBSSEL_A::TBSSEL_0,
            1 => TBSSEL_A::TBSSEL_1,
            2 => TBSSEL_A::TBSSEL_2,
            3 => TBSSEL_A::TBSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TBSSEL_0`"]
    #[inline(always)]
    pub fn is_tbssel_0(&self) -> bool {
        *self == TBSSEL_A::TBSSEL_0
    }
    #[doc = "Checks if the value of the field is `TBSSEL_1`"]
    #[inline(always)]
    pub fn is_tbssel_1(&self) -> bool {
        *self == TBSSEL_A::TBSSEL_1
    }
    #[doc = "Checks if the value of the field is `TBSSEL_2`"]
    #[inline(always)]
    pub fn is_tbssel_2(&self) -> bool {
        *self == TBSSEL_A::TBSSEL_2
    }
    #[doc = "Checks if the value of the field is `TBSSEL_3`"]
    #[inline(always)]
    pub fn is_tbssel_3(&self) -> bool {
        *self == TBSSEL_A::TBSSEL_3
    }
}
#[doc = "Field `TBSSEL` writer - Clock source 1"]
pub type TBSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TBCTL_SPEC, u8, TBSSEL_A, 2, O>;
impl<'a, const O: u8> TBSSEL_W<'a, O> {
    #[doc = "Clock Source: TBCLK"]
    #[inline(always)]
    pub fn tbssel_0(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBSSEL_0)
    }
    #[doc = "Clock Source: ACLK"]
    #[inline(always)]
    pub fn tbssel_1(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBSSEL_1)
    }
    #[doc = "Clock Source: SMCLK"]
    #[inline(always)]
    pub fn tbssel_2(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBSSEL_2)
    }
    #[doc = "Clock Source: INCLK"]
    #[inline(always)]
    pub fn tbssel_3(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBSSEL_3)
    }
}
#[doc = "Field `CNTL` reader - Counter lenght 1"]
pub type CNTL_R = crate::FieldReader<u8, CNTL_A>;
#[doc = "Counter lenght 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTL_A {
    #[doc = "0: Counter lenght: 16 bit"]
    CNTL_0 = 0,
    #[doc = "1: Counter lenght: 12 bit"]
    CNTL_1 = 1,
    #[doc = "2: Counter lenght: 10 bit"]
    CNTL_2 = 2,
    #[doc = "3: Counter lenght: 8 bit"]
    CNTL_3 = 3,
}
impl From<CNTL_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTL_A) -> Self {
        variant as _
    }
}
impl CNTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTL_A {
        match self.bits {
            0 => CNTL_A::CNTL_0,
            1 => CNTL_A::CNTL_1,
            2 => CNTL_A::CNTL_2,
            3 => CNTL_A::CNTL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CNTL_0`"]
    #[inline(always)]
    pub fn is_cntl_0(&self) -> bool {
        *self == CNTL_A::CNTL_0
    }
    #[doc = "Checks if the value of the field is `CNTL_1`"]
    #[inline(always)]
    pub fn is_cntl_1(&self) -> bool {
        *self == CNTL_A::CNTL_1
    }
    #[doc = "Checks if the value of the field is `CNTL_2`"]
    #[inline(always)]
    pub fn is_cntl_2(&self) -> bool {
        *self == CNTL_A::CNTL_2
    }
    #[doc = "Checks if the value of the field is `CNTL_3`"]
    #[inline(always)]
    pub fn is_cntl_3(&self) -> bool {
        *self == CNTL_A::CNTL_3
    }
}
#[doc = "Field `CNTL` writer - Counter lenght 1"]
pub type CNTL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TBCTL_SPEC, u8, CNTL_A, 2, O>;
impl<'a, const O: u8> CNTL_W<'a, O> {
    #[doc = "Counter lenght: 16 bit"]
    #[inline(always)]
    pub fn cntl_0(self) -> &'a mut W {
        self.variant(CNTL_A::CNTL_0)
    }
    #[doc = "Counter lenght: 12 bit"]
    #[inline(always)]
    pub fn cntl_1(self) -> &'a mut W {
        self.variant(CNTL_A::CNTL_1)
    }
    #[doc = "Counter lenght: 10 bit"]
    #[inline(always)]
    pub fn cntl_2(self) -> &'a mut W {
        self.variant(CNTL_A::CNTL_2)
    }
    #[doc = "Counter lenght: 8 bit"]
    #[inline(always)]
    pub fn cntl_3(self) -> &'a mut W {
        self.variant(CNTL_A::CNTL_3)
    }
}
#[doc = "Field `TBCLGRP` reader - Timer B Compare latch load group 1"]
pub type TBCLGRP_R = crate::FieldReader<u8, TBCLGRP_A>;
#[doc = "Timer B Compare latch load group 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TBCLGRP_A {
    #[doc = "0: Timer B Group: 0 - individually"]
    TBCLGRP_0 = 0,
    #[doc = "1: Timer B Group: 1 - 3 groups (1-2"]
    TBCLGRP_1 = 1,
    #[doc = "2: Timer B Group: 2 - 2 groups (1-3"]
    TBCLGRP_2 = 2,
    #[doc = "3: Timer B Group: 3 - 1 group (all)"]
    TBCLGRP_3 = 3,
}
impl From<TBCLGRP_A> for u8 {
    #[inline(always)]
    fn from(variant: TBCLGRP_A) -> Self {
        variant as _
    }
}
impl TBCLGRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCLGRP_A {
        match self.bits {
            0 => TBCLGRP_A::TBCLGRP_0,
            1 => TBCLGRP_A::TBCLGRP_1,
            2 => TBCLGRP_A::TBCLGRP_2,
            3 => TBCLGRP_A::TBCLGRP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_0`"]
    #[inline(always)]
    pub fn is_tbclgrp_0(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_0
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_1`"]
    #[inline(always)]
    pub fn is_tbclgrp_1(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_1
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_2`"]
    #[inline(always)]
    pub fn is_tbclgrp_2(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_2
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_3`"]
    #[inline(always)]
    pub fn is_tbclgrp_3(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_3
    }
}
#[doc = "Field `TBCLGRP` writer - Timer B Compare latch load group 1"]
pub type TBCLGRP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TBCTL_SPEC, u8, TBCLGRP_A, 2, O>;
impl<'a, const O: u8> TBCLGRP_W<'a, O> {
    #[doc = "Timer B Group: 0 - individually"]
    #[inline(always)]
    pub fn tbclgrp_0(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_0)
    }
    #[doc = "Timer B Group: 1 - 3 groups (1-2"]
    #[inline(always)]
    pub fn tbclgrp_1(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_1)
    }
    #[doc = "Timer B Group: 2 - 2 groups (1-3"]
    #[inline(always)]
    pub fn tbclgrp_2(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_2)
    }
    #[doc = "Timer B Group: 3 - 1 group (all)"]
    #[inline(always)]
    pub fn tbclgrp_3(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_3)
    }
}
impl R {
    #[doc = "Bit 0 - Timer B interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&self) -> TBIFG_R {
        TBIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer B interrupt enable"]
    #[inline(always)]
    pub fn tbie(&self) -> TBIE_R {
        TBIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B counter clear"]
    #[inline(always)]
    pub fn tbclr(&self) -> TBCLR_R {
        TBCLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer B mode control 1"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Timer B clock input divider 1"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Clock source 1"]
    #[inline(always)]
    pub fn tbssel(&self) -> TBSSEL_R {
        TBSSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Counter lenght 1"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Timer B Compare latch load group 1"]
    #[inline(always)]
    pub fn tbclgrp(&self) -> TBCLGRP_R {
        TBCLGRP_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer B interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbifg(&mut self) -> TBIFG_W<0> {
        TBIFG_W::new(self)
    }
    #[doc = "Bit 1 - Timer B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbie(&mut self) -> TBIE_W<1> {
        TBIE_W::new(self)
    }
    #[doc = "Bit 2 - Timer B counter clear"]
    #[inline(always)]
    #[must_use]
    pub fn tbclr(&mut self) -> TBCLR_W<2> {
        TBCLR_W::new(self)
    }
    #[doc = "Bits 4:5 - Timer B mode control 1"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<4> {
        MC_W::new(self)
    }
    #[doc = "Bits 6:7 - Timer B clock input divider 1"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<6> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock source 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbssel(&mut self) -> TBSSEL_W<8> {
        TBSSEL_W::new(self)
    }
    #[doc = "Bits 11:12 - Counter lenght 1"]
    #[inline(always)]
    #[must_use]
    pub fn cntl(&mut self) -> CNTL_W<11> {
        CNTL_W::new(self)
    }
    #[doc = "Bits 13:14 - Timer B Compare latch load group 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbclgrp(&mut self) -> TBCLGRP_W<13> {
        TBCLGRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbctl](index.html) module"]
pub struct TBCTL_SPEC;
impl crate::RegisterSpec for TBCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tbctl::R](R) reader structure"]
impl crate::Readable for TBCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbctl::W](W) writer structure"]
impl crate::Writable for TBCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBCTL to value 0"]
impl crate::Resettable for TBCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
