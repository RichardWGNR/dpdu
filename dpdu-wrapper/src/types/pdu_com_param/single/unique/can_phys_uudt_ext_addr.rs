use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::unique::CpCanPhysReqId;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[derive(Debug, Copy, Clone)]
pub struct CpCanRespUudtExtAddr(pub u32);

impl From<CpCanRespUudtExtAddr> for ComParamDefinition {
    fn from(value: CpCanRespUudtExtAddr) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUUDTExtAddr".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanRespUudtExtAddr> for u32 {
    fn from(value: CpCanRespUudtExtAddr) -> Self {
        value.0
    }
}