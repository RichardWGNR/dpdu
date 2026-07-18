use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CanPhysReqId
///
/// Specifies the CAN identifier (ID) of the message used for transmitting
/// physical diagnostic requests. This parameter defines the CAN frame ID
/// assigned to outgoing request messages sent to the ECU.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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

impl From<u32> for CpCanPhysReqId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
