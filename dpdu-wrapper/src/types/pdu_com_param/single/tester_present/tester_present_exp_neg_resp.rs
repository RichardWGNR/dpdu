use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_TesterPresentExpNegResp
///
/// Specifies the expected negative response for the TesterPresent request.
/// This parameter defines the byte sequence that shall be received if the
/// tester is not present or does not respond to the TesterPresent request on
/// the network.
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpTesterPresentExpNegResp(pub Vec<u8>);

impl From<CpTesterPresentExpNegResp> for ComParamDefinition {
    fn from(value: CpTesterPresentExpNegResp) -> Self {
        ComParamDefinition {
            class: PduPc::TesterPresent,
            short_name: "CP_TesterPresentExpNegResp".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpTesterPresentExpNegResp {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl From<CpTesterPresentExpNegResp> for Vec<u8> {
    fn from(value: CpTesterPresentExpNegResp) -> Self {
        value.0
    }
}

impl From<Vec<u8>> for CpTesterPresentExpNegResp {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&[u8]> for CpTesterPresentExpNegResp {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}
