#[doc = "Register `ADC12MCTL3` reader"]
pub struct R(crate::R<ADC12MCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12MCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12MCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12MCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12MCTL3` writer"]
pub struct W(crate::W<ADC12MCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12MCTL3_SPEC>;
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
impl From<crate::W<ADC12MCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12MCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCH` reader - ADC12 Input Channel Select Bit 0"]
pub type INCH_R = crate::FieldReader<u8, INCH_A>;
#[doc = "ADC12 Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INCH_A {
    #[doc = "0: ADC12 Input Channel 0"]
    INCH_0 = 0,
    #[doc = "1: ADC12 Input Channel 1"]
    INCH_1 = 1,
    #[doc = "2: ADC12 Input Channel 2"]
    INCH_2 = 2,
    #[doc = "3: ADC12 Input Channel 3"]
    INCH_3 = 3,
    #[doc = "4: ADC12 Input Channel 4"]
    INCH_4 = 4,
    #[doc = "5: ADC12 Input Channel 5"]
    INCH_5 = 5,
    #[doc = "6: ADC12 Input Channel 6"]
    INCH_6 = 6,
    #[doc = "7: ADC12 Input Channel 7"]
    INCH_7 = 7,
    #[doc = "8: ADC12 Input Channel 8"]
    INCH_8 = 8,
    #[doc = "9: ADC12 Input Channel 9"]
    INCH_9 = 9,
    #[doc = "10: ADC12 Input Channel 10"]
    INCH_10 = 10,
    #[doc = "11: ADC12 Input Channel 11"]
    INCH_11 = 11,
    #[doc = "12: ADC12 Input Channel 12"]
    INCH_12 = 12,
    #[doc = "13: ADC12 Input Channel 13"]
    INCH_13 = 13,
    #[doc = "14: ADC12 Input Channel 14"]
    INCH_14 = 14,
    #[doc = "15: ADC12 Input Channel 15"]
    INCH_15 = 15,
}
impl From<INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: INCH_A) -> Self {
        variant as _
    }
}
impl INCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INCH_A {
        match self.bits {
            0 => INCH_A::INCH_0,
            1 => INCH_A::INCH_1,
            2 => INCH_A::INCH_2,
            3 => INCH_A::INCH_3,
            4 => INCH_A::INCH_4,
            5 => INCH_A::INCH_5,
            6 => INCH_A::INCH_6,
            7 => INCH_A::INCH_7,
            8 => INCH_A::INCH_8,
            9 => INCH_A::INCH_9,
            10 => INCH_A::INCH_10,
            11 => INCH_A::INCH_11,
            12 => INCH_A::INCH_12,
            13 => INCH_A::INCH_13,
            14 => INCH_A::INCH_14,
            15 => INCH_A::INCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INCH_0`"]
    #[inline(always)]
    pub fn is_inch_0(&self) -> bool {
        *self == INCH_A::INCH_0
    }
    #[doc = "Checks if the value of the field is `INCH_1`"]
    #[inline(always)]
    pub fn is_inch_1(&self) -> bool {
        *self == INCH_A::INCH_1
    }
    #[doc = "Checks if the value of the field is `INCH_2`"]
    #[inline(always)]
    pub fn is_inch_2(&self) -> bool {
        *self == INCH_A::INCH_2
    }
    #[doc = "Checks if the value of the field is `INCH_3`"]
    #[inline(always)]
    pub fn is_inch_3(&self) -> bool {
        *self == INCH_A::INCH_3
    }
    #[doc = "Checks if the value of the field is `INCH_4`"]
    #[inline(always)]
    pub fn is_inch_4(&self) -> bool {
        *self == INCH_A::INCH_4
    }
    #[doc = "Checks if the value of the field is `INCH_5`"]
    #[inline(always)]
    pub fn is_inch_5(&self) -> bool {
        *self == INCH_A::INCH_5
    }
    #[doc = "Checks if the value of the field is `INCH_6`"]
    #[inline(always)]
    pub fn is_inch_6(&self) -> bool {
        *self == INCH_A::INCH_6
    }
    #[doc = "Checks if the value of the field is `INCH_7`"]
    #[inline(always)]
    pub fn is_inch_7(&self) -> bool {
        *self == INCH_A::INCH_7
    }
    #[doc = "Checks if the value of the field is `INCH_8`"]
    #[inline(always)]
    pub fn is_inch_8(&self) -> bool {
        *self == INCH_A::INCH_8
    }
    #[doc = "Checks if the value of the field is `INCH_9`"]
    #[inline(always)]
    pub fn is_inch_9(&self) -> bool {
        *self == INCH_A::INCH_9
    }
    #[doc = "Checks if the value of the field is `INCH_10`"]
    #[inline(always)]
    pub fn is_inch_10(&self) -> bool {
        *self == INCH_A::INCH_10
    }
    #[doc = "Checks if the value of the field is `INCH_11`"]
    #[inline(always)]
    pub fn is_inch_11(&self) -> bool {
        *self == INCH_A::INCH_11
    }
    #[doc = "Checks if the value of the field is `INCH_12`"]
    #[inline(always)]
    pub fn is_inch_12(&self) -> bool {
        *self == INCH_A::INCH_12
    }
    #[doc = "Checks if the value of the field is `INCH_13`"]
    #[inline(always)]
    pub fn is_inch_13(&self) -> bool {
        *self == INCH_A::INCH_13
    }
    #[doc = "Checks if the value of the field is `INCH_14`"]
    #[inline(always)]
    pub fn is_inch_14(&self) -> bool {
        *self == INCH_A::INCH_14
    }
    #[doc = "Checks if the value of the field is `INCH_15`"]
    #[inline(always)]
    pub fn is_inch_15(&self) -> bool {
        *self == INCH_A::INCH_15
    }
}
#[doc = "Field `INCH` writer - ADC12 Input Channel Select Bit 0"]
pub type INCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, ADC12MCTL3_SPEC, u8, INCH_A, 4, O>;
impl<'a, const O: u8> INCH_W<'a, O> {
    #[doc = "ADC12 Input Channel 0"]
    #[inline(always)]
    pub fn inch_0(self) -> &'a mut W {
        self.variant(INCH_A::INCH_0)
    }
    #[doc = "ADC12 Input Channel 1"]
    #[inline(always)]
    pub fn inch_1(self) -> &'a mut W {
        self.variant(INCH_A::INCH_1)
    }
    #[doc = "ADC12 Input Channel 2"]
    #[inline(always)]
    pub fn inch_2(self) -> &'a mut W {
        self.variant(INCH_A::INCH_2)
    }
    #[doc = "ADC12 Input Channel 3"]
    #[inline(always)]
    pub fn inch_3(self) -> &'a mut W {
        self.variant(INCH_A::INCH_3)
    }
    #[doc = "ADC12 Input Channel 4"]
    #[inline(always)]
    pub fn inch_4(self) -> &'a mut W {
        self.variant(INCH_A::INCH_4)
    }
    #[doc = "ADC12 Input Channel 5"]
    #[inline(always)]
    pub fn inch_5(self) -> &'a mut W {
        self.variant(INCH_A::INCH_5)
    }
    #[doc = "ADC12 Input Channel 6"]
    #[inline(always)]
    pub fn inch_6(self) -> &'a mut W {
        self.variant(INCH_A::INCH_6)
    }
    #[doc = "ADC12 Input Channel 7"]
    #[inline(always)]
    pub fn inch_7(self) -> &'a mut W {
        self.variant(INCH_A::INCH_7)
    }
    #[doc = "ADC12 Input Channel 8"]
    #[inline(always)]
    pub fn inch_8(self) -> &'a mut W {
        self.variant(INCH_A::INCH_8)
    }
    #[doc = "ADC12 Input Channel 9"]
    #[inline(always)]
    pub fn inch_9(self) -> &'a mut W {
        self.variant(INCH_A::INCH_9)
    }
    #[doc = "ADC12 Input Channel 10"]
    #[inline(always)]
    pub fn inch_10(self) -> &'a mut W {
        self.variant(INCH_A::INCH_10)
    }
    #[doc = "ADC12 Input Channel 11"]
    #[inline(always)]
    pub fn inch_11(self) -> &'a mut W {
        self.variant(INCH_A::INCH_11)
    }
    #[doc = "ADC12 Input Channel 12"]
    #[inline(always)]
    pub fn inch_12(self) -> &'a mut W {
        self.variant(INCH_A::INCH_12)
    }
    #[doc = "ADC12 Input Channel 13"]
    #[inline(always)]
    pub fn inch_13(self) -> &'a mut W {
        self.variant(INCH_A::INCH_13)
    }
    #[doc = "ADC12 Input Channel 14"]
    #[inline(always)]
    pub fn inch_14(self) -> &'a mut W {
        self.variant(INCH_A::INCH_14)
    }
    #[doc = "ADC12 Input Channel 15"]
    #[inline(always)]
    pub fn inch_15(self) -> &'a mut W {
        self.variant(INCH_A::INCH_15)
    }
}
#[doc = "Field `SREF` reader - ADC12 Select Reference Bit 0"]
pub type SREF_R = crate::FieldReader<u8, SREF_A>;
#[doc = "ADC12 Select Reference Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SREF_A {
    #[doc = "0: ADC12 Select Reference 0"]
    SREF_0 = 0,
    #[doc = "1: ADC12 Select Reference 1"]
    SREF_1 = 1,
    #[doc = "2: ADC12 Select Reference 2"]
    SREF_2 = 2,
    #[doc = "3: ADC12 Select Reference 3"]
    SREF_3 = 3,
    #[doc = "4: ADC12 Select Reference 4"]
    SREF_4 = 4,
    #[doc = "5: ADC12 Select Reference 5"]
    SREF_5 = 5,
    #[doc = "6: ADC12 Select Reference 6"]
    SREF_6 = 6,
    #[doc = "7: ADC12 Select Reference 7"]
    SREF_7 = 7,
}
impl From<SREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SREF_A) -> Self {
        variant as _
    }
}
impl SREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREF_A {
        match self.bits {
            0 => SREF_A::SREF_0,
            1 => SREF_A::SREF_1,
            2 => SREF_A::SREF_2,
            3 => SREF_A::SREF_3,
            4 => SREF_A::SREF_4,
            5 => SREF_A::SREF_5,
            6 => SREF_A::SREF_6,
            7 => SREF_A::SREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SREF_0`"]
    #[inline(always)]
    pub fn is_sref_0(&self) -> bool {
        *self == SREF_A::SREF_0
    }
    #[doc = "Checks if the value of the field is `SREF_1`"]
    #[inline(always)]
    pub fn is_sref_1(&self) -> bool {
        *self == SREF_A::SREF_1
    }
    #[doc = "Checks if the value of the field is `SREF_2`"]
    #[inline(always)]
    pub fn is_sref_2(&self) -> bool {
        *self == SREF_A::SREF_2
    }
    #[doc = "Checks if the value of the field is `SREF_3`"]
    #[inline(always)]
    pub fn is_sref_3(&self) -> bool {
        *self == SREF_A::SREF_3
    }
    #[doc = "Checks if the value of the field is `SREF_4`"]
    #[inline(always)]
    pub fn is_sref_4(&self) -> bool {
        *self == SREF_A::SREF_4
    }
    #[doc = "Checks if the value of the field is `SREF_5`"]
    #[inline(always)]
    pub fn is_sref_5(&self) -> bool {
        *self == SREF_A::SREF_5
    }
    #[doc = "Checks if the value of the field is `SREF_6`"]
    #[inline(always)]
    pub fn is_sref_6(&self) -> bool {
        *self == SREF_A::SREF_6
    }
    #[doc = "Checks if the value of the field is `SREF_7`"]
    #[inline(always)]
    pub fn is_sref_7(&self) -> bool {
        *self == SREF_A::SREF_7
    }
}
#[doc = "Field `SREF` writer - ADC12 Select Reference Bit 0"]
pub type SREF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, ADC12MCTL3_SPEC, u8, SREF_A, 3, O>;
impl<'a, const O: u8> SREF_W<'a, O> {
    #[doc = "ADC12 Select Reference 0"]
    #[inline(always)]
    pub fn sref_0(self) -> &'a mut W {
        self.variant(SREF_A::SREF_0)
    }
    #[doc = "ADC12 Select Reference 1"]
    #[inline(always)]
    pub fn sref_1(self) -> &'a mut W {
        self.variant(SREF_A::SREF_1)
    }
    #[doc = "ADC12 Select Reference 2"]
    #[inline(always)]
    pub fn sref_2(self) -> &'a mut W {
        self.variant(SREF_A::SREF_2)
    }
    #[doc = "ADC12 Select Reference 3"]
    #[inline(always)]
    pub fn sref_3(self) -> &'a mut W {
        self.variant(SREF_A::SREF_3)
    }
    #[doc = "ADC12 Select Reference 4"]
    #[inline(always)]
    pub fn sref_4(self) -> &'a mut W {
        self.variant(SREF_A::SREF_4)
    }
    #[doc = "ADC12 Select Reference 5"]
    #[inline(always)]
    pub fn sref_5(self) -> &'a mut W {
        self.variant(SREF_A::SREF_5)
    }
    #[doc = "ADC12 Select Reference 6"]
    #[inline(always)]
    pub fn sref_6(self) -> &'a mut W {
        self.variant(SREF_A::SREF_6)
    }
    #[doc = "ADC12 Select Reference 7"]
    #[inline(always)]
    pub fn sref_7(self) -> &'a mut W {
        self.variant(SREF_A::SREF_7)
    }
}
#[doc = "Field `EOS` reader - ADC12 End of Sequence"]
pub type EOS_R = crate::BitReader<bool>;
#[doc = "Field `EOS` writer - ADC12 End of Sequence"]
pub type EOS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADC12MCTL3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn inch(&self) -> INCH_R {
        INCH_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - ADC12 Select Reference Bit 0"]
    #[inline(always)]
    pub fn sref(&self) -> SREF_R {
        SREF_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn inch(&mut self) -> INCH_W<0> {
        INCH_W::new(self)
    }
    #[doc = "Bits 4:6 - ADC12 Select Reference Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sref(&mut self) -> SREF_W<4> {
        SREF_W::new(self)
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<7> {
        EOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12 Memory Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl3](index.html) module"]
pub struct ADC12MCTL3_SPEC;
impl crate::RegisterSpec for ADC12MCTL3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adc12mctl3::R](R) reader structure"]
impl crate::Readable for ADC12MCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12mctl3::W](W) writer structure"]
impl crate::Writable for ADC12MCTL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12MCTL3 to value 0"]
impl crate::Resettable for ADC12MCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
