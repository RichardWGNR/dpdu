use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::timing::CpChangeSpeedTxDelay;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpCyclicRespTimeout(pub u32);

impl From<CpCyclicRespTimeout> for ComParamDefinition {
    fn from(value: CpCyclicRespTimeout) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_CyclicRespTimeout".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCyclicRespTimeout> for u32 {
    fn from(value: CpCyclicRespTimeout) -> Self {
        value.0
    }
}