use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::timing::CpChangeSpeedTxDelay;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_SyncJumpWidth
///
/// Specifies the Synchronization Jump Width (SJW) used for CAN/CAN FD bit
/// timing. This parameter defines the maximum adjustment of the bit timing
/// during resynchronization to compensate for clock deviations between
/// network nodes. It is used to ensure reliable communication, especially at
/// higher communication speeds.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpSyncJumpWidth(pub u32);

impl From<CpSyncJumpWidth> for ComParamDefinition {
    fn from(value: CpSyncJumpWidth) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_SyncJumpWidth".to_string(),
            variant: value.0.into()
        }
    }
}

impl CpSyncJumpWidth {
    pub const ZERO: Self = Self(0);
}

impl From<CpSyncJumpWidth> for u32 {
    fn from(value: CpSyncJumpWidth) -> Self {
        value.0
    }
}

impl From<u32> for CpSyncJumpWidth {
    fn from(value: u32) -> Self {
        Self(value)
    }
}