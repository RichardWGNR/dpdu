use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::timing::CpChangeSpeedTxDelay;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CANFDBitSamplePoint
///
/// Specifies the bit sample point within a bit time. This parameter defines
/// the point at which the CAN controller samples the bus level to determine
/// the transmitted bit value. It is used to configure the bit timing and
/// ensure reliable communication on CAN and CAN FD networks.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanFdBitSamplePoint(pub u32);

impl From<CpCanFdBitSamplePoint> for ComParamDefinition {
    fn from(value: CpCanFdBitSamplePoint) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_CANFDBitSamplePoint".to_string(),
            variant: value.0.into()
        }
    }
}

impl CpCanFdBitSamplePoint {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanFdBitSamplePoint> for u32 {
    fn from(value: CpCanFdBitSamplePoint) -> Self {
        value.0
    }
}

impl From<u32> for CpCanFdBitSamplePoint {
    fn from(value: u32) -> Self {
        Self(value)
    }
}