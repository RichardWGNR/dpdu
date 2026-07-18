use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_CanTransmissionTime
///
/// Specifies the transmission time on the CAN (Controller Area Network).
/// It defines the duration required to transmit a message or a specific unit
/// of data over the CAN network.
#[derive(Debug, Copy, Clone)]
pub enum CpCanTransmissionTime {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpCanTransmissionTime {
    pub const ZERO: Self = Self::Millis(0);

    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl From<CpCanTransmissionTime> for ComParamDefinition {
    fn from(value: CpCanTransmissionTime) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_CanTransmissionTime".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpCanTransmissionTime> for u32 {
    fn from(value: CpCanTransmissionTime) -> Self {
        value.to_micros().into()
    }
}

impl From<u32> for CpCanTransmissionTime {
    fn from(value: u32) -> Self {
        Self::Micros(value)
    }
}

impl Serialize for CpCanTransmissionTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpCanTransmissionTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
