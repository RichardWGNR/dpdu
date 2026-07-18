use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_RCByteOffset
///
/// Specifies the byte offset for NRC handling in the system. This parameter
/// defines the byte position from which the processing of data or messages
/// associated with the NRC shall start. It is used to ensure correct error
/// handling when the starting point within the transmitted data needs to be
/// specified.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpRcByteOffset(pub u32);

impl From<CpRcByteOffset> for ComParamDefinition {
    fn from(value: CpRcByteOffset) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RCByteOffset".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpRcByteOffset {
    pub const FIRST_BYTE: Self = Self(0);
    pub const LAST_BYTE: Self = Self(0xFFFFFFFF);
}

impl From<CpRcByteOffset> for u32 {
    fn from(value: CpRcByteOffset) -> Self {
        value.0
    }
}

impl From<u32> for CpRcByteOffset {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
