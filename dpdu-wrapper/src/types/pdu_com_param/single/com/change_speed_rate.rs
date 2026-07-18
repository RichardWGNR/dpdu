use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpChangeSpeedRate(pub u32);

impl From<CpChangeSpeedRate> for ComParamDefinition {
    fn from(value: CpChangeSpeedRate) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_ChangeSpeedRate".to_string(),
            variant: value.0.into()
        }
    }
}

impl From<CpChangeSpeedRate> for u32 {
    fn from(value: CpChangeSpeedRate) -> Self {
        value.0
    }
}