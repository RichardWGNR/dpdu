use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::unique::CpCanPhysReqExtAddr;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum CpCanPhysReqFormat {
    NormalUnsegmented11Bit = 0,
    NormalUnsegmented29Bit = 2,
    NormalSegmented11Bit = 4,
    NormalSegmented11BitWithFc = 5,
    NormalSegmented29Bit = 6,
    NormalSegmented29BitWithFc = 7,
    ExtUnsegmented11Bit = 8,
    ExtUnsegmented29Bit = 10,
    ExtSegmented11Bit = 12,
    ExtSegmented11BitWithFc = 13,
    ExtSegmented29Bit = 14,
    ExtSegmented29BitWithFc = 15,
    NormalUnsegmented11BitWithoutPadding = 32,
    NormalUnsegmented29BitWithoutPadding = 34,
    NormalSegmented11BitWithoutFcAndWithoutPadding = 36,
    NormalSegmented11BitWithFcAndWithoutPadding = 37,
    NormalSegmented29BitWithoutFcAndWithoutPadding = 38,
    NormalSegmented29BitWithFcAndWithoutPadding = 39,
    ExtUnsegmented11BitWithoutPadding = 40,
    ExtUnsegmented29BitWithoutPadding = 42,
    ExtSegmented11BitWithoutFcAndWithoutPadding = 44,
    ExtSegmented11BitWithFcAndWithoutPadding = 45,
    ExtSegmented29BitWithoutFcAndWithoutPadding = 46,
    ExtSegmented29BitWithFcAndWithoutPadding = 47,
    NormalUnsegmented11BitWithPadding = 48,
    NormalUnsegmented29BitWithPadding = 50,
    NormalSegmented11BitWithoutFcAndWithPadding = 52,
    NormalSegmented11BitWithFcAndWithPadding = 53,
    NormalSegmented29BitWithoutFcAndWithPadding = 54,
    NormalSegmented29BitWithFcAndWithPadding = 55,
    ExtUnsegmented11BitWithPadding = 56,
    ExtUnsegmented29BitWithPadding = 58,
    ExtSegmented11BitWithoutFcAndWithPadding = 60,
    ExtSegmented11BitWithFcAndWithPadding = 61,
    ExtSegmented29BitWithoutFcAndWithPadding = 62,
    ExtSegmented29BitWithFcAndWithPadding = 63
}

impl From<CpCanPhysReqFormat> for ComParamDefinition {
    fn from(value: CpCanPhysReqFormat) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanPhysReqFormat".to_string(),
            variant: (value as u32).into(),
        }
    }
}

impl From<CpCanPhysReqFormat> for u32 {
    fn from(value: CpCanPhysReqFormat) -> Self {
        value as _
    }
}