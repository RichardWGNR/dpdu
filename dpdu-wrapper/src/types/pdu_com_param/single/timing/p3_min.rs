use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_P3Min
///
/// Specifies the minimum time that the diagnostic tester (for example, a
/// diagnostic application) shall wait after loss of communication before
/// restarting the diagnostic session.
#[derive(Debug, Copy, Clone)]
pub enum CpP3Min {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpP3Min {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpP3Min::Micros(v) => v.to_owned(),
            CpP3Min::Millis(v) => v.wrapping_mul(1000),
            CpP3Min::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl From<CpP3Min> for ComParamDefinition {
    fn from(value: CpP3Min) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P3Min".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP3Min> for u32 {
    fn from(value: CpP3Min) -> Self {
        value.to_micros().into()
    }
}

impl From<u32> for CpP3Min {
    fn from(value: u32) -> Self {
        Self::Micros(value)
    }
}

impl Serialize for CpP3Min {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpP3Min {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
