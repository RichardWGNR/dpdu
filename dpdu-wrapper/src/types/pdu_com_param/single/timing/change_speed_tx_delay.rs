use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpChangeSpeedRate;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpChangeSpeedTxDelay(pub u32);

impl From<CpChangeSpeedTxDelay> for ComParamDefinition {
    fn from(value: CpChangeSpeedTxDelay) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_ChangeSpeedTxDelay".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpChangeSpeedTxDelay> for u32 {
    fn from(value: CpChangeSpeedTxDelay) -> Self {
        value.0
    }
}