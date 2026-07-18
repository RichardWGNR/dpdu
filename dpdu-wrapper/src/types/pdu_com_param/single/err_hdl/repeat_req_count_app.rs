use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpCanFillerByte;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpRepeatReqCountApp(pub u8);

impl From<CpRepeatReqCountApp> for ComParamDefinition {
    fn from(value: CpRepeatReqCountApp) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RepeatReqCountApp".to_string(),
            variant: (value.0 as u32).into(),
        }
    }
}

impl From<CpRepeatReqCountApp> for u32 {
    fn from(value: CpRepeatReqCountApp) -> Self {
        value.0 as _
    }
}