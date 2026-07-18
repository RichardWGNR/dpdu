use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpChangeSpeedCtrl;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CpChangeSpeedMessage(pub Vec<u8>);

impl From<CpChangeSpeedMessage> for ComParamDefinition {
    fn from(value: CpChangeSpeedMessage) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_ChangeSpeedMessage".to_string(),
            variant: value.0.into()
        }
    }
}

impl From<CpChangeSpeedMessage> for Vec<u8> {
    fn from(value: CpChangeSpeedMessage) -> Self {
        value.0.clone()
    }
}