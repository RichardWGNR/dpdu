use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_Cs (Consecutive Frame send timeout).
///
/// Defines the maximum time allowed before sending the next Consecutive Frame
/// (CF) during an ISO-TP multi-frame transfer after receiving a Flow Control
/// (FC) frame from the receiver.
#[derive(Debug, Copy, Clone)]
pub enum CpCs {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpCs {
    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl CpCs {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpCs> for ComParamDefinition {
    fn from(value: CpCs) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_Cs".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpCs> for u32 {
    fn from(value: CpCs) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpCs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpCs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
