use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::unique::CpCanRespUudtId;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpCanFillerByte(pub u8);

impl From<CpCanFillerByte> for ComParamDefinition {
    fn from(value: CpCanFillerByte) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanFillerByte".to_string(),
            variant: (value.0 as u32).into(),
        }
    }
}

impl From<CpCanFillerByte> for u32 {
    fn from(value: CpCanFillerByte) -> Self {
        value.0 as _
    }
}