use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::timing::{CpP2Max, CpP2Min};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_P2Star
///
/// Specifies the maximum time for the second response time phase (P2*) in the
/// UDS protocol. This time may be extended when the data exchange between
/// devices requires additional processing time. This parameter is used to
/// configure the timeout period during which the system shall wait for the
/// completion of the second response phase, even if the process takes longer
/// than usual.
#[derive(Debug, Copy, Clone)]
pub enum CpP2Star {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpP2Star {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpP2Star::Micros(v) => v.to_owned(),
            CpP2Star::Millis(v) => v.wrapping_mul(1000),
            CpP2Star::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpP2Star> for ComParamDefinition {
    fn from(value: CpP2Star) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P2Star".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP2Star> for u32 {
    fn from(value: CpP2Star) -> Self {
        value.to_micros().into()
    }
}

impl From<u32> for CpP2Star {
    fn from(value: u32) -> Self {
        Self::Micros(value)
    }
}

impl Serialize for CpP2Star {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpP2Star {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}