use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_RepeatReqCountApp
///
/// Specifies the number of request retries for the application in case of
/// failure. This parameter defines how many times the system shall attempt to
/// execute the request again after unsuccessful attempts before considering
/// the operation permanently failed.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpRepeatReqCountApp(pub u32);

impl From<CpRepeatReqCountApp> for ComParamDefinition {
    fn from(value: CpRepeatReqCountApp) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RepeatReqCountApp".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpRepeatReqCountApp {
    pub const ZERO: Self = Self(0);
}

impl From<CpRepeatReqCountApp> for u32 {
    fn from(value: CpRepeatReqCountApp) -> Self {
        value.0
    }
}

impl From<u32> for CpRepeatReqCountApp {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
