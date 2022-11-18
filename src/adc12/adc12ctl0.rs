#[doc = "Register `ADC12CTL0` reader"]
pub struct R(crate::R<ADC12CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12CTL0` writer"]
pub struct W(crate::W<ADC12CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12CTL0_SPEC>;
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
impl From<crate::W<ADC12CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12SC` reader - ADC12 Start Conversion"]
pub type ADC12SC_R = crate::BitReader<bool>;
#[doc = "Field `ADC12SC` writer - ADC12 Start Conversion"]
pub type ADC12SC_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL0_SPEC, bool, O>;
#[doc = "Field `ENC` reader - ADC12 Enable Conversion"]
pub type ENC_R = crate::BitReader<bool>;
#[doc = "Field `ENC` writer - ADC12 Enable Conversion"]
pub type ENC_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL0_SPEC, bool, O>;
#[doc = "Field `ADC12TOVIE` reader - ADC12 Timer Overflow interrupt enable"]
pub type ADC12TOVIE_R = crate::BitReader<bool>;
#[doc = "Field `ADC12TOVIE` writer - ADC12 Timer Overflow interrupt enable"]
pub type ADC12TOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL0_SPEC, bool, O>;
#[doc = "Field `ADC12OVIE` reader - ADC12 Overflow interrupt enable"]
pub type ADC12OVIE_R = crate::BitReader<bool>;
#[doc = "Field `ADC12OVIE` writer - ADC12 Overflow interrupt enable"]
pub type ADC12OVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL0_SPEC, bool, O>;
#[doc = "Field `ADC12ON` reader - ADC12 On/enable"]
pub type ADC12ON_R = crate::BitReader<bool>;
#[doc = "Field `ADC12ON` writer - ADC12 On/enable"]
pub type ADC12ON_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL0_SPEC, bool, O>;
#[doc = "Field `REFON` reader - ADC12 Reference on"]
pub type REFON_R = crate::BitReader<bool>;
#[doc = "Field `REFON` writer - ADC12 Reference on"]
pub type REFON_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL0_SPEC, bool, O>;
#[doc = "Field `REF2_5V` reader - ADC12 Ref 0:1.5V / 1:2.5V"]
pub type REF2_5V_R = crate::BitReader<bool>;
#[doc = "Field `REF2_5V` writer - ADC12 Ref 0:1.5V / 1:2.5V"]
pub type REF2_5V_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL0_SPEC, bool, O>;
#[doc = "Field `MSC` reader - ADC12 Multiple SampleConversion"]
pub type MSC_R = crate::BitReader<bool>;
#[doc = "Field `MSC` writer - ADC12 Multiple SampleConversion"]
pub type MSC_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL0_SPEC, bool, O>;
#[doc = "Field `SHT0` reader - ADC12 Sample Hold 0 Select 0"]
pub type SHT0_R = crate::FieldReader<u8, SHT0_A>;
#[doc = "ADC12 Sample Hold 0 Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHT0_A {
    #[doc = "0: ADC12 Sample Hold 0 Select Bit: 0"]
    SHT0_0 = 0,
    #[doc = "1: ADC12 Sample Hold 0 Select Bit: 1"]
    SHT0_1 = 1,
    #[doc = "2: ADC12 Sample Hold 0 Select Bit: 2"]
    SHT0_2 = 2,
    #[doc = "3: ADC12 Sample Hold 0 Select Bit: 3"]
    SHT0_3 = 3,
    #[doc = "4: ADC12 Sample Hold 0 Select Bit: 4"]
    SHT0_4 = 4,
    #[doc = "5: ADC12 Sample Hold 0 Select Bit: 5"]
    SHT0_5 = 5,
    #[doc = "6: ADC12 Sample Hold 0 Select Bit: 6"]
    SHT0_6 = 6,
    #[doc = "7: ADC12 Sample Hold 0 Select Bit: 7"]
    SHT0_7 = 7,
    #[doc = "8: ADC12 Sample Hold 0 Select Bit: 8"]
    SHT0_8 = 8,
    #[doc = "9: ADC12 Sample Hold 0 Select Bit: 9"]
    SHT0_9 = 9,
    #[doc = "10: ADC12 Sample Hold 0 Select Bit: 10"]
    SHT0_10 = 10,
    #[doc = "11: ADC12 Sample Hold 0 Select Bit: 11"]
    SHT0_11 = 11,
    #[doc = "12: ADC12 Sample Hold 0 Select Bit: 12"]
    SHT0_12 = 12,
    #[doc = "13: ADC12 Sample Hold 0 Select Bit: 13"]
    SHT0_13 = 13,
    #[doc = "14: ADC12 Sample Hold 0 Select Bit: 14"]
    SHT0_14 = 14,
    #[doc = "15: ADC12 Sample Hold 0 Select Bit: 15"]
    SHT0_15 = 15,
}
impl From<SHT0_A> for u8 {
    #[inline(always)]
    fn from(variant: SHT0_A) -> Self {
        variant as _
    }
}
impl SHT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHT0_A {
        match self.bits {
            0 => SHT0_A::SHT0_0,
            1 => SHT0_A::SHT0_1,
            2 => SHT0_A::SHT0_2,
            3 => SHT0_A::SHT0_3,
            4 => SHT0_A::SHT0_4,
            5 => SHT0_A::SHT0_5,
            6 => SHT0_A::SHT0_6,
            7 => SHT0_A::SHT0_7,
            8 => SHT0_A::SHT0_8,
            9 => SHT0_A::SHT0_9,
            10 => SHT0_A::SHT0_10,
            11 => SHT0_A::SHT0_11,
            12 => SHT0_A::SHT0_12,
            13 => SHT0_A::SHT0_13,
            14 => SHT0_A::SHT0_14,
            15 => SHT0_A::SHT0_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SHT0_0`"]
    #[inline(always)]
    pub fn is_sht0_0(&self) -> bool {
        *self == SHT0_A::SHT0_0
    }
    #[doc = "Checks if the value of the field is `SHT0_1`"]
    #[inline(always)]
    pub fn is_sht0_1(&self) -> bool {
        *self == SHT0_A::SHT0_1
    }
    #[doc = "Checks if the value of the field is `SHT0_2`"]
    #[inline(always)]
    pub fn is_sht0_2(&self) -> bool {
        *self == SHT0_A::SHT0_2
    }
    #[doc = "Checks if the value of the field is `SHT0_3`"]
    #[inline(always)]
    pub fn is_sht0_3(&self) -> bool {
        *self == SHT0_A::SHT0_3
    }
    #[doc = "Checks if the value of the field is `SHT0_4`"]
    #[inline(always)]
    pub fn is_sht0_4(&self) -> bool {
        *self == SHT0_A::SHT0_4
    }
    #[doc = "Checks if the value of the field is `SHT0_5`"]
    #[inline(always)]
    pub fn is_sht0_5(&self) -> bool {
        *self == SHT0_A::SHT0_5
    }
    #[doc = "Checks if the value of the field is `SHT0_6`"]
    #[inline(always)]
    pub fn is_sht0_6(&self) -> bool {
        *self == SHT0_A::SHT0_6
    }
    #[doc = "Checks if the value of the field is `SHT0_7`"]
    #[inline(always)]
    pub fn is_sht0_7(&self) -> bool {
        *self == SHT0_A::SHT0_7
    }
    #[doc = "Checks if the value of the field is `SHT0_8`"]
    #[inline(always)]
    pub fn is_sht0_8(&self) -> bool {
        *self == SHT0_A::SHT0_8
    }
    #[doc = "Checks if the value of the field is `SHT0_9`"]
    #[inline(always)]
    pub fn is_sht0_9(&self) -> bool {
        *self == SHT0_A::SHT0_9
    }
    #[doc = "Checks if the value of the field is `SHT0_10`"]
    #[inline(always)]
    pub fn is_sht0_10(&self) -> bool {
        *self == SHT0_A::SHT0_10
    }
    #[doc = "Checks if the value of the field is `SHT0_11`"]
    #[inline(always)]
    pub fn is_sht0_11(&self) -> bool {
        *self == SHT0_A::SHT0_11
    }
    #[doc = "Checks if the value of the field is `SHT0_12`"]
    #[inline(always)]
    pub fn is_sht0_12(&self) -> bool {
        *self == SHT0_A::SHT0_12
    }
    #[doc = "Checks if the value of the field is `SHT0_13`"]
    #[inline(always)]
    pub fn is_sht0_13(&self) -> bool {
        *self == SHT0_A::SHT0_13
    }
    #[doc = "Checks if the value of the field is `SHT0_14`"]
    #[inline(always)]
    pub fn is_sht0_14(&self) -> bool {
        *self == SHT0_A::SHT0_14
    }
    #[doc = "Checks if the value of the field is `SHT0_15`"]
    #[inline(always)]
    pub fn is_sht0_15(&self) -> bool {
        *self == SHT0_A::SHT0_15
    }
}
#[doc = "Field `SHT0` writer - ADC12 Sample Hold 0 Select 0"]
pub type SHT0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC12CTL0_SPEC, u8, SHT0_A, 4, O>;
impl<'a, const O: u8> SHT0_W<'a, O> {
    #[doc = "ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn sht0_0(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_0)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 1"]
    #[inline(always)]
    pub fn sht0_1(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_1)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 2"]
    #[inline(always)]
    pub fn sht0_2(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_2)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 3"]
    #[inline(always)]
    pub fn sht0_3(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_3)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 4"]
    #[inline(always)]
    pub fn sht0_4(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_4)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 5"]
    #[inline(always)]
    pub fn sht0_5(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_5)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 6"]
    #[inline(always)]
    pub fn sht0_6(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_6)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 7"]
    #[inline(always)]
    pub fn sht0_7(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_7)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 8"]
    #[inline(always)]
    pub fn sht0_8(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_8)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 9"]
    #[inline(always)]
    pub fn sht0_9(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_9)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 10"]
    #[inline(always)]
    pub fn sht0_10(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_10)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 11"]
    #[inline(always)]
    pub fn sht0_11(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_11)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 12"]
    #[inline(always)]
    pub fn sht0_12(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_12)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 13"]
    #[inline(always)]
    pub fn sht0_13(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_13)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 14"]
    #[inline(always)]
    pub fn sht0_14(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_14)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 15"]
    #[inline(always)]
    pub fn sht0_15(self) -> &'a mut W {
        self.variant(SHT0_A::SHT0_15)
    }
}
#[doc = "Field `SHT1` reader - ADC12 Sample Hold 0 Select 0"]
pub type SHT1_R = crate::FieldReader<u8, SHT1_A>;
#[doc = "ADC12 Sample Hold 0 Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHT1_A {
    #[doc = "0: ADC12 Sample Hold 1 Select Bit: 0"]
    SHT1_0 = 0,
    #[doc = "1: ADC12 Sample Hold 1 Select Bit: 1"]
    SHT1_1 = 1,
    #[doc = "2: ADC12 Sample Hold 1 Select Bit: 2"]
    SHT1_2 = 2,
    #[doc = "3: ADC12 Sample Hold 1 Select Bit: 3"]
    SHT1_3 = 3,
    #[doc = "4: ADC12 Sample Hold 1 Select Bit: 4"]
    SHT1_4 = 4,
    #[doc = "5: ADC12 Sample Hold 1 Select Bit: 5"]
    SHT1_5 = 5,
    #[doc = "6: ADC12 Sample Hold 1 Select Bit: 6"]
    SHT1_6 = 6,
    #[doc = "7: ADC12 Sample Hold 1 Select Bit: 7"]
    SHT1_7 = 7,
    #[doc = "8: ADC12 Sample Hold 1 Select Bit: 8"]
    SHT1_8 = 8,
    #[doc = "9: ADC12 Sample Hold 1 Select Bit: 9"]
    SHT1_9 = 9,
    #[doc = "10: ADC12 Sample Hold 1 Select Bit: 10"]
    SHT1_10 = 10,
    #[doc = "11: ADC12 Sample Hold 1 Select Bit: 11"]
    SHT1_11 = 11,
    #[doc = "12: ADC12 Sample Hold 1 Select Bit: 12"]
    SHT1_12 = 12,
    #[doc = "13: ADC12 Sample Hold 1 Select Bit: 13"]
    SHT1_13 = 13,
    #[doc = "14: ADC12 Sample Hold 1 Select Bit: 14"]
    SHT1_14 = 14,
    #[doc = "15: ADC12 Sample Hold 1 Select Bit: 15"]
    SHT1_15 = 15,
}
impl From<SHT1_A> for u8 {
    #[inline(always)]
    fn from(variant: SHT1_A) -> Self {
        variant as _
    }
}
impl SHT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHT1_A {
        match self.bits {
            0 => SHT1_A::SHT1_0,
            1 => SHT1_A::SHT1_1,
            2 => SHT1_A::SHT1_2,
            3 => SHT1_A::SHT1_3,
            4 => SHT1_A::SHT1_4,
            5 => SHT1_A::SHT1_5,
            6 => SHT1_A::SHT1_6,
            7 => SHT1_A::SHT1_7,
            8 => SHT1_A::SHT1_8,
            9 => SHT1_A::SHT1_9,
            10 => SHT1_A::SHT1_10,
            11 => SHT1_A::SHT1_11,
            12 => SHT1_A::SHT1_12,
            13 => SHT1_A::SHT1_13,
            14 => SHT1_A::SHT1_14,
            15 => SHT1_A::SHT1_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SHT1_0`"]
    #[inline(always)]
    pub fn is_sht1_0(&self) -> bool {
        *self == SHT1_A::SHT1_0
    }
    #[doc = "Checks if the value of the field is `SHT1_1`"]
    #[inline(always)]
    pub fn is_sht1_1(&self) -> bool {
        *self == SHT1_A::SHT1_1
    }
    #[doc = "Checks if the value of the field is `SHT1_2`"]
    #[inline(always)]
    pub fn is_sht1_2(&self) -> bool {
        *self == SHT1_A::SHT1_2
    }
    #[doc = "Checks if the value of the field is `SHT1_3`"]
    #[inline(always)]
    pub fn is_sht1_3(&self) -> bool {
        *self == SHT1_A::SHT1_3
    }
    #[doc = "Checks if the value of the field is `SHT1_4`"]
    #[inline(always)]
    pub fn is_sht1_4(&self) -> bool {
        *self == SHT1_A::SHT1_4
    }
    #[doc = "Checks if the value of the field is `SHT1_5`"]
    #[inline(always)]
    pub fn is_sht1_5(&self) -> bool {
        *self == SHT1_A::SHT1_5
    }
    #[doc = "Checks if the value of the field is `SHT1_6`"]
    #[inline(always)]
    pub fn is_sht1_6(&self) -> bool {
        *self == SHT1_A::SHT1_6
    }
    #[doc = "Checks if the value of the field is `SHT1_7`"]
    #[inline(always)]
    pub fn is_sht1_7(&self) -> bool {
        *self == SHT1_A::SHT1_7
    }
    #[doc = "Checks if the value of the field is `SHT1_8`"]
    #[inline(always)]
    pub fn is_sht1_8(&self) -> bool {
        *self == SHT1_A::SHT1_8
    }
    #[doc = "Checks if the value of the field is `SHT1_9`"]
    #[inline(always)]
    pub fn is_sht1_9(&self) -> bool {
        *self == SHT1_A::SHT1_9
    }
    #[doc = "Checks if the value of the field is `SHT1_10`"]
    #[inline(always)]
    pub fn is_sht1_10(&self) -> bool {
        *self == SHT1_A::SHT1_10
    }
    #[doc = "Checks if the value of the field is `SHT1_11`"]
    #[inline(always)]
    pub fn is_sht1_11(&self) -> bool {
        *self == SHT1_A::SHT1_11
    }
    #[doc = "Checks if the value of the field is `SHT1_12`"]
    #[inline(always)]
    pub fn is_sht1_12(&self) -> bool {
        *self == SHT1_A::SHT1_12
    }
    #[doc = "Checks if the value of the field is `SHT1_13`"]
    #[inline(always)]
    pub fn is_sht1_13(&self) -> bool {
        *self == SHT1_A::SHT1_13
    }
    #[doc = "Checks if the value of the field is `SHT1_14`"]
    #[inline(always)]
    pub fn is_sht1_14(&self) -> bool {
        *self == SHT1_A::SHT1_14
    }
    #[doc = "Checks if the value of the field is `SHT1_15`"]
    #[inline(always)]
    pub fn is_sht1_15(&self) -> bool {
        *self == SHT1_A::SHT1_15
    }
}
#[doc = "Field `SHT1` writer - ADC12 Sample Hold 0 Select 0"]
pub type SHT1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC12CTL0_SPEC, u8, SHT1_A, 4, O>;
impl<'a, const O: u8> SHT1_W<'a, O> {
    #[doc = "ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn sht1_0(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_0)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 1"]
    #[inline(always)]
    pub fn sht1_1(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_1)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 2"]
    #[inline(always)]
    pub fn sht1_2(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_2)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 3"]
    #[inline(always)]
    pub fn sht1_3(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_3)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 4"]
    #[inline(always)]
    pub fn sht1_4(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_4)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 5"]
    #[inline(always)]
    pub fn sht1_5(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_5)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 6"]
    #[inline(always)]
    pub fn sht1_6(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_6)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 7"]
    #[inline(always)]
    pub fn sht1_7(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_7)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 8"]
    #[inline(always)]
    pub fn sht1_8(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_8)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 9"]
    #[inline(always)]
    pub fn sht1_9(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_9)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 10"]
    #[inline(always)]
    pub fn sht1_10(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_10)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 11"]
    #[inline(always)]
    pub fn sht1_11(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_11)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 12"]
    #[inline(always)]
    pub fn sht1_12(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_12)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 13"]
    #[inline(always)]
    pub fn sht1_13(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_13)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 14"]
    #[inline(always)]
    pub fn sht1_14(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_14)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 15"]
    #[inline(always)]
    pub fn sht1_15(self) -> &'a mut W {
        self.variant(SHT1_A::SHT1_15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    pub fn adc12sc(&self) -> ADC12SC_R {
        ADC12SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&self) -> ADC12TOVIE_R {
        ADC12TOVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12 Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&self) -> ADC12OVIE_R {
        ADC12OVIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    pub fn adc12on(&self) -> ADC12ON_R {
        ADC12ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12 Reference on"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&self) -> REF2_5V_R {
        REF2_5V_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select 0"]
    #[inline(always)]
    pub fn sht0(&self) -> SHT0_R {
        SHT0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 0 Select 0"]
    #[inline(always)]
    pub fn sht1(&self) -> SHT1_R {
        SHT1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn adc12sc(&mut self) -> ADC12SC_W<0> {
        ADC12SC_W::new(self)
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<1> {
        ENC_W::new(self)
    }
    #[doc = "Bit 2 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12tovie(&mut self) -> ADC12TOVIE_W<2> {
        ADC12TOVIE_W::new(self)
    }
    #[doc = "Bit 3 - ADC12 Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ovie(&mut self) -> ADC12OVIE_W<3> {
        ADC12OVIE_W::new(self)
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12on(&mut self) -> ADC12ON_W<4> {
        ADC12ON_W::new(self)
    }
    #[doc = "Bit 5 - ADC12 Reference on"]
    #[inline(always)]
    #[must_use]
    pub fn refon(&mut self) -> REFON_W<5> {
        REFON_W::new(self)
    }
    #[doc = "Bit 6 - ADC12 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    #[must_use]
    pub fn ref2_5v(&mut self) -> REF2_5V_W<6> {
        REF2_5V_W::new(self)
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    #[must_use]
    pub fn msc(&mut self) -> MSC_W<7> {
        MSC_W::new(self)
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn sht0(&mut self) -> SHT0_W<8> {
        SHT0_W::new(self)
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 0 Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn sht1(&mut self) -> SHT1_W<12> {
        SHT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12 Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl0](index.html) module"]
pub struct ADC12CTL0_SPEC;
impl crate::RegisterSpec for ADC12CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ctl0::R](R) reader structure"]
impl crate::Readable for ADC12CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ctl0::W](W) writer structure"]
impl crate::Writable for ADC12CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12CTL0 to value 0"]
impl crate::Resettable for ADC12CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
