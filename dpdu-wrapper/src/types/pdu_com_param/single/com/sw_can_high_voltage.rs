use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpSendRemoteFrame;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpSwCanHighVoltage(pub bool);

impl From<CpSwCanHighVoltage> for ComParamDefinition {
    fn from(value: CpSwCanHighVoltage) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_SwCan_HighVoltage".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl From<CpSwCanHighVoltage> for u32 {
    fn from(value: CpSwCanHighVoltage) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpSwCanHighVoltage> for bool {
    fn from(value: CpSwCanHighVoltage) -> Self {
        value.0
    }
}