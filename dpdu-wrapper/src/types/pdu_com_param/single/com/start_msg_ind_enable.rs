use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::single::err_hdl::CpSuspendQueueOnError;

/// CP_StartMsgIndEnable
///
/// Specifies whether the start message indication transmission is enabled.
/// This parameter indicates whether the system shall transmit a dedicated
/// message or signal at the start of a specific operation or process. It is
/// used to notify the system or other devices about the beginning of task
/// execution or data transmission.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpStartMsgIndEnable(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool
);

impl From<CpStartMsgIndEnable> for ComParamDefinition {
    fn from(value: CpStartMsgIndEnable) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_StartMsgIndEnable".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl CpStartMsgIndEnable {
    pub const DISABLE: Self = Self(false);
    pub const ENABLE: Self = Self(true);
}

impl From<CpStartMsgIndEnable> for u32 {
    fn from(value: CpStartMsgIndEnable) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpStartMsgIndEnable> for bool {
    fn from(value: CpStartMsgIndEnable) -> Self {
        value.0
    }
}

impl From<bool> for CpStartMsgIndEnable {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpStartMsgIndEnable {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}