use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::err_hdl::CpRc23Handling;
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::single::com::CpStartMsgIndEnable;

/// CP_RC78Handling
///
/// Specifies the handling method for NRC 0x78 in the system. This parameter
/// defines how the system shall react when NRC 0x78 occurs, for example,
/// whether to retry the request, use alternative methods, or perform other
/// actions depending on the configured error handling strategy.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpRc78Handling(pub u32);

impl From<CpRc78Handling> for ComParamDefinition {
    fn from(value: CpRc78Handling) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RC78Handling".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpRc78Handling {
    pub const DISABLE: Self = CpRc78Handling(0);
    pub const CONTINUE_UNTIL_RC78_TIMEOUT: Self = CpRc78Handling(1);
    pub const CONTINUE_UNLIMITED: Self = CpRc78Handling(2);
}

impl From<CpRc78Handling> for u32 {
    fn from(value: CpRc78Handling) -> Self {
        value.0
    }
}

impl From<u32> for CpRc78Handling {
    fn from(value: u32) -> Self {
        Self(value)
    }
}