use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_ChangeSpeedRate
///
/// Specifies the target data rate for a data rate change. This parameter
/// specifies the data rate to which communication shall be switched, for example,
/// in response to changing network conditions or a request to change the
/// communication channel data rate.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpChangeSpeedRate(pub u32);

impl From<CpChangeSpeedRate> for ComParamDefinition {
    fn from(value: CpChangeSpeedRate) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_ChangeSpeedRate".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpChangeSpeedRate {
    pub const ZERO: Self = Self(0);
}

impl From<CpChangeSpeedRate> for u32 {
    fn from(value: CpChangeSpeedRate) -> Self {
        value.0
    }
}

impl From<u32> for CpChangeSpeedRate {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
