use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_P2Min
///
/// Specifies the minimum time interval after which the tester (diagnostic
/// application) shall wait for a response from the ECU after transmitting
/// a request.
#[derive(Debug, Copy, Clone)]
pub enum CpP2Min {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpP2Min {
    pub const ZERO: Self = Self::Millis(0);
}

impl CpP2Min {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpP2Min::Micros(v) => v.to_owned(),
            CpP2Min::Millis(v) => v.wrapping_mul(1000),
            CpP2Min::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl From<CpP2Min> for ComParamDefinition {
    fn from(value: CpP2Min) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P2Min".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP2Min> for u32 {
    fn from(value: CpP2Min) -> Self {
        value.to_micros().into()
    }
}

impl From<u32> for CpP2Min {
    fn from(value: u32) -> Self {
        Self::Micros(value)
    }
}

impl Serialize for CpP2Min {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpP2Min {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
