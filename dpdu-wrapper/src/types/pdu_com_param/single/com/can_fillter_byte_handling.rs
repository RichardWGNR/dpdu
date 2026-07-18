use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::single::com::CpTransmitIndEnable;

/// CP_CanFillerByteHandling
///
/// Specifies the handling rules for filler bytes in ISO-TP messages. This
/// parameter defines how padding bytes shall be processed during transmission
/// and reception when the payload length is less than the maximum CAN frame
/// size (8 bytes for Classical CAN and up to 64 bytes for CAN FD).
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanFillerByteHandling(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool
);

impl From<CpCanFillerByteHandling> for ComParamDefinition {
    fn from(value: CpCanFillerByteHandling) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanFillerByteHandling".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl CpCanFillerByteHandling {
    pub const DISABLE: Self = Self(false);
    pub const ENABLE: Self = Self(true);
}


impl From<CpCanFillerByteHandling> for u32 {
    fn from(value: CpCanFillerByteHandling) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpCanFillerByteHandling> for bool {
    fn from(value: CpCanFillerByteHandling) -> Self {
        value.0
    }
}

impl From<bool> for CpCanFillerByteHandling {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpCanFillerByteHandling {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}