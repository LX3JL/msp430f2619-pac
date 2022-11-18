#[doc = "Register `DMA2CTL` reader"]
pub struct R(crate::R<DMA2CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA2CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA2CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA2CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA2CTL` writer"]
pub struct W(crate::W<DMA2CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA2CTL_SPEC>;
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
impl From<crate::W<DMA2CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA2CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAREQ` reader - Initiate DMA transfer with DMATSEL"]
pub type DMAREQ_R = crate::BitReader<bool>;
#[doc = "Field `DMAREQ` writer - Initiate DMA transfer with DMATSEL"]
pub type DMAREQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2CTL_SPEC, bool, O>;
#[doc = "Field `DMAABORT` reader - DMA transfer aborted by NMI"]
pub type DMAABORT_R = crate::BitReader<bool>;
#[doc = "Field `DMAABORT` writer - DMA transfer aborted by NMI"]
pub type DMAABORT_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2CTL_SPEC, bool, O>;
#[doc = "Field `DMAIE` reader - DMA interrupt enable"]
pub type DMAIE_R = crate::BitReader<bool>;
#[doc = "Field `DMAIE` writer - DMA interrupt enable"]
pub type DMAIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2CTL_SPEC, bool, O>;
#[doc = "Field `DMAIFG` reader - DMA interrupt flag"]
pub type DMAIFG_R = crate::BitReader<bool>;
#[doc = "Field `DMAIFG` writer - DMA interrupt flag"]
pub type DMAIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2CTL_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2CTL_SPEC, bool, O>;
#[doc = "Field `DMALEVEL` reader - DMA level sensitive trigger select"]
pub type DMALEVEL_R = crate::BitReader<bool>;
#[doc = "Field `DMALEVEL` writer - DMA level sensitive trigger select"]
pub type DMALEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2CTL_SPEC, bool, O>;
#[doc = "Field `DMASRCBYTE` reader - DMA source byte"]
pub type DMASRCBYTE_R = crate::BitReader<bool>;
#[doc = "Field `DMASRCBYTE` writer - DMA source byte"]
pub type DMASRCBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2CTL_SPEC, bool, O>;
#[doc = "Field `DMADSTBYTE` reader - DMA destination byte"]
pub type DMADSTBYTE_R = crate::BitReader<bool>;
#[doc = "Field `DMADSTBYTE` writer - DMA destination byte"]
pub type DMADSTBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2CTL_SPEC, bool, O>;
#[doc = "Field `DMASRCINCR` reader - DMA source increment bit 0"]
pub type DMASRCINCR_R = crate::FieldReader<u8, DMASRCINCR_A>;
#[doc = "DMA source increment bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMASRCINCR_A {
    #[doc = "0: DMA source increment 0: source address unchanged"]
    DMASRCINCR_0 = 0,
    #[doc = "1: DMA source increment 1: source address unchanged"]
    DMASRCINCR_1 = 1,
    #[doc = "2: DMA source increment 2: source address decremented"]
    DMASRCINCR_2 = 2,
    #[doc = "3: DMA source increment 3: source address incremented"]
    DMASRCINCR_3 = 3,
}
impl From<DMASRCINCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASRCINCR_A) -> Self {
        variant as _
    }
}
impl DMASRCINCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASRCINCR_A {
        match self.bits {
            0 => DMASRCINCR_A::DMASRCINCR_0,
            1 => DMASRCINCR_A::DMASRCINCR_1,
            2 => DMASRCINCR_A::DMASRCINCR_2,
            3 => DMASRCINCR_A::DMASRCINCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_0`"]
    #[inline(always)]
    pub fn is_dmasrcincr_0(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_0
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_1`"]
    #[inline(always)]
    pub fn is_dmasrcincr_1(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_1
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_2`"]
    #[inline(always)]
    pub fn is_dmasrcincr_2(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_2
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_3`"]
    #[inline(always)]
    pub fn is_dmasrcincr_3(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_3
    }
}
#[doc = "Field `DMASRCINCR` writer - DMA source increment bit 0"]
pub type DMASRCINCR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DMA2CTL_SPEC, u8, DMASRCINCR_A, 2, O>;
impl<'a, const O: u8> DMASRCINCR_W<'a, O> {
    #[doc = "DMA source increment 0: source address unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_0(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_0)
    }
    #[doc = "DMA source increment 1: source address unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_1(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_1)
    }
    #[doc = "DMA source increment 2: source address decremented"]
    #[inline(always)]
    pub fn dmasrcincr_2(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_2)
    }
    #[doc = "DMA source increment 3: source address incremented"]
    #[inline(always)]
    pub fn dmasrcincr_3(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_3)
    }
}
#[doc = "Field `DMADSTINCR` reader - DMA destination increment bit 0"]
pub type DMADSTINCR_R = crate::FieldReader<u8, DMADSTINCR_A>;
#[doc = "DMA destination increment bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMADSTINCR_A {
    #[doc = "0: DMA destination increment 0: destination address unchanged"]
    DMADSTINCR_0 = 0,
    #[doc = "1: DMA destination increment 1: destination address unchanged"]
    DMADSTINCR_1 = 1,
    #[doc = "2: DMA destination increment 2: destination address decremented"]
    DMADSTINCR_2 = 2,
    #[doc = "3: DMA destination increment 3: destination address incremented"]
    DMADSTINCR_3 = 3,
}
impl From<DMADSTINCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADSTINCR_A) -> Self {
        variant as _
    }
}
impl DMADSTINCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADSTINCR_A {
        match self.bits {
            0 => DMADSTINCR_A::DMADSTINCR_0,
            1 => DMADSTINCR_A::DMADSTINCR_1,
            2 => DMADSTINCR_A::DMADSTINCR_2,
            3 => DMADSTINCR_A::DMADSTINCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_0`"]
    #[inline(always)]
    pub fn is_dmadstincr_0(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_0
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_1`"]
    #[inline(always)]
    pub fn is_dmadstincr_1(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_1
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_2`"]
    #[inline(always)]
    pub fn is_dmadstincr_2(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_2
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_3`"]
    #[inline(always)]
    pub fn is_dmadstincr_3(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_3
    }
}
#[doc = "Field `DMADSTINCR` writer - DMA destination increment bit 0"]
pub type DMADSTINCR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DMA2CTL_SPEC, u8, DMADSTINCR_A, 2, O>;
impl<'a, const O: u8> DMADSTINCR_W<'a, O> {
    #[doc = "DMA destination increment 0: destination address unchanged"]
    #[inline(always)]
    pub fn dmadstincr_0(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_0)
    }
    #[doc = "DMA destination increment 1: destination address unchanged"]
    #[inline(always)]
    pub fn dmadstincr_1(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_1)
    }
    #[doc = "DMA destination increment 2: destination address decremented"]
    #[inline(always)]
    pub fn dmadstincr_2(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_2)
    }
    #[doc = "DMA destination increment 3: destination address incremented"]
    #[inline(always)]
    pub fn dmadstincr_3(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_3)
    }
}
#[doc = "Field `DMADT` reader - DMA transfer mode bit 0"]
pub type DMADT_R = crate::FieldReader<u8, DMADT_A>;
#[doc = "DMA transfer mode bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMADT_A {
    #[doc = "0: DMA transfer mode 0: single"]
    DMADT_0 = 0,
    #[doc = "1: DMA transfer mode 1: block"]
    DMADT_1 = 1,
    #[doc = "2: DMA transfer mode 2: interleaved"]
    DMADT_2 = 2,
    #[doc = "3: DMA transfer mode 3: interleaved"]
    DMADT_3 = 3,
    #[doc = "4: DMA transfer mode 4: single"]
    DMADT_4 = 4,
    #[doc = "5: DMA transfer mode 5: block"]
    DMADT_5 = 5,
    #[doc = "6: DMA transfer mode 6: interleaved"]
    DMADT_6 = 6,
    #[doc = "7: DMA transfer mode 7: interleaved"]
    DMADT_7 = 7,
}
impl From<DMADT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADT_A) -> Self {
        variant as _
    }
}
impl DMADT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADT_A {
        match self.bits {
            0 => DMADT_A::DMADT_0,
            1 => DMADT_A::DMADT_1,
            2 => DMADT_A::DMADT_2,
            3 => DMADT_A::DMADT_3,
            4 => DMADT_A::DMADT_4,
            5 => DMADT_A::DMADT_5,
            6 => DMADT_A::DMADT_6,
            7 => DMADT_A::DMADT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMADT_0`"]
    #[inline(always)]
    pub fn is_dmadt_0(&self) -> bool {
        *self == DMADT_A::DMADT_0
    }
    #[doc = "Checks if the value of the field is `DMADT_1`"]
    #[inline(always)]
    pub fn is_dmadt_1(&self) -> bool {
        *self == DMADT_A::DMADT_1
    }
    #[doc = "Checks if the value of the field is `DMADT_2`"]
    #[inline(always)]
    pub fn is_dmadt_2(&self) -> bool {
        *self == DMADT_A::DMADT_2
    }
    #[doc = "Checks if the value of the field is `DMADT_3`"]
    #[inline(always)]
    pub fn is_dmadt_3(&self) -> bool {
        *self == DMADT_A::DMADT_3
    }
    #[doc = "Checks if the value of the field is `DMADT_4`"]
    #[inline(always)]
    pub fn is_dmadt_4(&self) -> bool {
        *self == DMADT_A::DMADT_4
    }
    #[doc = "Checks if the value of the field is `DMADT_5`"]
    #[inline(always)]
    pub fn is_dmadt_5(&self) -> bool {
        *self == DMADT_A::DMADT_5
    }
    #[doc = "Checks if the value of the field is `DMADT_6`"]
    #[inline(always)]
    pub fn is_dmadt_6(&self) -> bool {
        *self == DMADT_A::DMADT_6
    }
    #[doc = "Checks if the value of the field is `DMADT_7`"]
    #[inline(always)]
    pub fn is_dmadt_7(&self) -> bool {
        *self == DMADT_A::DMADT_7
    }
}
#[doc = "Field `DMADT` writer - DMA transfer mode bit 0"]
pub type DMADT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DMA2CTL_SPEC, u8, DMADT_A, 3, O>;
impl<'a, const O: u8> DMADT_W<'a, O> {
    #[doc = "DMA transfer mode 0: single"]
    #[inline(always)]
    pub fn dmadt_0(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_0)
    }
    #[doc = "DMA transfer mode 1: block"]
    #[inline(always)]
    pub fn dmadt_1(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_1)
    }
    #[doc = "DMA transfer mode 2: interleaved"]
    #[inline(always)]
    pub fn dmadt_2(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_2)
    }
    #[doc = "DMA transfer mode 3: interleaved"]
    #[inline(always)]
    pub fn dmadt_3(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_3)
    }
    #[doc = "DMA transfer mode 4: single"]
    #[inline(always)]
    pub fn dmadt_4(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_4)
    }
    #[doc = "DMA transfer mode 5: block"]
    #[inline(always)]
    pub fn dmadt_5(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_5)
    }
    #[doc = "DMA transfer mode 6: interleaved"]
    #[inline(always)]
    pub fn dmadt_6(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_6)
    }
    #[doc = "DMA transfer mode 7: interleaved"]
    #[inline(always)]
    pub fn dmadt_7(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_7)
    }
}
impl R {
    #[doc = "Bit 0 - Initiate DMA transfer with DMATSEL"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA transfer aborted by NMI"]
    #[inline(always)]
    pub fn dmaabort(&self) -> DMAABORT_R {
        DMAABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&self) -> DMAIE_R {
        DMAIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&self) -> DMAIFG_R {
        DMAIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA level sensitive trigger select"]
    #[inline(always)]
    pub fn dmalevel(&self) -> DMALEVEL_R {
        DMALEVEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&self) -> DMASRCBYTE_R {
        DMASRCBYTE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&self) -> DMADSTBYTE_R {
        DMADSTBYTE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DMA source increment bit 0"]
    #[inline(always)]
    pub fn dmasrcincr(&self) -> DMASRCINCR_R {
        DMASRCINCR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DMA destination increment bit 0"]
    #[inline(always)]
    pub fn dmadstincr(&self) -> DMADSTINCR_R {
        DMADSTINCR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - DMA transfer mode bit 0"]
    #[inline(always)]
    pub fn dmadt(&self) -> DMADT_R {
        DMADT_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate DMA transfer with DMATSEL"]
    #[inline(always)]
    #[must_use]
    pub fn dmareq(&mut self) -> DMAREQ_W<0> {
        DMAREQ_W::new(self)
    }
    #[doc = "Bit 1 - DMA transfer aborted by NMI"]
    #[inline(always)]
    #[must_use]
    pub fn dmaabort(&mut self) -> DMAABORT_W<1> {
        DMAABORT_W::new(self)
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaie(&mut self) -> DMAIE_W<2> {
        DMAIE_W::new(self)
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmaifg(&mut self) -> DMAIFG_W<3> {
        DMAIFG_W::new(self)
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<4> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 5 - DMA level sensitive trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn dmalevel(&mut self) -> DMALEVEL_W<5> {
        DMALEVEL_W::new(self)
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    #[must_use]
    pub fn dmasrcbyte(&mut self) -> DMASRCBYTE_W<6> {
        DMASRCBYTE_W::new(self)
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    #[must_use]
    pub fn dmadstbyte(&mut self) -> DMADSTBYTE_W<7> {
        DMADSTBYTE_W::new(self)
    }
    #[doc = "Bits 8:9 - DMA source increment bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dmasrcincr(&mut self) -> DMASRCINCR_W<8> {
        DMASRCINCR_W::new(self)
    }
    #[doc = "Bits 10:11 - DMA destination increment bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dmadstincr(&mut self) -> DMADSTINCR_W<10> {
        DMADSTINCR_W::new(self)
    }
    #[doc = "Bits 12:14 - DMA transfer mode bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dmadt(&mut self) -> DMADT_W<12> {
        DMADT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 2 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2ctl](index.html) module"]
pub struct DMA2CTL_SPEC;
impl crate::RegisterSpec for DMA2CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dma2ctl::R](R) reader structure"]
impl crate::Readable for DMA2CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma2ctl::W](W) writer structure"]
impl crate::Writable for DMA2CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA2CTL to value 0"]
impl crate::Resettable for DMA2CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
