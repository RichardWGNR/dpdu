use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::err_hdl::repeat_req_count_app::CpRepeatReqCountApp;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpRepeatReqCountTrans(pub u8);

impl From<CpRepeatReqCountTrans> for ComParamDefinition {
    fn from(value: CpRepeatReqCountTrans) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RepeatReqCountTrans".to_string(),
            variant: (value.0 as u32).into(),
        }
    }
}

impl From<CpRepeatReqCountTrans> for u32 {
    fn from(value: CpRepeatReqCountTrans) -> Self {
        value.0 as _
    }
}