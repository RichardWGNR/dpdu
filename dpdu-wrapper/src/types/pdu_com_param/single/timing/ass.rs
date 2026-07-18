use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_As (Send timeout).
///
/// Defines the maximum time to wait for successful transmission of a frame
/// (Single Frame, First Frame, Flow Control, or Consecutive Frame) onto the bus.
#[derive(Debug, Copy, Clone)]
pub enum CpAs {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpAs {
    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl CpAs {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpAs> for ComParamDefinition {
    fn from(value: CpAs) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_As".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpAs> for u32 {
    fn from(value: CpAs) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpAs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpAs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}