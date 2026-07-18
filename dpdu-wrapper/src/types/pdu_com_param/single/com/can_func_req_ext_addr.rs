use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CanFuncReqExtAddr
///
/// Specifies the extended address used for functional CAN diagnostic requests.
/// This parameter defines the address byte included in CAN frames when using
/// extended addressing mode for ISO-TP communication. It is used to identify
/// the target ECU in networks where extended addressing is required.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanFuncReqExtAddr(pub u32);

impl From<CpCanFuncReqExtAddr> for ComParamDefinition {
    fn from(value: CpCanFuncReqExtAddr) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanFuncReqExtAddr".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpCanFuncReqExtAddr {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanFuncReqExtAddr> for u32 {
    fn from(value: CpCanFuncReqExtAddr) -> Self {
        value.0
    }
}

impl From<u32> for CpCanFuncReqExtAddr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
