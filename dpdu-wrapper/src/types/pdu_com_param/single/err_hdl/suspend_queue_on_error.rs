use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::table::ComParamDefinition;
use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::single::com::CpSwCanHighVoltage;

/// CP_SuspendQueueOnError
///
/// Specifies whether the message queue shall be suspended in case of an error.
/// This parameter indicates whether the system shall suspend the processing of
/// subsequent messages in the queue when an error occurs, for example, during
/// request processing or data transmission. It is used to prevent the
/// transmission of subsequent messages until the error condition has been
/// resolved.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpSuspendQueueOnError(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool
);

impl From<CpSuspendQueueOnError> for ComParamDefinition {
    fn from(value: CpSuspendQueueOnError) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_SuspendQueueOnError".to_string(),
            variant: (if value.0 { 1u32 } else { 0 }).into(),
        }
    }
}

impl CpSuspendQueueOnError {
    pub const DISABLE: Self = CpSuspendQueueOnError(false);
    pub const ENABLE: Self = CpSuspendQueueOnError(true);
}

impl From<CpSuspendQueueOnError> for u32 {
    fn from(value: CpSuspendQueueOnError) -> Self {
        value.0.then(|| 1).unwrap_or(0)
    }
}

impl From<CpSuspendQueueOnError> for bool {
    fn from(value: CpSuspendQueueOnError) -> Self {
        value.0
    }
}

impl From<bool> for CpSuspendQueueOnError {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpSuspendQueueOnError {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}