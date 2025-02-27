#[doc = "Register `SWTRIGCTRL` reader"]
pub struct R(crate::R<SWTRIGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWTRIGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWTRIGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWTRIGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWTRIGCTRL` writer"]
pub struct W(crate::W<SWTRIGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIGCTRL_SPEC>;
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
impl From<crate::W<SWTRIGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWTRIG0` reader - Channel 0 Software Trigger"]
pub struct SWTRIG0_R(crate::FieldReader<bool, bool>);
impl SWTRIG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG0` writer - Channel 0 Software Trigger"]
pub struct SWTRIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SWTRIG1` reader - Channel 1 Software Trigger"]
pub struct SWTRIG1_R(crate::FieldReader<bool, bool>);
impl SWTRIG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG1` writer - Channel 1 Software Trigger"]
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SWTRIG2` reader - Channel 2 Software Trigger"]
pub struct SWTRIG2_R(crate::FieldReader<bool, bool>);
impl SWTRIG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG2` writer - Channel 2 Software Trigger"]
pub struct SWTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SWTRIG3` reader - Channel 3 Software Trigger"]
pub struct SWTRIG3_R(crate::FieldReader<bool, bool>);
impl SWTRIG3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG3` writer - Channel 3 Software Trigger"]
pub struct SWTRIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SWTRIG4` reader - Channel 4 Software Trigger"]
pub struct SWTRIG4_R(crate::FieldReader<bool, bool>);
impl SWTRIG4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG4` writer - Channel 4 Software Trigger"]
pub struct SWTRIG4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SWTRIG5` reader - Channel 5 Software Trigger"]
pub struct SWTRIG5_R(crate::FieldReader<bool, bool>);
impl SWTRIG5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG5` writer - Channel 5 Software Trigger"]
pub struct SWTRIG5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SWTRIG6` reader - Channel 6 Software Trigger"]
pub struct SWTRIG6_R(crate::FieldReader<bool, bool>);
impl SWTRIG6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG6` writer - Channel 6 Software Trigger"]
pub struct SWTRIG6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SWTRIG7` reader - Channel 7 Software Trigger"]
pub struct SWTRIG7_R(crate::FieldReader<bool, bool>);
impl SWTRIG7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG7` writer - Channel 7 Software Trigger"]
pub struct SWTRIG7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SWTRIG8` reader - Channel 8 Software Trigger"]
pub struct SWTRIG8_R(crate::FieldReader<bool, bool>);
impl SWTRIG8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG8` writer - Channel 8 Software Trigger"]
pub struct SWTRIG8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SWTRIG9` reader - Channel 9 Software Trigger"]
pub struct SWTRIG9_R(crate::FieldReader<bool, bool>);
impl SWTRIG9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG9` writer - Channel 9 Software Trigger"]
pub struct SWTRIG9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SWTRIG10` reader - Channel 10 Software Trigger"]
pub struct SWTRIG10_R(crate::FieldReader<bool, bool>);
impl SWTRIG10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG10` writer - Channel 10 Software Trigger"]
pub struct SWTRIG10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SWTRIG11` reader - Channel 11 Software Trigger"]
pub struct SWTRIG11_R(crate::FieldReader<bool, bool>);
impl SWTRIG11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG11` writer - Channel 11 Software Trigger"]
pub struct SWTRIG11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&self) -> SWTRIG0_R {
        SWTRIG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&self) -> SWTRIG1_R {
        SWTRIG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&self) -> SWTRIG2_R {
        SWTRIG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&self) -> SWTRIG3_R {
        SWTRIG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&self) -> SWTRIG4_R {
        SWTRIG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&self) -> SWTRIG5_R {
        SWTRIG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&self) -> SWTRIG6_R {
        SWTRIG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&self) -> SWTRIG7_R {
        SWTRIG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&self) -> SWTRIG8_R {
        SWTRIG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&self) -> SWTRIG9_R {
        SWTRIG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&self) -> SWTRIG10_R {
        SWTRIG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&self) -> SWTRIG11_R {
        SWTRIG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&mut self) -> SWTRIG0_W {
        SWTRIG0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W {
        SWTRIG2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&mut self) -> SWTRIG3_W {
        SWTRIG3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&mut self) -> SWTRIG4_W {
        SWTRIG4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&mut self) -> SWTRIG5_W {
        SWTRIG5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&mut self) -> SWTRIG6_W {
        SWTRIG6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&mut self) -> SWTRIG7_W {
        SWTRIG7_W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&mut self) -> SWTRIG8_W {
        SWTRIG8_W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&mut self) -> SWTRIG9_W {
        SWTRIG9_W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&mut self) -> SWTRIG10_W {
        SWTRIG10_W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&mut self) -> SWTRIG11_W {
        SWTRIG11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigctrl](index.html) module"]
pub struct SWTRIGCTRL_SPEC;
impl crate::RegisterSpec for SWTRIGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swtrigctrl::R](R) reader structure"]
impl crate::Readable for SWTRIGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swtrigctrl::W](W) writer structure"]
impl crate::Writable for SWTRIGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWTRIGCTRL to value 0"]
impl crate::Resettable for SWTRIGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
