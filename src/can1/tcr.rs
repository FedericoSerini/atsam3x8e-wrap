#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB0` writer - Transfer Request for Mailbox 0"]
pub type MB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `MB1` writer - Transfer Request for Mailbox 1"]
pub type MB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `MB2` writer - Transfer Request for Mailbox 2"]
pub type MB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `MB3` writer - Transfer Request for Mailbox 3"]
pub type MB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `MB4` writer - Transfer Request for Mailbox 4"]
pub type MB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `MB5` writer - Transfer Request for Mailbox 5"]
pub type MB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `MB6` writer - Transfer Request for Mailbox 6"]
pub type MB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `MB7` writer - Transfer Request for Mailbox 7"]
pub type MB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `TIMRST` writer - Timer Reset"]
pub type TIMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Transfer Request for Mailbox 0"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> MB0_W<0> {
        MB0_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Request for Mailbox 1"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> MB1_W<1> {
        MB1_W::new(self)
    }
    #[doc = "Bit 2 - Transfer Request for Mailbox 2"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> MB2_W<2> {
        MB2_W::new(self)
    }
    #[doc = "Bit 3 - Transfer Request for Mailbox 3"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> MB3_W<3> {
        MB3_W::new(self)
    }
    #[doc = "Bit 4 - Transfer Request for Mailbox 4"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> MB4_W<4> {
        MB4_W::new(self)
    }
    #[doc = "Bit 5 - Transfer Request for Mailbox 5"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> MB5_W<5> {
        MB5_W::new(self)
    }
    #[doc = "Bit 6 - Transfer Request for Mailbox 6"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> MB6_W<6> {
        MB6_W::new(self)
    }
    #[doc = "Bit 7 - Transfer Request for Mailbox 7"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> MB7_W<7> {
        MB7_W::new(self)
    }
    #[doc = "Bit 31 - Timer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn timrst(&mut self) -> TIMRST_W<31> {
        TIMRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
