use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_TesterPresentSendType
///
/// Specifies the message type that shall be transmitted by the system in
/// response to the TesterPresent request. This parameter defines the format
/// or type of data that shall be used for the response to the presence request
/// of the diagnostic device (tester), for example, a standard or extended
/// message type.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpTesterPresentSendType(pub u32);

impl From<CpTesterPresentSendType> for ComParamDefinition {
    fn from(value: CpTesterPresentSendType) -> Self {
        ComParamDefinition {
            class: PduPc::TesterPresent,
            short_name: "CP_TesterPresentSendType".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpTesterPresentSendType {
    pub const PERIODIC: Self = CpTesterPresentSendType(0);
    pub const ON_IDLE: Self = CpTesterPresentSendType(1);
}

impl From<CpTesterPresentSendType> for u32 {
    fn from(value: CpTesterPresentSendType) -> Self {
        value.0
    }
}

impl From<u32> for CpTesterPresentSendType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
