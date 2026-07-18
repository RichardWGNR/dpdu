use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::single::timing::CpModifyTiming;

/// CP_Loopback
///
/// Specifies whether the loopback mode for data transmission is enabled.
/// This parameter indicates whether the system shall operate in loopback mode,
/// in which transmitted data is immediately returned, for communication channel
/// or device testing and diagnostics.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpLoopback(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool
);

impl From<CpLoopback> for ComParamDefinition {
    fn from(value: CpLoopback) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_Loopback".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl CpLoopback {
    pub const DISABLE: Self = Self(false);
    pub const ENABLE: Self = Self(true);
}

impl From<CpLoopback> for u32 {
    fn from(value: CpLoopback) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpLoopback> for bool {
    fn from(value: CpLoopback) -> Self {
        value.0
    }
}

impl From<bool> for CpLoopback {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpLoopback {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}