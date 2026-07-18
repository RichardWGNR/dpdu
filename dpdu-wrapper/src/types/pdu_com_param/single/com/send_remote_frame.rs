use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_SendRemoteFrame
///
/// Specifies whether the transmission of CAN Remote Frames is enabled.
///
/// A Remote Frame is a CAN frame type used to request data from another node
/// without carrying any payload data. It allows a CAN node to request the
/// transmission of a specific message from another ECU without sending the
/// actual data itself.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpSendRemoteFrame(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool,
);

impl From<CpSendRemoteFrame> for ComParamDefinition {
    fn from(value: CpSendRemoteFrame) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_SendRemoteFrame".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl CpSendRemoteFrame {
    pub const DISABLE: Self = Self(false);
    pub const ENABLE: Self = Self(true);
}

impl From<CpSendRemoteFrame> for u32 {
    fn from(value: CpSendRemoteFrame) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpSendRemoteFrame> for bool {
    fn from(value: CpSendRemoteFrame) -> Self {
        value.0
    }
}

impl From<bool> for CpSendRemoteFrame {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpSendRemoteFrame {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}
