use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CanFillerByte
///
/// Specifies the value used to fill unused bytes in a CAN frame when the
/// transmitted payload length is less than the maximum frame length. This
/// parameter defines the padding byte applied to CAN and CAN FD frames.
/// The value range is from 0x00 to 0xFF.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanFillerByte(pub u8);

impl From<CpCanFillerByte> for ComParamDefinition {
    fn from(value: CpCanFillerByte) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanFillerByte".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpCanFillerByte {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanFillerByte> for u8 {
    fn from(value: CpCanFillerByte) -> Self {
        value.0
    }
}

impl From<u8> for CpCanFillerByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
