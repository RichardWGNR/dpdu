use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_BlockSize.
///
/// Specifies the maximum number of Consecutive Frames (CF) that the sender may
/// transmit consecutively in a multi-frame transfer before waiting for the next
/// Flow Control (FC) frame from the receiver.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpBlockSize(pub u32);

impl From<CpBlockSize> for ComParamDefinition {
    fn from(value: CpBlockSize) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_BlockSize".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpBlockSize {
    pub const ZERO: Self = Self(0);
}

impl From<CpBlockSize> for u32 {
    fn from(value: CpBlockSize) -> Self {
        value.0
    }
}

impl From<u32> for CpBlockSize {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
