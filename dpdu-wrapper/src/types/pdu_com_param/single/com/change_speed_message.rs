use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::com::CpChangeSpeedCtrl;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_ChangeSpeedMessage
///
/// Specifies the message to be transmitted for a data rate change.
/// This parameter specifies the byte sequence or data to be transmitted
/// to change the data rate in the system, for example, when the operating mode
/// of the communication interface or communication channel changes.
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpChangeSpeedMessage(pub Vec<u8>);

impl From<CpChangeSpeedMessage> for ComParamDefinition {
    fn from(value: CpChangeSpeedMessage) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_ChangeSpeedMessage".to_string(),
            variant: value.0.into()
        }
    }
}

impl CpChangeSpeedMessage {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl From<CpChangeSpeedMessage> for Vec<u8> {
    fn from(value: CpChangeSpeedMessage) -> Self {
        value.0.clone()
    }
}

impl From<&[u8]> for CpChangeSpeedMessage {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Vec<u8>> for CpChangeSpeedMessage {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}