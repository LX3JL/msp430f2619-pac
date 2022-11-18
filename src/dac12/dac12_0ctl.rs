#[doc = "Register `DAC12_0CTL` reader"]
pub struct R(crate::R<DAC12_0CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC12_0CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC12_0CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC12_0CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC12_0CTL` writer"]
pub struct W(crate::W<DAC12_0CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC12_0CTL_SPEC>;
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
impl From<crate::W<DAC12_0CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC12_0CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC12GRP` reader - DAC12 group"]
pub type DAC12GRP_R = crate::BitReader<bool>;
#[doc = "Field `DAC12GRP` writer - DAC12 group"]
pub type DAC12GRP_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
#[doc = "Field `DAC12ENC` reader - DAC12 enable conversion"]
pub type DAC12ENC_R = crate::BitReader<bool>;
#[doc = "Field `DAC12ENC` writer - DAC12 enable conversion"]
pub type DAC12ENC_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
#[doc = "Field `DAC12IFG` reader - DAC12 interrupt flag"]
pub type DAC12IFG_R = crate::BitReader<bool>;
#[doc = "Field `DAC12IFG` writer - DAC12 interrupt flag"]
pub type DAC12IFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
#[doc = "Field `DAC12IE` reader - DAC12 interrupt enable"]
pub type DAC12IE_R = crate::BitReader<bool>;
#[doc = "Field `DAC12IE` writer - DAC12 interrupt enable"]
pub type DAC12IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
#[doc = "Field `DAC12DF` reader - DAC12 data format"]
pub type DAC12DF_R = crate::BitReader<bool>;
#[doc = "Field `DAC12DF` writer - DAC12 data format"]
pub type DAC12DF_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
#[doc = "Field `DAC12AMP` reader - DAC12 amplifier bit 0"]
pub type DAC12AMP_R = crate::FieldReader<u8, DAC12AMP_A>;
#[doc = "DAC12 amplifier bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC12AMP_A {
    #[doc = "0: DAC12 amplifier 0: off"]
    DAC12AMP_0 = 0,
    #[doc = "1: DAC12 amplifier 1: off"]
    DAC12AMP_1 = 1,
    #[doc = "2: DAC12 amplifier 2: low"]
    DAC12AMP_2 = 2,
    #[doc = "3: DAC12 amplifier 3: low"]
    DAC12AMP_3 = 3,
    #[doc = "4: DAC12 amplifier 4: low"]
    DAC12AMP_4 = 4,
    #[doc = "5: DAC12 amplifier 5: medium"]
    DAC12AMP_5 = 5,
    #[doc = "6: DAC12 amplifier 6: medium"]
    DAC12AMP_6 = 6,
    #[doc = "7: DAC12 amplifier 7: high"]
    DAC12AMP_7 = 7,
}
impl From<DAC12AMP_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC12AMP_A) -> Self {
        variant as _
    }
}
impl DAC12AMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC12AMP_A {
        match self.bits {
            0 => DAC12AMP_A::DAC12AMP_0,
            1 => DAC12AMP_A::DAC12AMP_1,
            2 => DAC12AMP_A::DAC12AMP_2,
            3 => DAC12AMP_A::DAC12AMP_3,
            4 => DAC12AMP_A::DAC12AMP_4,
            5 => DAC12AMP_A::DAC12AMP_5,
            6 => DAC12AMP_A::DAC12AMP_6,
            7 => DAC12AMP_A::DAC12AMP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DAC12AMP_0`"]
    #[inline(always)]
    pub fn is_dac12amp_0(&self) -> bool {
        *self == DAC12AMP_A::DAC12AMP_0
    }
    #[doc = "Checks if the value of the field is `DAC12AMP_1`"]
    #[inline(always)]
    pub fn is_dac12amp_1(&self) -> bool {
        *self == DAC12AMP_A::DAC12AMP_1
    }
    #[doc = "Checks if the value of the field is `DAC12AMP_2`"]
    #[inline(always)]
    pub fn is_dac12amp_2(&self) -> bool {
        *self == DAC12AMP_A::DAC12AMP_2
    }
    #[doc = "Checks if the value of the field is `DAC12AMP_3`"]
    #[inline(always)]
    pub fn is_dac12amp_3(&self) -> bool {
        *self == DAC12AMP_A::DAC12AMP_3
    }
    #[doc = "Checks if the value of the field is `DAC12AMP_4`"]
    #[inline(always)]
    pub fn is_dac12amp_4(&self) -> bool {
        *self == DAC12AMP_A::DAC12AMP_4
    }
    #[doc = "Checks if the value of the field is `DAC12AMP_5`"]
    #[inline(always)]
    pub fn is_dac12amp_5(&self) -> bool {
        *self == DAC12AMP_A::DAC12AMP_5
    }
    #[doc = "Checks if the value of the field is `DAC12AMP_6`"]
    #[inline(always)]
    pub fn is_dac12amp_6(&self) -> bool {
        *self == DAC12AMP_A::DAC12AMP_6
    }
    #[doc = "Checks if the value of the field is `DAC12AMP_7`"]
    #[inline(always)]
    pub fn is_dac12amp_7(&self) -> bool {
        *self == DAC12AMP_A::DAC12AMP_7
    }
}
#[doc = "Field `DAC12AMP` writer - DAC12 amplifier bit 0"]
pub type DAC12AMP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DAC12_0CTL_SPEC, u8, DAC12AMP_A, 3, O>;
impl<'a, const O: u8> DAC12AMP_W<'a, O> {
    #[doc = "DAC12 amplifier 0: off"]
    #[inline(always)]
    pub fn dac12amp_0(self) -> &'a mut W {
        self.variant(DAC12AMP_A::DAC12AMP_0)
    }
    #[doc = "DAC12 amplifier 1: off"]
    #[inline(always)]
    pub fn dac12amp_1(self) -> &'a mut W {
        self.variant(DAC12AMP_A::DAC12AMP_1)
    }
    #[doc = "DAC12 amplifier 2: low"]
    #[inline(always)]
    pub fn dac12amp_2(self) -> &'a mut W {
        self.variant(DAC12AMP_A::DAC12AMP_2)
    }
    #[doc = "DAC12 amplifier 3: low"]
    #[inline(always)]
    pub fn dac12amp_3(self) -> &'a mut W {
        self.variant(DAC12AMP_A::DAC12AMP_3)
    }
    #[doc = "DAC12 amplifier 4: low"]
    #[inline(always)]
    pub fn dac12amp_4(self) -> &'a mut W {
        self.variant(DAC12AMP_A::DAC12AMP_4)
    }
    #[doc = "DAC12 amplifier 5: medium"]
    #[inline(always)]
    pub fn dac12amp_5(self) -> &'a mut W {
        self.variant(DAC12AMP_A::DAC12AMP_5)
    }
    #[doc = "DAC12 amplifier 6: medium"]
    #[inline(always)]
    pub fn dac12amp_6(self) -> &'a mut W {
        self.variant(DAC12AMP_A::DAC12AMP_6)
    }
    #[doc = "DAC12 amplifier 7: high"]
    #[inline(always)]
    pub fn dac12amp_7(self) -> &'a mut W {
        self.variant(DAC12AMP_A::DAC12AMP_7)
    }
}
#[doc = "Field `DAC12IR` reader - DAC12 input reference and output range"]
pub type DAC12IR_R = crate::BitReader<bool>;
#[doc = "Field `DAC12IR` writer - DAC12 input reference and output range"]
pub type DAC12IR_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
#[doc = "Field `DAC12CALON` reader - DAC12 calibration"]
pub type DAC12CALON_R = crate::BitReader<bool>;
#[doc = "Field `DAC12CALON` writer - DAC12 calibration"]
pub type DAC12CALON_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
#[doc = "Field `DAC12LSEL` reader - DAC12 load select bit 0"]
pub type DAC12LSEL_R = crate::FieldReader<u8, DAC12LSEL_A>;
#[doc = "DAC12 load select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC12LSEL_A {
    #[doc = "0: DAC12 load select 0: direct"]
    DAC12LSEL_0 = 0,
    #[doc = "1: DAC12 load select 1: latched with DAT"]
    DAC12LSEL_1 = 1,
    #[doc = "2: DAC12 load select 2: latched with pos. Timer_A3.OUT1"]
    DAC12LSEL_2 = 2,
    #[doc = "3: DAC12 load select 3: latched with pos. Timer_B7.OUT1"]
    DAC12LSEL_3 = 3,
}
impl From<DAC12LSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC12LSEL_A) -> Self {
        variant as _
    }
}
impl DAC12LSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC12LSEL_A {
        match self.bits {
            0 => DAC12LSEL_A::DAC12LSEL_0,
            1 => DAC12LSEL_A::DAC12LSEL_1,
            2 => DAC12LSEL_A::DAC12LSEL_2,
            3 => DAC12LSEL_A::DAC12LSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DAC12LSEL_0`"]
    #[inline(always)]
    pub fn is_dac12lsel_0(&self) -> bool {
        *self == DAC12LSEL_A::DAC12LSEL_0
    }
    #[doc = "Checks if the value of the field is `DAC12LSEL_1`"]
    #[inline(always)]
    pub fn is_dac12lsel_1(&self) -> bool {
        *self == DAC12LSEL_A::DAC12LSEL_1
    }
    #[doc = "Checks if the value of the field is `DAC12LSEL_2`"]
    #[inline(always)]
    pub fn is_dac12lsel_2(&self) -> bool {
        *self == DAC12LSEL_A::DAC12LSEL_2
    }
    #[doc = "Checks if the value of the field is `DAC12LSEL_3`"]
    #[inline(always)]
    pub fn is_dac12lsel_3(&self) -> bool {
        *self == DAC12LSEL_A::DAC12LSEL_3
    }
}
#[doc = "Field `DAC12LSEL` writer - DAC12 load select bit 0"]
pub type DAC12LSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DAC12_0CTL_SPEC, u8, DAC12LSEL_A, 2, O>;
impl<'a, const O: u8> DAC12LSEL_W<'a, O> {
    #[doc = "DAC12 load select 0: direct"]
    #[inline(always)]
    pub fn dac12lsel_0(self) -> &'a mut W {
        self.variant(DAC12LSEL_A::DAC12LSEL_0)
    }
    #[doc = "DAC12 load select 1: latched with DAT"]
    #[inline(always)]
    pub fn dac12lsel_1(self) -> &'a mut W {
        self.variant(DAC12LSEL_A::DAC12LSEL_1)
    }
    #[doc = "DAC12 load select 2: latched with pos. Timer_A3.OUT1"]
    #[inline(always)]
    pub fn dac12lsel_2(self) -> &'a mut W {
        self.variant(DAC12LSEL_A::DAC12LSEL_2)
    }
    #[doc = "DAC12 load select 3: latched with pos. Timer_B7.OUT1"]
    #[inline(always)]
    pub fn dac12lsel_3(self) -> &'a mut W {
        self.variant(DAC12LSEL_A::DAC12LSEL_3)
    }
}
#[doc = "Field `DAC12RES` reader - DAC12 resolution"]
pub type DAC12RES_R = crate::BitReader<bool>;
#[doc = "Field `DAC12RES` writer - DAC12 resolution"]
pub type DAC12RES_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
#[doc = "Field `DAC12SREF` reader - DAC12 reference bit 0"]
pub type DAC12SREF_R = crate::FieldReader<u8, DAC12SREF_A>;
#[doc = "DAC12 reference bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC12SREF_A {
    #[doc = "0: DAC12 reference 0: Vref+"]
    DAC12SREF_0 = 0,
    #[doc = "1: DAC12 reference 1: Vref+"]
    DAC12SREF_1 = 1,
    #[doc = "2: DAC12 reference 2: Veref+"]
    DAC12SREF_2 = 2,
    #[doc = "3: DAC12 reference 3: Veref+"]
    DAC12SREF_3 = 3,
}
impl From<DAC12SREF_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC12SREF_A) -> Self {
        variant as _
    }
}
impl DAC12SREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC12SREF_A {
        match self.bits {
            0 => DAC12SREF_A::DAC12SREF_0,
            1 => DAC12SREF_A::DAC12SREF_1,
            2 => DAC12SREF_A::DAC12SREF_2,
            3 => DAC12SREF_A::DAC12SREF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DAC12SREF_0`"]
    #[inline(always)]
    pub fn is_dac12sref_0(&self) -> bool {
        *self == DAC12SREF_A::DAC12SREF_0
    }
    #[doc = "Checks if the value of the field is `DAC12SREF_1`"]
    #[inline(always)]
    pub fn is_dac12sref_1(&self) -> bool {
        *self == DAC12SREF_A::DAC12SREF_1
    }
    #[doc = "Checks if the value of the field is `DAC12SREF_2`"]
    #[inline(always)]
    pub fn is_dac12sref_2(&self) -> bool {
        *self == DAC12SREF_A::DAC12SREF_2
    }
    #[doc = "Checks if the value of the field is `DAC12SREF_3`"]
    #[inline(always)]
    pub fn is_dac12sref_3(&self) -> bool {
        *self == DAC12SREF_A::DAC12SREF_3
    }
}
#[doc = "Field `DAC12SREF` writer - DAC12 reference bit 0"]
pub type DAC12SREF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DAC12_0CTL_SPEC, u8, DAC12SREF_A, 2, O>;
impl<'a, const O: u8> DAC12SREF_W<'a, O> {
    #[doc = "DAC12 reference 0: Vref+"]
    #[inline(always)]
    pub fn dac12sref_0(self) -> &'a mut W {
        self.variant(DAC12SREF_A::DAC12SREF_0)
    }
    #[doc = "DAC12 reference 1: Vref+"]
    #[inline(always)]
    pub fn dac12sref_1(self) -> &'a mut W {
        self.variant(DAC12SREF_A::DAC12SREF_1)
    }
    #[doc = "DAC12 reference 2: Veref+"]
    #[inline(always)]
    pub fn dac12sref_2(self) -> &'a mut W {
        self.variant(DAC12SREF_A::DAC12SREF_2)
    }
    #[doc = "DAC12 reference 3: Veref+"]
    #[inline(always)]
    pub fn dac12sref_3(self) -> &'a mut W {
        self.variant(DAC12SREF_A::DAC12SREF_3)
    }
}
#[doc = "Field `DAC12OPS` reader - DAC12 Operation Amp."]
pub type DAC12OPS_R = crate::BitReader<bool>;
#[doc = "Field `DAC12OPS` writer - DAC12 Operation Amp."]
pub type DAC12OPS_W<'a, const O: u8> = crate::BitWriter<'a, u16, DAC12_0CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DAC12 group"]
    #[inline(always)]
    pub fn dac12grp(&self) -> DAC12GRP_R {
        DAC12GRP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC12 enable conversion"]
    #[inline(always)]
    pub fn dac12enc(&self) -> DAC12ENC_R {
        DAC12ENC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC12 interrupt flag"]
    #[inline(always)]
    pub fn dac12ifg(&self) -> DAC12IFG_R {
        DAC12IFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC12 interrupt enable"]
    #[inline(always)]
    pub fn dac12ie(&self) -> DAC12IE_R {
        DAC12IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DAC12 data format"]
    #[inline(always)]
    pub fn dac12df(&self) -> DAC12DF_R {
        DAC12DF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - DAC12 amplifier bit 0"]
    #[inline(always)]
    pub fn dac12amp(&self) -> DAC12AMP_R {
        DAC12AMP_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - DAC12 input reference and output range"]
    #[inline(always)]
    pub fn dac12ir(&self) -> DAC12IR_R {
        DAC12IR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC12 calibration"]
    #[inline(always)]
    pub fn dac12calon(&self) -> DAC12CALON_R {
        DAC12CALON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - DAC12 load select bit 0"]
    #[inline(always)]
    pub fn dac12lsel(&self) -> DAC12LSEL_R {
        DAC12LSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - DAC12 resolution"]
    #[inline(always)]
    pub fn dac12res(&self) -> DAC12RES_R {
        DAC12RES_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - DAC12 reference bit 0"]
    #[inline(always)]
    pub fn dac12sref(&self) -> DAC12SREF_R {
        DAC12SREF_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - DAC12 Operation Amp."]
    #[inline(always)]
    pub fn dac12ops(&self) -> DAC12OPS_R {
        DAC12OPS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC12 group"]
    #[inline(always)]
    #[must_use]
    pub fn dac12grp(&mut self) -> DAC12GRP_W<0> {
        DAC12GRP_W::new(self)
    }
    #[doc = "Bit 1 - DAC12 enable conversion"]
    #[inline(always)]
    #[must_use]
    pub fn dac12enc(&mut self) -> DAC12ENC_W<1> {
        DAC12ENC_W::new(self)
    }
    #[doc = "Bit 2 - DAC12 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dac12ifg(&mut self) -> DAC12IFG_W<2> {
        DAC12IFG_W::new(self)
    }
    #[doc = "Bit 3 - DAC12 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac12ie(&mut self) -> DAC12IE_W<3> {
        DAC12IE_W::new(self)
    }
    #[doc = "Bit 4 - DAC12 data format"]
    #[inline(always)]
    #[must_use]
    pub fn dac12df(&mut self) -> DAC12DF_W<4> {
        DAC12DF_W::new(self)
    }
    #[doc = "Bits 5:7 - DAC12 amplifier bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dac12amp(&mut self) -> DAC12AMP_W<5> {
        DAC12AMP_W::new(self)
    }
    #[doc = "Bit 8 - DAC12 input reference and output range"]
    #[inline(always)]
    #[must_use]
    pub fn dac12ir(&mut self) -> DAC12IR_W<8> {
        DAC12IR_W::new(self)
    }
    #[doc = "Bit 9 - DAC12 calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dac12calon(&mut self) -> DAC12CALON_W<9> {
        DAC12CALON_W::new(self)
    }
    #[doc = "Bits 10:11 - DAC12 load select bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dac12lsel(&mut self) -> DAC12LSEL_W<10> {
        DAC12LSEL_W::new(self)
    }
    #[doc = "Bit 12 - DAC12 resolution"]
    #[inline(always)]
    #[must_use]
    pub fn dac12res(&mut self) -> DAC12RES_W<12> {
        DAC12RES_W::new(self)
    }
    #[doc = "Bits 13:14 - DAC12 reference bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dac12sref(&mut self) -> DAC12SREF_W<13> {
        DAC12SREF_W::new(self)
    }
    #[doc = "Bit 15 - DAC12 Operation Amp."]
    #[inline(always)]
    #[must_use]
    pub fn dac12ops(&mut self) -> DAC12OPS_W<15> {
        DAC12OPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC12_0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac12_0ctl](index.html) module"]
pub struct DAC12_0CTL_SPEC;
impl crate::RegisterSpec for DAC12_0CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dac12_0ctl::R](R) reader structure"]
impl crate::Readable for DAC12_0CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac12_0ctl::W](W) writer structure"]
impl crate::Writable for DAC12_0CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC12_0CTL to value 0"]
impl crate::Resettable for DAC12_0CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
