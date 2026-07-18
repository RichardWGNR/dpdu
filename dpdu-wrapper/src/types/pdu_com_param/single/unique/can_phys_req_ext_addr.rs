use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CanPhysReqExtAddr
///
/// Specifies the extended address used for physical CAN diagnostic requests.
/// This parameter defines the address byte included in CAN frames when using
/// extended addressing mode for ISO-TP communication. It is used to identify
/// the target ECU in networks where extended addressing is required.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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

impl CpCanPhysReqExtAddr {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanPhysReqExtAddr> for u32 {
    fn from(value: CpCanPhysReqExtAddr) -> Self {
        value.0
    }
}

impl From<u32> for CpCanPhysReqExtAddr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
