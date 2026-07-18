use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::unique::CpCanPhysReqFormat;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CanPhysReqId
///
/// Specifies the CAN identifier (ID) of the message used for transmitting
/// physical diagnostic requests. This parameter defines the CAN frame ID
/// assigned to outgoing request messages sent to the ECU.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanFuncReqId(pub u32);

impl From<CpCanFuncReqId> for ComParamDefinition {
    fn from(value: CpCanFuncReqId) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanFuncReqId".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanFuncReqId> for u32 {
    fn from(value: CpCanFuncReqId) -> Self {
        value.0
    }
}

impl From<u32> for CpCanFuncReqId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}