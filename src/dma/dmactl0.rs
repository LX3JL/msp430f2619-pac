#[doc = "Register `DMACTL0` reader"]
pub struct R(crate::R<DMACTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL0` writer"]
pub struct W(crate::W<DMACTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL0_SPEC>;
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
impl From<crate::W<DMACTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA0TSEL` reader - DMA channel 0 transfer select bit 0"]
pub type DMA0TSEL_R = crate::FieldReader<u8, DMA0TSEL_A>;
#[doc = "DMA channel 0 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0TSEL_A {
    #[doc = "0: DMA channel 0 transfer select 0: DMA_REQ (sw)"]
    DMA0TSEL_0 = 0,
    #[doc = "1: DMA channel 0 transfer select 1: Timer_A (TACCR2.IFG)"]
    DMA0TSEL_1 = 1,
    #[doc = "2: DMA channel 0 transfer select 2: Timer_B (TBCCR2.IFG)"]
    DMA0TSEL_2 = 2,
    #[doc = "3: DMA channel 0 transfer select 3: USCIA0 receive"]
    DMA0TSEL_3 = 3,
    #[doc = "4: DMA channel 0 transfer select 4: USCIA0 transmit"]
    DMA0TSEL_4 = 4,
    #[doc = "5: DMA channel 0 transfer select 5: DAC12_0CTL.DAC12IFG"]
    DMA0TSEL_5 = 5,
    #[doc = "6: DMA channel 0 transfer select 6: ADC12 (ADC12IFG)"]
    DMA0TSEL_6 = 6,
    #[doc = "7: DMA channel 0 transfer select 7: Timer_A (TACCR0.IFG)"]
    DMA0TSEL_7 = 7,
    #[doc = "8: DMA channel 0 transfer select 8: Timer_B (TBCCR0.IFG)"]
    DMA0TSEL_8 = 8,
    #[doc = "9: DMA channel 0 transfer select 9: USCIA1 receive"]
    DMA0TSEL_9 = 9,
    #[doc = "10: DMA channel 0 transfer select 10: USCIA1 transmit"]
    DMA0TSEL_10 = 10,
    #[doc = "11: DMA channel 0 transfer select 11: Multiplier ready"]
    DMA0TSEL_11 = 11,
    #[doc = "12: DMA channel 0 transfer select 12: USCIB0 receive"]
    DMA0TSEL_12 = 12,
    #[doc = "13: DMA channel 0 transfer select 13: USCIB0 transmit"]
    DMA0TSEL_13 = 13,
    #[doc = "14: DMA channel 0 transfer select 14: previous DMA channel DMA2IFG"]
    DMA0TSEL_14 = 14,
    #[doc = "15: DMA channel 0 transfer select 15: ext. Trigger (DMAE0)"]
    DMA0TSEL_15 = 15,
}
impl From<DMA0TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0TSEL_A) -> Self {
        variant as _
    }
}
impl DMA0TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0TSEL_A {
        match self.bits {
            0 => DMA0TSEL_A::DMA0TSEL_0,
            1 => DMA0TSEL_A::DMA0TSEL_1,
            2 => DMA0TSEL_A::DMA0TSEL_2,
            3 => DMA0TSEL_A::DMA0TSEL_3,
            4 => DMA0TSEL_A::DMA0TSEL_4,
            5 => DMA0TSEL_A::DMA0TSEL_5,
            6 => DMA0TSEL_A::DMA0TSEL_6,
            7 => DMA0TSEL_A::DMA0TSEL_7,
            8 => DMA0TSEL_A::DMA0TSEL_8,
            9 => DMA0TSEL_A::DMA0TSEL_9,
            10 => DMA0TSEL_A::DMA0TSEL_10,
            11 => DMA0TSEL_A::DMA0TSEL_11,
            12 => DMA0TSEL_A::DMA0TSEL_12,
            13 => DMA0TSEL_A::DMA0TSEL_13,
            14 => DMA0TSEL_A::DMA0TSEL_14,
            15 => DMA0TSEL_A::DMA0TSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_0`"]
    #[inline(always)]
    pub fn is_dma0tsel_0(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_0
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_1`"]
    #[inline(always)]
    pub fn is_dma0tsel_1(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_1
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_2`"]
    #[inline(always)]
    pub fn is_dma0tsel_2(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_2
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_3`"]
    #[inline(always)]
    pub fn is_dma0tsel_3(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_3
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_4`"]
    #[inline(always)]
    pub fn is_dma0tsel_4(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_4
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_5`"]
    #[inline(always)]
    pub fn is_dma0tsel_5(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_5
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_6`"]
    #[inline(always)]
    pub fn is_dma0tsel_6(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_6
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_7`"]
    #[inline(always)]
    pub fn is_dma0tsel_7(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_7
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_8`"]
    #[inline(always)]
    pub fn is_dma0tsel_8(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_8
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_9`"]
    #[inline(always)]
    pub fn is_dma0tsel_9(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_9
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_10`"]
    #[inline(always)]
    pub fn is_dma0tsel_10(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_10
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_11`"]
    #[inline(always)]
    pub fn is_dma0tsel_11(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_11
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_12`"]
    #[inline(always)]
    pub fn is_dma0tsel_12(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_12
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_13`"]
    #[inline(always)]
    pub fn is_dma0tsel_13(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_13
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_14`"]
    #[inline(always)]
    pub fn is_dma0tsel_14(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_14
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_15`"]
    #[inline(always)]
    pub fn is_dma0tsel_15(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_15
    }
}
#[doc = "Field `DMA0TSEL` writer - DMA channel 0 transfer select bit 0"]
pub type DMA0TSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DMACTL0_SPEC, u8, DMA0TSEL_A, 4, O>;
impl<'a, const O: u8> DMA0TSEL_W<'a, O> {
    #[doc = "DMA channel 0 transfer select 0: DMA_REQ (sw)"]
    #[inline(always)]
    pub fn dma0tsel_0(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_0)
    }
    #[doc = "DMA channel 0 transfer select 1: Timer_A (TACCR2.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_1(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_1)
    }
    #[doc = "DMA channel 0 transfer select 2: Timer_B (TBCCR2.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_2(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_2)
    }
    #[doc = "DMA channel 0 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn dma0tsel_3(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_3)
    }
    #[doc = "DMA channel 0 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma0tsel_4(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_4)
    }
    #[doc = "DMA channel 0 transfer select 5: DAC12_0CTL.DAC12IFG"]
    #[inline(always)]
    pub fn dma0tsel_5(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_5)
    }
    #[doc = "DMA channel 0 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn dma0tsel_6(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_6)
    }
    #[doc = "DMA channel 0 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_7(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_7)
    }
    #[doc = "DMA channel 0 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_8(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_8)
    }
    #[doc = "DMA channel 0 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn dma0tsel_9(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_9)
    }
    #[doc = "DMA channel 0 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma0tsel_10(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_10)
    }
    #[doc = "DMA channel 0 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn dma0tsel_11(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_11)
    }
    #[doc = "DMA channel 0 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn dma0tsel_12(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_12)
    }
    #[doc = "DMA channel 0 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma0tsel_13(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_13)
    }
    #[doc = "DMA channel 0 transfer select 14: previous DMA channel DMA2IFG"]
    #[inline(always)]
    pub fn dma0tsel_14(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_14)
    }
    #[doc = "DMA channel 0 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma0tsel_15(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_15)
    }
}
#[doc = "Field `DMA1TSEL` reader - DMA channel 1 transfer select bit 0"]
pub type DMA1TSEL_R = crate::FieldReader<u8, DMA1TSEL_A>;
#[doc = "DMA channel 1 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1TSEL_A {
    #[doc = "0: DMA channel 1 transfer select 0: DMA_REQ"]
    DMA1TSEL_0 = 0,
    #[doc = "1: DMA channel 1 transfer select 1: Timer_A CCRIFG.2"]
    DMA1TSEL_1 = 1,
    #[doc = "2: DMA channel 1 transfer select 2: Timer_B CCRIFG.2"]
    DMA1TSEL_2 = 2,
    #[doc = "3: DMA channel 1 transfer select 3: USCIA0 receive"]
    DMA1TSEL_3 = 3,
    #[doc = "4: DMA channel 1 transfer select 4: USCIA0 transmit"]
    DMA1TSEL_4 = 4,
    #[doc = "5: DMA channel 1 transfer select 5: DAC12.0IFG"]
    DMA1TSEL_5 = 5,
    #[doc = "6: DMA channel 1 transfer select 6: ADC12 (ADC12IFG)"]
    DMA1TSEL_6 = 6,
    #[doc = "7: DMA channel 1 transfer select 7: Timer_A (TACCR0.IFG)"]
    DMA1TSEL_7 = 7,
    #[doc = "8: DMA channel 1 transfer select 8: Timer_B (TBCCR0.IFG)"]
    DMA1TSEL_8 = 8,
    #[doc = "9: DMA channel 1 transfer select 9: USCIA1 receive"]
    DMA1TSEL_9 = 9,
    #[doc = "10: DMA channel 1 transfer select 10: USCIA1 transmit"]
    DMA1TSEL_10 = 10,
    #[doc = "11: DMA channel 1 transfer select 11: Multiplier ready"]
    DMA1TSEL_11 = 11,
    #[doc = "12: DMA channel 1 transfer select 12: USCIB0 receive"]
    DMA1TSEL_12 = 12,
    #[doc = "13: DMA channel 1 transfer select 13: USCIB0 transmit"]
    DMA1TSEL_13 = 13,
    #[doc = "14: DMA channel 1 transfer select 14: previous DMA channel DMA0IFG"]
    DMA1TSEL_14 = 14,
    #[doc = "15: DMA channel 1 transfer select 15: ext. Trigger (DMAE0)"]
    DMA1TSEL_15 = 15,
}
impl From<DMA1TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1TSEL_A) -> Self {
        variant as _
    }
}
impl DMA1TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1TSEL_A {
        match self.bits {
            0 => DMA1TSEL_A::DMA1TSEL_0,
            1 => DMA1TSEL_A::DMA1TSEL_1,
            2 => DMA1TSEL_A::DMA1TSEL_2,
            3 => DMA1TSEL_A::DMA1TSEL_3,
            4 => DMA1TSEL_A::DMA1TSEL_4,
            5 => DMA1TSEL_A::DMA1TSEL_5,
            6 => DMA1TSEL_A::DMA1TSEL_6,
            7 => DMA1TSEL_A::DMA1TSEL_7,
            8 => DMA1TSEL_A::DMA1TSEL_8,
            9 => DMA1TSEL_A::DMA1TSEL_9,
            10 => DMA1TSEL_A::DMA1TSEL_10,
            11 => DMA1TSEL_A::DMA1TSEL_11,
            12 => DMA1TSEL_A::DMA1TSEL_12,
            13 => DMA1TSEL_A::DMA1TSEL_13,
            14 => DMA1TSEL_A::DMA1TSEL_14,
            15 => DMA1TSEL_A::DMA1TSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_0`"]
    #[inline(always)]
    pub fn is_dma1tsel_0(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_0
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_1`"]
    #[inline(always)]
    pub fn is_dma1tsel_1(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_1
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_2`"]
    #[inline(always)]
    pub fn is_dma1tsel_2(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_2
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_3`"]
    #[inline(always)]
    pub fn is_dma1tsel_3(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_3
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_4`"]
    #[inline(always)]
    pub fn is_dma1tsel_4(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_4
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_5`"]
    #[inline(always)]
    pub fn is_dma1tsel_5(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_5
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_6`"]
    #[inline(always)]
    pub fn is_dma1tsel_6(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_6
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_7`"]
    #[inline(always)]
    pub fn is_dma1tsel_7(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_7
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_8`"]
    #[inline(always)]
    pub fn is_dma1tsel_8(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_8
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_9`"]
    #[inline(always)]
    pub fn is_dma1tsel_9(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_9
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_10`"]
    #[inline(always)]
    pub fn is_dma1tsel_10(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_10
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_11`"]
    #[inline(always)]
    pub fn is_dma1tsel_11(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_11
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_12`"]
    #[inline(always)]
    pub fn is_dma1tsel_12(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_12
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_13`"]
    #[inline(always)]
    pub fn is_dma1tsel_13(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_13
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_14`"]
    #[inline(always)]
    pub fn is_dma1tsel_14(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_14
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_15`"]
    #[inline(always)]
    pub fn is_dma1tsel_15(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_15
    }
}
#[doc = "Field `DMA1TSEL` writer - DMA channel 1 transfer select bit 0"]
pub type DMA1TSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DMACTL0_SPEC, u8, DMA1TSEL_A, 4, O>;
impl<'a, const O: u8> DMA1TSEL_W<'a, O> {
    #[doc = "DMA channel 1 transfer select 0: DMA_REQ"]
    #[inline(always)]
    pub fn dma1tsel_0(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_0)
    }
    #[doc = "DMA channel 1 transfer select 1: Timer_A CCRIFG.2"]
    #[inline(always)]
    pub fn dma1tsel_1(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_1)
    }
    #[doc = "DMA channel 1 transfer select 2: Timer_B CCRIFG.2"]
    #[inline(always)]
    pub fn dma1tsel_2(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_2)
    }
    #[doc = "DMA channel 1 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn dma1tsel_3(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_3)
    }
    #[doc = "DMA channel 1 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma1tsel_4(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_4)
    }
    #[doc = "DMA channel 1 transfer select 5: DAC12.0IFG"]
    #[inline(always)]
    pub fn dma1tsel_5(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_5)
    }
    #[doc = "DMA channel 1 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn dma1tsel_6(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_6)
    }
    #[doc = "DMA channel 1 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_7(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_7)
    }
    #[doc = "DMA channel 1 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_8(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_8)
    }
    #[doc = "DMA channel 1 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn dma1tsel_9(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_9)
    }
    #[doc = "DMA channel 1 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma1tsel_10(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_10)
    }
    #[doc = "DMA channel 1 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn dma1tsel_11(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_11)
    }
    #[doc = "DMA channel 1 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn dma1tsel_12(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_12)
    }
    #[doc = "DMA channel 1 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma1tsel_13(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_13)
    }
    #[doc = "DMA channel 1 transfer select 14: previous DMA channel DMA0IFG"]
    #[inline(always)]
    pub fn dma1tsel_14(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_14)
    }
    #[doc = "DMA channel 1 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma1tsel_15(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_15)
    }
}
#[doc = "Field `DMA2TSEL` reader - DMA channel 2 transfer select bit 0"]
pub type DMA2TSEL_R = crate::FieldReader<u8, DMA2TSEL_A>;
#[doc = "DMA channel 2 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA2TSEL_A {
    #[doc = "0: DMA channel 2 transfer select 0: DMA_REQ"]
    DMA2TSEL_0 = 0,
    #[doc = "1: DMA channel 2 transfer select 1: Timer_A CCRIFG.2"]
    DMA2TSEL_1 = 1,
    #[doc = "2: DMA channel 2 transfer select 2: Timer_B CCRIFG.2"]
    DMA2TSEL_2 = 2,
    #[doc = "3: DMA channel 2 transfer select 3: USCIA0 receive"]
    DMA2TSEL_3 = 3,
    #[doc = "4: DMA channel 2 transfer select 4: USCIA0 transmit"]
    DMA2TSEL_4 = 4,
    #[doc = "5: DMA channel 2 transfer select 5: DAC12.0IFG"]
    DMA2TSEL_5 = 5,
    #[doc = "6: DMA channel 2 transfer select 6: ADC12 (ADC12IFG)"]
    DMA2TSEL_6 = 6,
    #[doc = "7: DMA channel 2 transfer select 7: Timer_A (TACCR0.IFG)"]
    DMA2TSEL_7 = 7,
    #[doc = "8: DMA channel 2 transfer select 8: Timer_B (TBCCR0.IFG)"]
    DMA2TSEL_8 = 8,
    #[doc = "9: DMA channel 2 transfer select 9: USCIA1 receive"]
    DMA2TSEL_9 = 9,
    #[doc = "10: DMA channel 2 transfer select 10: USCIA1 transmit"]
    DMA2TSEL_10 = 10,
    #[doc = "11: DMA channel 2 transfer select 11: Multiplier ready"]
    DMA2TSEL_11 = 11,
    #[doc = "12: DMA channel 2 transfer select 12: USCIB0 receive"]
    DMA2TSEL_12 = 12,
    #[doc = "13: DMA channel 2 transfer select 13: USCIB0 transmit"]
    DMA2TSEL_13 = 13,
    #[doc = "14: DMA channel 2 transfer select 14: previous DMA channel DMA1IFG"]
    DMA2TSEL_14 = 14,
    #[doc = "15: DMA channel 2 transfer select 15: ext. Trigger (DMAE0)"]
    DMA2TSEL_15 = 15,
}
impl From<DMA2TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA2TSEL_A) -> Self {
        variant as _
    }
}
impl DMA2TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2TSEL_A {
        match self.bits {
            0 => DMA2TSEL_A::DMA2TSEL_0,
            1 => DMA2TSEL_A::DMA2TSEL_1,
            2 => DMA2TSEL_A::DMA2TSEL_2,
            3 => DMA2TSEL_A::DMA2TSEL_3,
            4 => DMA2TSEL_A::DMA2TSEL_4,
            5 => DMA2TSEL_A::DMA2TSEL_5,
            6 => DMA2TSEL_A::DMA2TSEL_6,
            7 => DMA2TSEL_A::DMA2TSEL_7,
            8 => DMA2TSEL_A::DMA2TSEL_8,
            9 => DMA2TSEL_A::DMA2TSEL_9,
            10 => DMA2TSEL_A::DMA2TSEL_10,
            11 => DMA2TSEL_A::DMA2TSEL_11,
            12 => DMA2TSEL_A::DMA2TSEL_12,
            13 => DMA2TSEL_A::DMA2TSEL_13,
            14 => DMA2TSEL_A::DMA2TSEL_14,
            15 => DMA2TSEL_A::DMA2TSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_0`"]
    #[inline(always)]
    pub fn is_dma2tsel_0(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_0
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_1`"]
    #[inline(always)]
    pub fn is_dma2tsel_1(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_1
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_2`"]
    #[inline(always)]
    pub fn is_dma2tsel_2(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_2
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_3`"]
    #[inline(always)]
    pub fn is_dma2tsel_3(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_3
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_4`"]
    #[inline(always)]
    pub fn is_dma2tsel_4(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_4
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_5`"]
    #[inline(always)]
    pub fn is_dma2tsel_5(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_5
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_6`"]
    #[inline(always)]
    pub fn is_dma2tsel_6(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_6
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_7`"]
    #[inline(always)]
    pub fn is_dma2tsel_7(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_7
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_8`"]
    #[inline(always)]
    pub fn is_dma2tsel_8(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_8
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_9`"]
    #[inline(always)]
    pub fn is_dma2tsel_9(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_9
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_10`"]
    #[inline(always)]
    pub fn is_dma2tsel_10(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_10
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_11`"]
    #[inline(always)]
    pub fn is_dma2tsel_11(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_11
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_12`"]
    #[inline(always)]
    pub fn is_dma2tsel_12(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_12
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_13`"]
    #[inline(always)]
    pub fn is_dma2tsel_13(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_13
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_14`"]
    #[inline(always)]
    pub fn is_dma2tsel_14(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_14
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_15`"]
    #[inline(always)]
    pub fn is_dma2tsel_15(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TSEL_15
    }
}
#[doc = "Field `DMA2TSEL` writer - DMA channel 2 transfer select bit 0"]
pub type DMA2TSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DMACTL0_SPEC, u8, DMA2TSEL_A, 4, O>;
impl<'a, const O: u8> DMA2TSEL_W<'a, O> {
    #[doc = "DMA channel 2 transfer select 0: DMA_REQ"]
    #[inline(always)]
    pub fn dma2tsel_0(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_0)
    }
    #[doc = "DMA channel 2 transfer select 1: Timer_A CCRIFG.2"]
    #[inline(always)]
    pub fn dma2tsel_1(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_1)
    }
    #[doc = "DMA channel 2 transfer select 2: Timer_B CCRIFG.2"]
    #[inline(always)]
    pub fn dma2tsel_2(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_2)
    }
    #[doc = "DMA channel 2 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn dma2tsel_3(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_3)
    }
    #[doc = "DMA channel 2 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma2tsel_4(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_4)
    }
    #[doc = "DMA channel 2 transfer select 5: DAC12.0IFG"]
    #[inline(always)]
    pub fn dma2tsel_5(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_5)
    }
    #[doc = "DMA channel 2 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn dma2tsel_6(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_6)
    }
    #[doc = "DMA channel 2 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_7(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_7)
    }
    #[doc = "DMA channel 2 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_8(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_8)
    }
    #[doc = "DMA channel 2 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn dma2tsel_9(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_9)
    }
    #[doc = "DMA channel 2 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma2tsel_10(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_10)
    }
    #[doc = "DMA channel 2 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn dma2tsel_11(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_11)
    }
    #[doc = "DMA channel 2 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn dma2tsel_12(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_12)
    }
    #[doc = "DMA channel 2 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma2tsel_13(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_13)
    }
    #[doc = "DMA channel 2 transfer select 14: previous DMA channel DMA1IFG"]
    #[inline(always)]
    pub fn dma2tsel_14(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_14)
    }
    #[doc = "DMA channel 2 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma2tsel_15(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_15)
    }
}
impl R {
    #[doc = "Bits 0:3 - DMA channel 0 transfer select bit 0"]
    #[inline(always)]
    pub fn dma0tsel(&self) -> DMA0TSEL_R {
        DMA0TSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA channel 1 transfer select bit 0"]
    #[inline(always)]
    pub fn dma1tsel(&self) -> DMA1TSEL_R {
        DMA1TSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA channel 2 transfer select bit 0"]
    #[inline(always)]
    pub fn dma2tsel(&self) -> DMA2TSEL_R {
        DMA2TSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA channel 0 transfer select bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma0tsel(&mut self) -> DMA0TSEL_W<0> {
        DMA0TSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - DMA channel 1 transfer select bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma1tsel(&mut self) -> DMA1TSEL_W<4> {
        DMA1TSEL_W::new(self)
    }
    #[doc = "Bits 8:11 - DMA channel 2 transfer select bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma2tsel(&mut self) -> DMA2TSEL_W<8> {
        DMA2TSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Module Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl0](index.html) module"]
pub struct DMACTL0_SPEC;
impl crate::RegisterSpec for DMACTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmactl0::R](R) reader structure"]
impl crate::Readable for DMACTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl0::W](W) writer structure"]
impl crate::Writable for DMACTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTL0 to value 0"]
impl crate::Resettable for DMACTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
