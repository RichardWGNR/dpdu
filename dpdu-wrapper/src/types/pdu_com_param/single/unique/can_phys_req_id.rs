use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::unique::CpCanPhysReqFormat;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[derive(Debug, Copy, Clone)]
pub struct CpCanPhysReqId(pub u32);

impl From<CpCanPhysReqId> for ComParamDefinition {
    fn from(value: CpCanPhysReqId) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanPhysReqId".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanPhysReqId> for u32 {
    fn from(value: CpCanPhysReqId) -> Self {
        value.0
    }
}