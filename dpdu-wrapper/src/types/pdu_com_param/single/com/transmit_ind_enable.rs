use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpSwCanHighVoltage;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpTransmitIndEnable(pub bool);

impl From<CpTransmitIndEnable> for ComParamDefinition {
    fn from(value: CpTransmitIndEnable) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_TransmitIndEnable".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl From<CpTransmitIndEnable> for u32 {
    fn from(value: CpTransmitIndEnable) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpTransmitIndEnable> for bool {
    fn from(value: CpTransmitIndEnable) -> Self {
        value.0
    }
}