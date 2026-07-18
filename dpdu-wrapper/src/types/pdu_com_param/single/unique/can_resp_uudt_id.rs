use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CanRespUUDTId
///
/// Specifies the CAN identifier (ID) used for physical diagnostic response
/// messages. This parameter defines the CAN frame ID assigned to outgoing
/// response messages, allowing responses from different ECUs or message
/// types to be uniquely identified on the CAN network.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanRespUudtId(pub u32);

impl CpCanRespUudtId {
    pub const NOT_USED: CpCanRespUudtId = CpCanRespUudtId(0xFFFFFFFF);
}

impl From<CpCanRespUudtId> for ComParamDefinition {
    fn from(value: CpCanRespUudtId) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUUDTId".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanRespUudtId> for u32 {
    fn from(value: CpCanRespUudtId) -> Self {
        value.0
    }
}

impl From<u32> for CpCanRespUudtId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
