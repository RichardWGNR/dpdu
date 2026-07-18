use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::timing::CpChangeSpeedTxDelay;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_Baudrate
///
/// Specifies the communication baud rate for the CAN network (for example,
/// 500 kbit/s or 1 Mbit/s). This parameter defines the communication speed
/// used on the bus. All nodes on the CAN network shall use the same baud rate
/// to ensure correct communication.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpBaudrate(pub u32);

impl From<CpBaudrate> for ComParamDefinition {
    fn from(value: CpBaudrate) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_Baudrate".to_string(),
            variant: value.0.into()
        }
    }
}

impl CpBaudrate {
    pub const ZERO: Self = Self(0);
}

impl From<CpBaudrate> for u32 {
    fn from(value: CpBaudrate) -> Self {
        value.0
    }
}

impl From<u32> for CpBaudrate {
    fn from(value: u32) -> Self {
        Self(value)
    }
}