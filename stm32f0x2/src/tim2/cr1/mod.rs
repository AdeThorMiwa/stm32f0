#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
    #[doc = r" Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CKD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKDR {
    #[doc = "Clock is not divided"] DIV1,
    #[doc = "Clock is divided by 2"] DIV2,
    #[doc = "Clock is divided by 4"] DIV4,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl CKDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            CKDR::DIV1 => 0,
            CKDR::DIV2 => 1,
            CKDR::DIV4 => 2,
            CKDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> CKDR {
        match value {
            0 => CKDR::DIV1,
            1 => CKDR::DIV2,
            2 => CKDR::DIV4,
            i => CKDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CKDR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CKDR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CKDR::DIV4
    }
}
#[doc = "Possible values of the field `ARPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARPER {
    #[doc = "TIMx_ARR register is not buffered"] NOTBUFFERED,
    #[doc = "TIMx_ARR register is buffered"] BUFFERED,
}
impl ARPER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            ARPER::NOTBUFFERED => false,
            ARPER::BUFFERED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> ARPER {
        match value {
            false => ARPER::NOTBUFFERED,
            true => ARPER::BUFFERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUFFERED`"]
    #[inline(always)]
    pub fn is_not_buffered(&self) -> bool {
        *self == ARPER::NOTBUFFERED
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == ARPER::BUFFERED
    }
}
#[doc = "Possible values of the field `CMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMSR {
    #[doc = "Edge-aligned mode. The counter counts up or down depending on the direction bit (DIR)."] EDGEALIGN,
    #[doc = "Center-aligned mode 1. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set only when the counter is counting down."] CENTERALIGNMODE1,
    #[doc = "Center-aligned mode 2. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set only when the counter is counting up."] CENTERALIGNMODE2,
    #[doc = "Center-aligned mode 3. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set both when the counter is counting up or down."] CENTERALIGNMODE3,
}
impl CMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            CMSR::EDGEALIGN => 0,
            CMSR::CENTERALIGNMODE1 => 1,
            CMSR::CENTERALIGNMODE2 => 2,
            CMSR::CENTERALIGNMODE3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> CMSR {
        match value {
            0 => CMSR::EDGEALIGN,
            1 => CMSR::CENTERALIGNMODE1,
            2 => CMSR::CENTERALIGNMODE2,
            3 => CMSR::CENTERALIGNMODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGEALIGN`"]
    #[inline(always)]
    pub fn is_edge_align(&self) -> bool {
        *self == CMSR::EDGEALIGN
    }
    #[doc = "Checks if the value of the field is `CENTERALIGNMODE1`"]
    #[inline(always)]
    pub fn is_center_align_mode1(&self) -> bool {
        *self == CMSR::CENTERALIGNMODE1
    }
    #[doc = "Checks if the value of the field is `CENTERALIGNMODE2`"]
    #[inline(always)]
    pub fn is_center_align_mode2(&self) -> bool {
        *self == CMSR::CENTERALIGNMODE2
    }
    #[doc = "Checks if the value of the field is `CENTERALIGNMODE3`"]
    #[inline(always)]
    pub fn is_center_align_mode3(&self) -> bool {
        *self == CMSR::CENTERALIGNMODE3
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Counter used as upcounter"] UP,
    #[doc = "Counter used as downcounter"] DOWN,
}
impl DIRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            DIRR::UP => false,
            DIRR::DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::UP,
            true => DIRR::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DIRR::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DIRR::DOWN
    }
}
#[doc = "Possible values of the field `OPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPMR {
    #[doc = "Counter is not stopped at update event"] CONTINUOUS,
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"] ONEPULSE,
}
impl OPMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            OPMR::CONTINUOUS => false,
            OPMR::ONEPULSE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> OPMR {
        match value {
            false => OPMR::CONTINUOUS,
            true => OPMR::ONEPULSE,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == OPMR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ONEPULSE`"]
    #[inline(always)]
    pub fn is_one_pulse(&self) -> bool {
        *self == OPMR::ONEPULSE
    }
}
#[doc = "Possible values of the field `URS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URSR {
    #[doc = "Any of the events generate an update interrupt or DMA request if enabled"] ANYEVENT,
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request if enabled"] ONLYOVERUNDER,
}
impl URSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            URSR::ANYEVENT => false,
            URSR::ONLYOVERUNDER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> URSR {
        match value {
            false => URSR::ANYEVENT,
            true => URSR::ONLYOVERUNDER,
        }
    }
    #[doc = "Checks if the value of the field is `ANYEVENT`"]
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == URSR::ANYEVENT
    }
    #[doc = "Checks if the value of the field is `ONLYOVERUNDER`"]
    #[inline(always)]
    pub fn is_only_over_under(&self) -> bool {
        *self == URSR::ONLYOVERUNDER
    }
}
#[doc = "Possible values of the field `UDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDISR {
    #[doc = "UEV enabled."] ENABLED,
    #[doc = "UEV disabled."] DISABLED,
}
impl UDISR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            UDISR::ENABLED => false,
            UDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> UDISR {
        match value {
            false => UDISR::ENABLED,
            true => UDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDISR::DISABLED
    }
}
#[doc = "Possible values of the field `CEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENR {
    #[doc = "Counter disabled"] DISABLED,
    #[doc = "Counter enabled"] ENABLED,
}
impl CENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            CENR::DISABLED => false,
            CENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CENR {
        match value {
            false => CENR::DISABLED,
            true => CENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CKD`"]
pub enum CKDW {
    #[doc = "Clock is not divided"] DIV1,
    #[doc = "Clock is divided by 2"] DIV2,
    #[doc = "Clock is divided by 4"] DIV4,
}
impl CKDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKDW::DIV1 => 0,
            CKDW::DIV2 => 1,
            CKDW::DIV4 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKDW<'a> {
    w: &'a mut W,
}
impl<'a> _CKDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock is not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CKDW::DIV1)
    }
    #[doc = "Clock is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CKDW::DIV2)
    }
    #[doc = "Clock is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CKDW::DIV4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARPE`"]
pub enum ARPEW {
    #[doc = "TIMx_ARR register is not buffered"] NOTBUFFERED,
    #[doc = "TIMx_ARR register is buffered"] BUFFERED,
}
impl ARPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ARPEW::NOTBUFFERED => false,
            ARPEW::BUFFERED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARPEW<'a> {
    w: &'a mut W,
}
impl<'a> _ARPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TIMx_ARR register is not buffered"]
    #[inline(always)]
    pub fn not_buffered(self) -> &'a mut W {
        self.variant(ARPEW::NOTBUFFERED)
    }
    #[doc = "TIMx_ARR register is buffered"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut W {
        self.variant(ARPEW::BUFFERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMS`"]
pub enum CMSW {
    #[doc = "Edge-aligned mode. The counter counts up or down depending on the direction bit (DIR)."] EDGEALIGN,
    #[doc = "Center-aligned mode 1. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set only when the counter is counting down."] CENTERALIGNMODE1,
    #[doc = "Center-aligned mode 2. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set only when the counter is counting up."] CENTERALIGNMODE2,
    #[doc = "Center-aligned mode 3. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set both when the counter is counting up or down."] CENTERALIGNMODE3,
}
impl CMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMSW::EDGEALIGN => 0,
            CMSW::CENTERALIGNMODE1 => 1,
            CMSW::CENTERALIGNMODE2 => 2,
            CMSW::CENTERALIGNMODE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMSW<'a> {
    w: &'a mut W,
}
impl<'a> _CMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Edge-aligned mode. The counter counts up or down depending on the direction bit (DIR)."]
    #[inline(always)]
    pub fn edge_align(self) -> &'a mut W {
        self.variant(CMSW::EDGEALIGN)
    }
    #[doc = "Center-aligned mode 1. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set only when the counter is counting down."]
    #[inline(always)]
    pub fn center_align_mode1(self) -> &'a mut W {
        self.variant(CMSW::CENTERALIGNMODE1)
    }
    #[doc = "Center-aligned mode 2. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set only when the counter is counting up."]
    #[inline(always)]
    pub fn center_align_mode2(self) -> &'a mut W {
        self.variant(CMSW::CENTERALIGNMODE2)
    }
    #[doc = "Center-aligned mode 3. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in TIMx_CCMRx register) are set both when the counter is counting up or down."]
    #[inline(always)]
    pub fn center_align_mode3(self) -> &'a mut W {
        self.variant(CMSW::CENTERALIGNMODE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIR`"]
pub enum DIRW {
    #[doc = "Counter used as upcounter"] UP,
    #[doc = "Counter used as downcounter"] DOWN,
}
impl DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRW::UP => false,
            DIRW::DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter used as upcounter"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(DIRW::UP)
    }
    #[doc = "Counter used as downcounter"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(DIRW::DOWN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPM`"]
pub enum OPMW {
    #[doc = "Counter is not stopped at update event"] CONTINUOUS,
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"] ONEPULSE,
}
impl OPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OPMW::CONTINUOUS => false,
            OPMW::ONEPULSE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPMW<'a> {
    w: &'a mut W,
}
impl<'a> _OPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(OPMW::CONTINUOUS)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn one_pulse(self) -> &'a mut W {
        self.variant(OPMW::ONEPULSE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `URS`"]
pub enum URSW {
    #[doc = "Any of the events generate an update interrupt or DMA request if enabled"] ANYEVENT,
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request if enabled"] ONLYOVERUNDER,
}
impl URSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            URSW::ANYEVENT => false,
            URSW::ONLYOVERUNDER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _URSW<'a> {
    w: &'a mut W,
}
impl<'a> _URSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: URSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Any of the events generate an update interrupt or DMA request if enabled"]
    #[inline(always)]
    pub fn any_event(self) -> &'a mut W {
        self.variant(URSW::ANYEVENT)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request if enabled"]
    #[inline(always)]
    pub fn only_over_under(self) -> &'a mut W {
        self.variant(URSW::ONLYOVERUNDER)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UDIS`"]
pub enum UDISW {
    #[doc = "UEV enabled."] ENABLED,
    #[doc = "UEV disabled."] DISABLED,
}
impl UDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UDISW::ENABLED => false,
            UDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UDISW<'a> {
    w: &'a mut W,
}
impl<'a> _UDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UEV enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDISW::ENABLED)
    }
    #[doc = "UEV disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDISW::DISABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CEN`"]
pub enum CENW {
    #[doc = "Counter disabled"] DISABLED,
    #[doc = "Counter enabled"] ENABLED,
}
impl CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CENW::DISABLED => false,
            CENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CENW<'a> {
    w: &'a mut W,
}
impl<'a> _CENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CENW::DISABLED)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:9 - Division ratio between the timer clock (CK_INT) frequency and sampling clock"]
    #[inline(always)]
    pub fn ckd(&self) -> CKDR {
        CKDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPER {
        ARPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cms(&self) -> CMSR {
        CMSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OPMR {
        OPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&self) -> URSR {
        URSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&self) -> UDISR {
        UDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CENR {
        CENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:9 - Division ratio between the timer clock (CK_INT) frequency and sampling clock"]
    #[inline(always)]
    pub fn ckd(&mut self) -> _CKDW {
        _CKDW { w: self }
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&mut self) -> _ARPEW {
        _ARPEW { w: self }
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cms(&mut self) -> _CMSW {
        _CMSW { w: self }
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&mut self) -> _OPMW {
        _OPMW { w: self }
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&mut self) -> _URSW {
        _URSW { w: self }
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&mut self) -> _UDISW {
        _UDISW { w: self }
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> _CENW {
        _CENW { w: self }
    }
}
