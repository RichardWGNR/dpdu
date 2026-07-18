use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CanRespUSDTExtAddr.
///
/// Specifies the extended address value used for receiving USDT response
/// messages when extended addressing is enabled.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanRespUsdtExtAddr(pub u32);

impl From<CpCanRespUsdtExtAddr> for ComParamDefinition {
    fn from(value: CpCanRespUsdtExtAddr) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUSDTExtAddr".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpCanRespUsdtExtAddr {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanRespUsdtExtAddr> for u32 {
    fn from(value: CpCanRespUsdtExtAddr) -> Self {
        value.0
    }
}

impl From<u32> for CpCanRespUsdtExtAddr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
