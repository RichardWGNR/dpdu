use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_RepeatReqCountTrans
///
/// Specifies the number of request transmission retries in case of a failed
/// transmission. If a request cannot be successfully transmitted or processed,
/// this parameter defines how many additional attempts shall be performed
/// before the operation is considered unsuccessful.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpRepeatReqCountTrans(pub u32);

impl From<CpRepeatReqCountTrans> for ComParamDefinition {
    fn from(value: CpRepeatReqCountTrans) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RepeatReqCountTrans".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpRepeatReqCountTrans {
    pub const ZERO: Self = Self(0);
}

impl From<CpRepeatReqCountTrans> for u32 {
    fn from(value: CpRepeatReqCountTrans) -> Self {
        value.0
    }
}

impl From<u32> for CpRepeatReqCountTrans {
    fn from(value: u32) -> Self {
        Self(value)
    }
}