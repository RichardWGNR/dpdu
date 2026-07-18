use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::com::CpChangeSpeedRate;
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};

/// CP_RC23Handling
///
/// Specifies the handling method for NRC 0x23 in the system. This parameter
/// defines how the system shall react when NRC 0x23 occurs, for example,
/// whether to retry the request, enter a safe state, or perform other actions
/// according to the predefined error handling logic.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpRc23Handling(pub u32);

impl From<CpRc23Handling> for ComParamDefinition {
    fn from(value: CpRc23Handling) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RC23Handling".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpRc23Handling {
    pub const DISABLE: Self = CpRc23Handling(0);
    pub const CONTINUE_UNTIL_RC23_TIMEOUT: Self = CpRc23Handling(1);
    pub const CONTINUE_UNLIMITED: Self = CpRc23Handling(2);
}

impl From<CpRc23Handling> for u32 {
    fn from(value: CpRc23Handling) -> Self {
        value.0
    }
}

impl From<u32> for CpRc23Handling {
    fn from(value: u32) -> Self {
        Self(value)
    }
}