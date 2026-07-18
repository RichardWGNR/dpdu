use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_CanRespUSDTFormat.
///
/// Specifies the received CAN format for unacknowledged segmented data
/// transfer (USDT) response messages. Defines the addressing scheme, CAN
/// identifier size, and segmentation format used for received response
/// messages.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanRespUsdtFormat(pub u32);

impl CpCanRespUsdtFormat {
    pub const NORMAL_UNSEGMENTED_11_BIT_WITHOUT_FC: Self = Self(4);

    pub const NORMAL_UNSEGMENTED_11_BIT_WITH_FC: Self = Self(5);

    pub const NORMAL_UNSEGMENTED_29_BIT_WITHOUT_FC: Self = Self(6);

    pub const NORMAL_UNSEGMENTED_29_BIT_WITH_FC: Self = Self(7);

    pub const EXT_UNSEGMENTED_11_BIT_WITHOUT_FC: Self = Self(12);

    pub const EXT_UNSEGMENTED_11_BIT_WITH_FC: Self = Self(13);

    pub const EXT_UNSEGMENTED_29_BIT_WITHOUT_FC: Self = Self(14);

    pub const EXT_UNSEGMENTED_29_BIT_WITH_FC: Self = Self(15);
}

impl From<CpCanRespUsdtFormat> for ComParamDefinition {
    fn from(value: CpCanRespUsdtFormat) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUSDTFormat".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanRespUsdtFormat> for u32 {
    fn from(value: CpCanRespUsdtFormat) -> Self {
        value.0
    }
}

impl From<u32> for CpCanRespUsdtFormat {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
