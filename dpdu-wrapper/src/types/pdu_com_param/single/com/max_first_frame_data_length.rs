use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_MaxFirstFrameDataLength.
///
/// Defines the maximum amount of data that can be transmitted in a First Frame
/// (FF).
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpMaxFirstFrameDataLength(pub u32);

impl From<CpMaxFirstFrameDataLength> for ComParamDefinition {
    fn from(value: CpMaxFirstFrameDataLength) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_MaxFirstFrameDataLength".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpMaxFirstFrameDataLength> for u32 {
    fn from(value: CpMaxFirstFrameDataLength) -> Self {
        value.0
    }
}

impl From<u32> for CpMaxFirstFrameDataLength {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
