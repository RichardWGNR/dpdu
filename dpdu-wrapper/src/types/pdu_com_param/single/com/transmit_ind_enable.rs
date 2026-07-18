use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_TransmitIndEnable
///
/// Specifies whether the message transmission indication is enabled. This
/// parameter indicates whether the system shall transmit indications or
/// notifications about the start of data transmission. It is used to notify
/// the system or other devices about the beginning or successful completion
/// of message transmission.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpTransmitIndEnable(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool,
);

impl From<CpTransmitIndEnable> for ComParamDefinition {
    fn from(value: CpTransmitIndEnable) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_TransmitIndEnable".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl CpTransmitIndEnable {
    pub const ENABLE: CpTransmitIndEnable = CpTransmitIndEnable(true);
    pub const DISABLE: CpTransmitIndEnable = CpTransmitIndEnable(false);
}

impl From<CpTransmitIndEnable> for u32 {
    fn from(value: CpTransmitIndEnable) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpTransmitIndEnable> for bool {
    fn from(value: CpTransmitIndEnable) -> Self {
        value.0
    }
}

impl From<bool> for CpTransmitIndEnable {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpTransmitIndEnable {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}
