use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_RC21Handling
///
/// Specifies the handling method for NRC 0x21 in the system. This parameter
/// defines how the system shall react to NRC 0x21, for example, which method
/// or procedure shall be used to handle or recover from the condition. It may
/// include actions such as retrying the request, entering a safe state, or
/// performing other predefined operations.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpRc21Handling(pub u32);

impl From<CpRc21Handling> for ComParamDefinition {
    fn from(value: CpRc21Handling) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RC21Handling".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpRc21Handling {
    pub const DISABLE: Self = CpRc21Handling(0);
    pub const CONTINUE_UNTIL_RC21_TIMEOUT: Self = CpRc21Handling(1);
    pub const CONTINUE_UNLIMITED: Self = CpRc21Handling(2);
}

impl From<CpRc21Handling> for u32 {
    fn from(value: CpRc21Handling) -> Self {
        value.0
    }
}

impl From<u32> for CpRc21Handling {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
