use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::com::CpSendRemoteFrame;
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::single::tester_present::CpTesterPresentHandling;

/// CP_SwCan_HighVoltage
///
/// Specifies whether high voltage is available for the SWCAN bus. This parameter
/// indicates whether high voltage is used for operation with the SWCAN
/// (Single Wire CAN) bus. It is typically used for configuring interfaces that
/// support operation with high voltage levels on the communication bus.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpSwCanHighVoltage(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool
);

impl From<CpSwCanHighVoltage> for ComParamDefinition {
    fn from(value: CpSwCanHighVoltage) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_SwCan_HighVoltage".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl CpSwCanHighVoltage {
    pub const DISABLE: Self = Self(false);
    pub const ENABLE: Self = Self(true);
}


impl From<CpSwCanHighVoltage> for u32 {
    fn from(value: CpSwCanHighVoltage) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpSwCanHighVoltage> for bool {
    fn from(value: CpSwCanHighVoltage) -> Self {
        value.0
    }
}

impl From<bool> for CpSwCanHighVoltage {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpSwCanHighVoltage {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}