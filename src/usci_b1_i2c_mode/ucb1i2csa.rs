#[doc = "Register `UCB1I2CSA` reader"]
pub struct R(crate::R<UCB1I2CSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1I2CSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB1I2CSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB1I2CSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1I2CSA` writer"]
pub struct W(crate::W<UCB1I2CSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1I2CSA_SPEC>;
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
impl From<crate::W<UCB1I2CSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB1I2CSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSA0` reader - I2C Slave Address 0"]
pub type UCSA0_R = crate::BitReader<bool>;
#[doc = "Field `UCSA0` writer - I2C Slave Address 0"]
pub type UCSA0_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA1` reader - I2C Slave Address 1"]
pub type UCSA1_R = crate::BitReader<bool>;
#[doc = "Field `UCSA1` writer - I2C Slave Address 1"]
pub type UCSA1_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA2` reader - I2C Slave Address 2"]
pub type UCSA2_R = crate::BitReader<bool>;
#[doc = "Field `UCSA2` writer - I2C Slave Address 2"]
pub type UCSA2_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA3` reader - I2C Slave Address 3"]
pub type UCSA3_R = crate::BitReader<bool>;
#[doc = "Field `UCSA3` writer - I2C Slave Address 3"]
pub type UCSA3_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA4` reader - I2C Slave Address 4"]
pub type UCSA4_R = crate::BitReader<bool>;
#[doc = "Field `UCSA4` writer - I2C Slave Address 4"]
pub type UCSA4_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA5` reader - I2C Slave Address 5"]
pub type UCSA5_R = crate::BitReader<bool>;
#[doc = "Field `UCSA5` writer - I2C Slave Address 5"]
pub type UCSA5_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA6` reader - I2C Slave Address 6"]
pub type UCSA6_R = crate::BitReader<bool>;
#[doc = "Field `UCSA6` writer - I2C Slave Address 6"]
pub type UCSA6_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA7` reader - I2C Slave Address 7"]
pub type UCSA7_R = crate::BitReader<bool>;
#[doc = "Field `UCSA7` writer - I2C Slave Address 7"]
pub type UCSA7_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA8` reader - I2C Slave Address 8"]
pub type UCSA8_R = crate::BitReader<bool>;
#[doc = "Field `UCSA8` writer - I2C Slave Address 8"]
pub type UCSA8_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
#[doc = "Field `UCSA9` reader - I2C Slave Address 9"]
pub type UCSA9_R = crate::BitReader<bool>;
#[doc = "Field `UCSA9` writer - I2C Slave Address 9"]
pub type UCSA9_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1I2CSA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa0(&self) -> UCSA0_R {
        UCSA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Slave Address 1"]
    #[inline(always)]
    pub fn ucsa1(&self) -> UCSA1_R {
        UCSA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Slave Address 2"]
    #[inline(always)]
    pub fn ucsa2(&self) -> UCSA2_R {
        UCSA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Slave Address 3"]
    #[inline(always)]
    pub fn ucsa3(&self) -> UCSA3_R {
        UCSA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Slave Address 4"]
    #[inline(always)]
    pub fn ucsa4(&self) -> UCSA4_R {
        UCSA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Slave Address 5"]
    #[inline(always)]
    pub fn ucsa5(&self) -> UCSA5_R {
        UCSA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Slave Address 6"]
    #[inline(always)]
    pub fn ucsa6(&self) -> UCSA6_R {
        UCSA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Slave Address 7"]
    #[inline(always)]
    pub fn ucsa7(&self) -> UCSA7_R {
        UCSA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Slave Address 8"]
    #[inline(always)]
    pub fn ucsa8(&self) -> UCSA8_R {
        UCSA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Slave Address 9"]
    #[inline(always)]
    pub fn ucsa9(&self) -> UCSA9_R {
        UCSA9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa0(&mut self) -> UCSA0_W<0> {
        UCSA0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Slave Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa1(&mut self) -> UCSA1_W<1> {
        UCSA1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Slave Address 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa2(&mut self) -> UCSA2_W<2> {
        UCSA2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Slave Address 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa3(&mut self) -> UCSA3_W<3> {
        UCSA3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Slave Address 4"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa4(&mut self) -> UCSA4_W<4> {
        UCSA4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Slave Address 5"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa5(&mut self) -> UCSA5_W<5> {
        UCSA5_W::new(self)
    }
    #[doc = "Bit 6 - I2C Slave Address 6"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa6(&mut self) -> UCSA6_W<6> {
        UCSA6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Slave Address 7"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa7(&mut self) -> UCSA7_W<7> {
        UCSA7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Slave Address 8"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa8(&mut self) -> UCSA8_W<8> {
        UCSA8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Slave Address 9"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa9(&mut self) -> UCSA9_W<9> {
        UCSA9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 I2C Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2csa](index.html) module"]
pub struct UCB1I2CSA_SPEC;
impl crate::RegisterSpec for UCB1I2CSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb1i2csa::R](R) reader structure"]
impl crate::Readable for UCB1I2CSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1i2csa::W](W) writer structure"]
impl crate::Writable for UCB1I2CSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB1I2CSA to value 0"]
impl crate::Resettable for UCB1I2CSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
