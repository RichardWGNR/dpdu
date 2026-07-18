use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::timing::CpP3Phys;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[derive(Debug, Copy, Clone)]
pub struct CpCanPhysReqExtAddr(pub u32);

impl From<CpCanPhysReqExtAddr> for ComParamDefinition {
    fn from(value: CpCanPhysReqExtAddr) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanPhysReqExtAddr".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanPhysReqExtAddr> for u32 {
    fn from(value: CpCanPhysReqExtAddr) -> Self {
        value.0
    }
}