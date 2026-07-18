use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::com::CpChangeSpeedRate;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CanMaxNumWaitFrames.
///
/// Defines the maximum number of Flow Control (FC) frames that can be sent by
/// the receiver during an ISO-TP data transfer in response to a request.
/// Flow Control frames are used to regulate the transmission rate and prevent
/// receiver buffer overflow when transferring large messages.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCanMaxNumWaitFrames(pub u32);

impl From<CpCanMaxNumWaitFrames> for ComParamDefinition {
    fn from(value: CpCanMaxNumWaitFrames) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanMaxNumWaitFrames".to_string(),
            variant: value.0.into()
        }
    }
}

impl CpCanMaxNumWaitFrames {
    pub const ZERO: Self = Self(0);
}

impl From<CpCanMaxNumWaitFrames> for u32 {
    fn from(value: CpCanMaxNumWaitFrames) -> Self {
        value.0
    }
}

impl From<u32> for CpCanMaxNumWaitFrames {
    fn from(value: u32) -> Self {
        Self(value)
    }
}