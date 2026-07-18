use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_TesterPresentExpPosResp
///
/// Specifies the expected positive response for the TesterPresent request.
/// This parameter defines the byte sequence that shall be received in case of
/// successful confirmation of the presence of the diagnostic device (tester)
/// on the network.
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpTesterPresentExpPosResp(pub Vec<u8>);

impl From<CpTesterPresentExpPosResp> for ComParamDefinition {
    fn from(value: CpTesterPresentExpPosResp) -> Self {
        ComParamDefinition {
            class: PduPc::TesterPresent,
            short_name: "CP_TesterPresentExpPosResp".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpTesterPresentExpPosResp {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl From<CpTesterPresentExpPosResp> for Vec<u8> {
    fn from(value: CpTesterPresentExpPosResp) -> Self {
        value.0
    }
}

impl From<Vec<u8>> for CpTesterPresentExpPosResp {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&[u8]> for CpTesterPresentExpPosResp {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}
