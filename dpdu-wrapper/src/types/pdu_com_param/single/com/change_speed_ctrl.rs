use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_ChangeSpeedCtrl
///
/// Indicates whether data rate change control is enabled. This parameter
/// specifies whether the system shall control changes to the data rate, for example,
/// depending on the current network conditions or other factors.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpChangeSpeedCtrl(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool,
);

impl From<CpChangeSpeedCtrl> for ComParamDefinition {
    fn from(value: CpChangeSpeedCtrl) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_ChangeSpeedCtrl".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl CpChangeSpeedCtrl {
    pub const NAME: &'static str = "CP_ChangeSpeedCtrl";

    pub const DISABLE: Self = Self(false);
    pub const ENABLE: Self = Self(true);
}

impl From<CpChangeSpeedCtrl> for u32 {
    fn from(value: CpChangeSpeedCtrl) -> Self {
        value.0.then(|| 1).unwrap_or(0)
    }
}

impl From<CpChangeSpeedCtrl> for bool {
    fn from(value: CpChangeSpeedCtrl) -> Self {
        value.0
    }
}

impl From<bool> for CpChangeSpeedCtrl {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
