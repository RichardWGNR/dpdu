use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpChangeSpeedCtrl(pub u32);

impl From<CpChangeSpeedCtrl> for ComParamDefinition {
    fn from(value: CpChangeSpeedCtrl) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_ChangeSpeedCtrl".to_string(),
            variant: value.0.into()
        }
    }
}

impl From<CpChangeSpeedCtrl> for u32 {
    fn from(value: CpChangeSpeedCtrl) -> Self {
        value.0
    }
}