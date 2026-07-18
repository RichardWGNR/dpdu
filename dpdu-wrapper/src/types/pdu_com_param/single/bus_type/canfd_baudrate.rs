use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CANFDBaudrate
///
/// Specifies the communication baud rate for the CAN network (for example,
/// 500 kbit/s or 1 Mbit/s). This parameter defines the communication speed
/// used on the bus. All nodes on the CAN network shall use the same baud rate
/// to ensure correct communication.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanFdBaudrate(pub u32);

impl From<CpCanFdBaudrate> for ComParamDefinition {
    fn from(value: CpCanFdBaudrate) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_CANFDBaudrate".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpCanFdBaudrate {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanFdBaudrate> for u32 {
    fn from(value: CpCanFdBaudrate) -> Self {
        value.0
    }
}

impl From<u32> for CpCanFdBaudrate {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
