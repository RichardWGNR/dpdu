use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_BitSamplePoint
///
/// Specifies the bit sample point within a bit time. This parameter defines
/// the point at which the CAN controller samples the bus level to determine
/// the transmitted bit value. It is used to configure the bit timing and
/// ensure reliable communication on CAN and CAN FD networks.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpBitSamplePoint(pub u32);

impl From<CpBitSamplePoint> for ComParamDefinition {
    fn from(value: CpBitSamplePoint) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_BitSamplePoint".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpBitSamplePoint {
    pub const ZERO: Self = Self(0);
}

impl From<CpBitSamplePoint> for u32 {
    fn from(value: CpBitSamplePoint) -> Self {
        value.0
    }
}

impl From<u32> for CpBitSamplePoint {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
