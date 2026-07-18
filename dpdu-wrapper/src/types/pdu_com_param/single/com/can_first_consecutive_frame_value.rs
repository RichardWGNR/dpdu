use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CanFirstConsecutiveFrameValue.
///
/// Defines the value (field) that will be used as the first Consecutive Frame
/// (CF) when data transmission in multiple frames begins as part of a
/// multi-frame message.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanFirstConsecutiveFrameValue(pub u32);

impl From<CpCanFirstConsecutiveFrameValue> for ComParamDefinition {
    fn from(value: CpCanFirstConsecutiveFrameValue) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanFirstConsecutiveFrameValue".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpCanFirstConsecutiveFrameValue {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanFirstConsecutiveFrameValue> for u32 {
    fn from(value: CpCanFirstConsecutiveFrameValue) -> Self {
        value.0
    }
}

impl From<u32> for CpCanFirstConsecutiveFrameValue {
    fn from(value: u32) -> Self {
        Self(value)
    }
}