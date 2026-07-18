use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_BlockSizeOverride.
///
/// Defines the number of Consecutive Frames (CF) that can be transmitted before
/// the sender must wait for a Flow Control (FC) frame from the receiver during
/// an ISO-TP multi-frame transfer.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpBlockSizeOverride(pub u32);

impl From<CpBlockSizeOverride> for ComParamDefinition {
    fn from(value: CpBlockSizeOverride) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_BlockSizeOverride".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpBlockSizeOverride> for u32 {
    fn from(value: CpBlockSizeOverride) -> Self {
        value.0
    }
}

impl From<u32> for CpBlockSizeOverride {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
