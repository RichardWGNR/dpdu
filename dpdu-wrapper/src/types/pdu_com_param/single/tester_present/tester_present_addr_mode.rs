use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::err_hdl::CpRepeatReqCountApp;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_TesterPresentAddrMode
///
/// Specifies the addressing mode for the TesterPresent request. This parameter
/// defines how addressing shall be performed when transmitting a TesterPresent
/// request to the diagnostic device in the network. Different addressing modes
/// may be used, such as physical or functional addressing of the device.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpTesterPresentAddrMode(pub u32);

impl From<CpTesterPresentAddrMode> for ComParamDefinition {
    fn from(value: CpTesterPresentAddrMode) -> Self {
        ComParamDefinition {
            class: PduPc::TesterPresent,
            short_name: "CP_TesterPresentAddrMode".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpTesterPresentAddrMode {
    pub const PHYSICAL: Self = CpTesterPresentAddrMode(0);
    pub const FUNCTIONAL: Self = CpTesterPresentAddrMode(1);
}

impl From<CpTesterPresentAddrMode> for u32 {
    fn from(value: CpTesterPresentAddrMode) -> Self {
        value.0
    }
}

impl From<u32> for CpTesterPresentAddrMode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}