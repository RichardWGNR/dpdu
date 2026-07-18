use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_Ar (Response timeout).
///
/// Defines the maximum response timeout in milliseconds. This is the maximum
/// time the sender waits for a response frame from the receiver after
/// transmitting a request.
#[derive(Debug, Copy, Clone)]
pub enum CpAr {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpAr {
    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl CpAr {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpAr> for ComParamDefinition {
    fn from(value: CpAr) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_Ar".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpAr> for u32 {
    fn from(value: CpAr) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpAr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpAr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
