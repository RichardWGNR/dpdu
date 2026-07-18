use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::com::CpChangeSpeedRate;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CanDataSizeOffset.
///
/// Defines the number of bytes to subtract from the total expected data length
/// specified in a First Frame message during multi-frame transmission.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanDataSizeOffset(pub u32);

impl From<CpCanDataSizeOffset> for ComParamDefinition {
    fn from(value: CpCanDataSizeOffset) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanDataSizeOffset".to_string(),
            variant: value.0.into()
        }
    }
}

impl CpCanDataSizeOffset {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanDataSizeOffset> for u32 {
    fn from(value: CpCanDataSizeOffset) -> Self {
        value.0
    }
}

impl From<u32> for CpCanDataSizeOffset {
    fn from(value: u32) -> Self {
        Self(value)
    }
}