use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_StMinOverride.
///
/// Specifies an override value for the minimum separation time between
/// Consecutive Frames (CF), replacing the value defined by CP_StMin for
/// specific transfers or operating conditions. This allows adjustment of the
/// inter-frame delay to optimize performance or satisfy specific system
/// requirements.
#[derive(Debug, Copy, Clone)]
pub enum CpStMinOverride {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpStMinOverride {
    pub const NOT_USED: Self = Self::Micros(0xFFFFFFFF);

    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl From<CpStMinOverride> for ComParamDefinition {
    fn from(value: CpStMinOverride) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_StMinOverride".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpStMinOverride> for u32 {
    fn from(value: CpStMinOverride) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpStMinOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpStMinOverride {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
