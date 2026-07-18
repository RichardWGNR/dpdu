use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::tester_present::CpTesterPresentExpPosResp;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_TesterPresentMessage
///
/// Specifies the message that shall be transmitted by the system in response
/// to the TesterPresent request. This parameter defines the specific data or
/// byte sequence that shall be sent in response to a request for the presence
/// of the diagnostic device (tester) on the network.
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpTesterPresentMessage(pub Vec<u8>);

impl From<CpTesterPresentMessage> for ComParamDefinition {
    fn from(value: CpTesterPresentMessage) -> Self {
        ComParamDefinition {
            class: PduPc::TesterPresent,
            short_name: "CP_TesterPresentMessage".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpTesterPresentMessage> for Vec<u8> {
    fn from(value: CpTesterPresentMessage) -> Self {
        value.0
    }
}

impl From<Vec<u8>> for CpTesterPresentMessage {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&[u8]> for CpTesterPresentMessage {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}