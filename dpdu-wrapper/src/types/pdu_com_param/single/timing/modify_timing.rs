use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_ModifyTiming
///
/// Specifies whether standard timing parameters shall be modified. This parameter
/// defines whether timing values, such as response time or data transmission
/// time, shall be adjusted. A non-zero value enables modification of timing
/// intervals during message transmission, for example, in CAN or UDS communication.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpModifyTiming(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool,
);

impl From<CpModifyTiming> for ComParamDefinition {
    fn from(value: CpModifyTiming) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_ModifyTiming".to_string(),
            variant: (value.0.then(|| 1).unwrap_or(0) as u32).into(),
        }
    }
}

impl CpModifyTiming {
    pub const DISABLE: Self = CpModifyTiming(false);
    pub const ENABLE: Self = CpModifyTiming(true);
}

impl From<CpModifyTiming> for u32 {
    fn from(value: CpModifyTiming) -> Self {
        value.0.then(|| 1).unwrap_or(0)
    }
}

impl From<CpModifyTiming> for bool {
    fn from(value: CpModifyTiming) -> Self {
        value.0
    }
}

impl From<bool> for CpModifyTiming {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpModifyTiming {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}
