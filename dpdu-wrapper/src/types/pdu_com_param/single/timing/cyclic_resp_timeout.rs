use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::com::CpLoopback;
use crate::types::pdu_com_param::single::timing::CpChangeSpeedTxDelay;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CyclicRespTimeout
///
/// Specifies the timeout for waiting for a cyclic response. This parameter
/// defines the maximum time the VCI shall wait for a response to a request
/// before the request is considered to have timed out and the appropriate
/// timeout handling is performed.
#[derive(Debug, Copy, Clone)]
pub enum CpCyclicRespTimeout {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpCyclicRespTimeout {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpCyclicRespTimeout::Micros(v) => v.to_owned(),
            CpCyclicRespTimeout::Millis(v) => v.wrapping_mul(1000),
            CpCyclicRespTimeout::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl CpCyclicRespTimeout {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpCyclicRespTimeout> for ComParamDefinition {
    fn from(value: CpCyclicRespTimeout) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_CyclicRespTimeout".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpCyclicRespTimeout> for u32 {
    fn from(value: CpCyclicRespTimeout) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpCyclicRespTimeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpCyclicRespTimeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}