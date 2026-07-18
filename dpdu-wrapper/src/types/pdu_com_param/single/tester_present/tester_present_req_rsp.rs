use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::single::com::CpTransmitIndEnable;

/// CP_TesterPresentReqRsp
///
/// Specifies whether the system shall transmit a response to the TesterPresent
/// request. This parameter indicates whether the system shall respond to
/// requests for the presence of the diagnostic device (tester) on the network.
/// The value may enable or disable the response depending on the system
/// configuration for diagnostic device communication.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpTesterPresentReqRsp(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool
);

impl From<CpTesterPresentReqRsp> for ComParamDefinition {
    fn from(value: CpTesterPresentReqRsp) -> Self {
        ComParamDefinition {
            class: PduPc::TesterPresent,
            short_name: "CP_TesterPresentReqRsp".to_string(),
            variant: (value.0.then(|| 1).unwrap_or(0) as u32).into(),
        }
    }
}

impl CpTesterPresentReqRsp {
    pub const NO_RESPONSE: Self = CpTesterPresentReqRsp(false);
    pub const RESPONSE_EXPECTED: Self = CpTesterPresentReqRsp(true);
}

impl From<CpTesterPresentReqRsp> for u32 {
    fn from(value: CpTesterPresentReqRsp) -> Self {
        value.0.then(|| 1).unwrap_or(0)
    }
}

impl From<CpTesterPresentReqRsp> for bool {
    fn from(value: CpTesterPresentReqRsp) -> Self {
        value.0
    }
}

impl From<bool> for CpTesterPresentReqRsp {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpTesterPresentReqRsp {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}