use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_StMin.
///
/// Defines the minimum delay between frames when data is transmitted using
/// multi-frame messages in the ISO-TP protocol. This parameter is important for
/// ensuring proper data transmission timing and synchronization between
/// devices.
#[derive(Debug, Copy, Clone)]
pub enum CpStMin {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpStMin {
    pub const ZERO: Self = Self::Micros(0);

    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl From<CpStMin> for ComParamDefinition {
    fn from(value: CpStMin) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_StMin".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpStMin> for u32 {
    fn from(value: CpStMin) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpStMin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpStMin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
