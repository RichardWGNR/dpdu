use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::unique::CpCanPhysReqExtAddr;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CanRespUUDTExtAddr
///
/// Specifies the 29-bit extended address used for CAN response messages.
/// This parameter defines the extended CAN identifier format for response
/// messages, enabling communication with ECUs that use extended addressing
/// instead of the standard 11-bit CAN identifier format.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanRespUudtExtAddr(pub u32);

impl From<CpCanRespUudtExtAddr> for ComParamDefinition {
    fn from(value: CpCanRespUudtExtAddr) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUUDTExtAddr".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpCanRespUudtExtAddr {
    pub const ZERO: Self = Self(0);
    pub const NOT_USED: Self = Self(0xFFFFFFFF);
}

impl From<CpCanRespUudtExtAddr> for u32 {
    fn from(value: CpCanRespUudtExtAddr) -> Self {
        value.0
    }
}

impl From<u32> for CpCanRespUudtExtAddr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}