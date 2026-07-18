use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CanRespUSDTId.
///
/// Specifies the CAN ID expected for received USDT response messages when
/// segmented data transfer is used.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanRespUsdtId(pub u32);

impl CpCanRespUsdtId {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanRespUsdtId> for ComParamDefinition {
    fn from(value: CpCanRespUsdtId) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUSDTId".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanRespUsdtId> for u32 {
    fn from(value: CpCanRespUsdtId) -> Self {
        value.0
    }
}