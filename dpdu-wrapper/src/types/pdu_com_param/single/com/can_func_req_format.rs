use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CanPhysReqFormat
///
/// Specifies the request format used for physical CAN diagnostic communication.
/// This parameter defines the CAN addressing format and frame structure used
/// when transmitting physical diagnostic requests to the ECU.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanFuncReqFormat(pub u32);

impl From<CpCanFuncReqFormat> for ComParamDefinition {
    fn from(value: CpCanFuncReqFormat) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanFuncReqFormat".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpCanFuncReqFormat {
    pub const NORMAL_UNSEGMENTED_11_BIT: Self = Self(0);
    pub const NORMAL_UNSEGMENTED_29_BIT: Self = Self(2);

    pub const NORMAL_SEGMENTED_11_BIT: Self = Self(4);
    pub const NORMAL_SEGMENTED_11_BIT_WITH_FC: Self = Self(5);

    pub const NORMAL_SEGMENTED_29_BIT: Self = Self(6);
    pub const NORMAL_SEGMENTED_29_BIT_WITH_FC: Self = Self(7);

    pub const EXT_UNSEGMENTED_11_BIT: Self = Self(8);
    pub const EXT_UNSEGMENTED_29_BIT: Self = Self(10);

    pub const EXT_SEGMENTED_11_BIT: Self = Self(12);
    pub const EXT_SEGMENTED_11_BIT_WITH_FC: Self = Self(13);

    pub const EXT_SEGMENTED_29_BIT: Self = Self(14);
    pub const EXT_SEGMENTED_29_BIT_WITH_FC: Self = Self(15);

    pub const NORMAL_UNSEGMENTED_11_BIT_WITHOUT_PADDING: Self = Self(32);
    pub const NORMAL_UNSEGMENTED_29_BIT_WITHOUT_PADDING: Self = Self(34);

    pub const NORMAL_SEGMENTED_11_BIT_WITHOUT_FC_AND_WITHOUT_PADDING: Self = Self(36);
    pub const NORMAL_SEGMENTED_11_BIT_WITH_FC_AND_WITHOUT_PADDING: Self = Self(37);

    pub const NORMAL_SEGMENTED_29_BIT_WITHOUT_FC_AND_WITHOUT_PADDING: Self = Self(38);
    pub const NORMAL_SEGMENTED_29_BIT_WITH_FC_AND_WITHOUT_PADDING: Self = Self(39);

    pub const EXT_UNSEGMENTED_11_BIT_WITHOUT_PADDING: Self = Self(40);
    pub const EXT_UNSEGMENTED_29_BIT_WITHOUT_PADDING: Self = Self(42);

    pub const EXT_SEGMENTED_11_BIT_WITHOUT_FC_AND_WITHOUT_PADDING: Self = Self(44);
    pub const EXT_SEGMENTED_11_BIT_WITH_FC_AND_WITHOUT_PADDING: Self = Self(45);

    pub const EXT_SEGMENTED_29_BIT_WITHOUT_FC_AND_WITHOUT_PADDING: Self = Self(46);
    pub const EXT_SEGMENTED_29_BIT_WITH_FC_AND_WITHOUT_PADDING: Self = Self(47);

    pub const NORMAL_UNSEGMENTED_11_BIT_WITH_PADDING: Self = Self(48);
    pub const NORMAL_UNSEGMENTED_29_BIT_WITH_PADDING: Self = Self(50);

    pub const NORMAL_SEGMENTED_11_BIT_WITHOUT_FC_AND_WITH_PADDING: Self = Self(52);
    pub const NORMAL_SEGMENTED_11_BIT_WITH_FC_AND_WITH_PADDING: Self = Self(53);

    pub const NORMAL_SEGMENTED_29_BIT_WITHOUT_FC_AND_WITH_PADDING: Self = Self(54);
    pub const NORMAL_SEGMENTED_29_BIT_WITH_FC_AND_WITH_PADDING: Self = Self(55);

    pub const EXT_UNSEGMENTED_11_BIT_WITH_PADDING: Self = Self(56);
    pub const EXT_UNSEGMENTED_29_BIT_WITH_PADDING: Self = Self(58);

    pub const EXT_SEGMENTED_11_BIT_WITHOUT_FC_AND_WITH_PADDING: Self = Self(60);
    pub const EXT_SEGMENTED_11_BIT_WITH_FC_AND_WITH_PADDING: Self = Self(61);

    pub const EXT_SEGMENTED_29_BIT_WITHOUT_FC_AND_WITH_PADDING: Self = Self(62);
    pub const EXT_SEGMENTED_29_BIT_WITH_FC_AND_WITH_PADDING: Self = Self(63);
}

impl From<CpCanFuncReqFormat> for u32 {
    fn from(value: CpCanFuncReqFormat) -> Self {
        value.0
    }
}

impl From<u32> for CpCanFuncReqFormat {
    fn from(value: u32) -> Self {
        Self(value)
    }
}