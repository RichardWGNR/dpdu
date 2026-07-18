use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpChangeSpeedRate;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpChangeSpeedResCtrl(pub u32);

impl From<CpChangeSpeedResCtrl> for ComParamDefinition {
    fn from(value: CpChangeSpeedResCtrl) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_ChangeSpeedResCtrl".to_string(),
            variant: value.0.into()
        }
    }
}

impl From<CpChangeSpeedResCtrl> for u32 {
    fn from(value: CpChangeSpeedResCtrl) -> Self {
        value.0
    }
}