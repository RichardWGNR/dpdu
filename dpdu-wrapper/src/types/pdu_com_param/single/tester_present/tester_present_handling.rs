use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::single::tester_present::CpTesterPresentReqRsp;

/// CP_TesterPresentHandling
///
/// Specifies the handling method for the TesterPresent request. This parameter
/// defines how the system shall react to a request for the presence of the
/// diagnostic device (tester). It may include actions such as transmitting a
/// confirmation of tester presence, ignoring the request, or performing other
/// operations depending on the system configuration.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpTesterPresentHandling(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool
);

impl From<CpTesterPresentHandling> for ComParamDefinition {
    fn from(value: CpTesterPresentHandling) -> Self {
        ComParamDefinition {
            class: PduPc::TesterPresent,
            short_name: "CP_TesterPresentHandling".to_string(),
            variant: (value.0.then(|| 1).unwrap_or(0) as u32).into(),
        }
    }
}

impl CpTesterPresentHandling {
    pub const DISABLE: Self = CpTesterPresentHandling(false);
    pub const ENABLE: Self = CpTesterPresentHandling(true);
}

impl From<CpTesterPresentHandling> for u32 {
    fn from(value: CpTesterPresentHandling) -> Self {
        value.0.then(|| 1).unwrap_or(0)
    }
}

impl From<CpTesterPresentHandling> for bool {
    fn from(value: CpTesterPresentHandling) -> Self {
        value.0
    }
}

impl From<bool> for CpTesterPresentHandling {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpTesterPresentHandling {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}