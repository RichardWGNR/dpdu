use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CanRespUUDTFormat
///
/// Specifies the format of CAN physical response messages. This parameter
/// defines how response data shall be structured, packed, and organized in
/// CAN frames during physical diagnostic communication.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanRespUudtFormat(pub u32);

impl CpCanRespUudtFormat {
    pub const NORMAL_UNSEGMENTED_11_BIT: Self = Self(0);

    pub const NORMAL_UNSEGMENTED_29_BIT: Self = Self(2);

    pub const EXT_UNSEGMENTED_11_BIT: Self = Self(8);

    pub const EXT_UNSEGMENTED_29_BIT: Self = Self(10);
}

impl From<CpCanRespUudtFormat> for ComParamDefinition {
    fn from(value: CpCanRespUudtFormat) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUUDTFormat".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanRespUudtFormat> for u32 {
    fn from(value: CpCanRespUudtFormat) -> Self {
        value.0
    }
}

impl From<u32> for CpCanRespUudtFormat {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
