use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::com::CpChangeSpeedRate;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_ChangeSpeedResCtrl
///
/// Specifies whether the VCI shall wait for a response from the ECU after a
/// Change Speed command (e.g. 0x10 or 0x85) before switching to the new
/// communication baud rate.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpChangeSpeedResCtrl(pub u32);

impl From<CpChangeSpeedResCtrl> for ComParamDefinition {
    fn from(value: CpChangeSpeedResCtrl) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_ChangeSpeedResCtrl".to_string(),
            variant: value.0.into()
        }
    }
}

impl CpChangeSpeedResCtrl {
    pub const NOT_USED: Self = Self(0);
    pub const AC_RESISTOR: Self = Self(1);
    pub const _60_OHM_RESISTOR: Self = Self(2);
    pub const _120_OHM_RESISTOR: Self = Self(3);
    pub const SWCAN_RESISTOR: Self = Self(4);
    pub const UNLOAD_RESISTOR: Self = Self(128);
}

impl From<CpChangeSpeedResCtrl> for u32 {
    fn from(value: CpChangeSpeedResCtrl) -> Self {
        value.0
    }
}

impl From<u32> for CpChangeSpeedResCtrl {
    fn from(value: u32) -> Self {
        Self(value)
    }
}