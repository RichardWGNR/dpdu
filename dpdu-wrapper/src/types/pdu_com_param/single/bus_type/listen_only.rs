use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_ListenOnly
///
/// Specifies whether the CAN controller shall operate in Listen-Only mode.
/// In this mode, the controller monitors and receives messages from the bus
/// without transmitting frames, acknowledgements, or error flags. It is used
/// for passive network monitoring and diagnostics without influencing bus
/// communication.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpListenOnly(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool,
);

impl From<CpListenOnly> for ComParamDefinition {
    fn from(value: CpListenOnly) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_ListenOnly".to_string(),
            variant: (if value.0 { 1u32 } else { 0 }).into(),
        }
    }
}

impl CpListenOnly {
    pub const DISABLE: Self = CpListenOnly(false);
    pub const ENABLE: Self = CpListenOnly(true);
}

impl From<CpListenOnly> for u32 {
    fn from(value: CpListenOnly) -> Self {
        value.0.then(|| 1).unwrap_or(0)
    }
}

impl From<CpListenOnly> for bool {
    fn from(value: CpListenOnly) -> Self {
        value.0
    }
}

impl From<bool> for CpListenOnly {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpListenOnly {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}
