use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_Br.
///
/// Defines the maximum time the sender waits for a Flow Control (FC) frame from
/// the receiver after transmitting a block of Consecutive Frames (CF) when
/// Block Size (BS) is greater than zero.
#[derive(Debug, Copy, Clone)]
pub enum CpBr {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpBr {
    pub const ZERO: Self = Self::Micros(0);

    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl From<CpBr> for ComParamDefinition {
    fn from(value: CpBr) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_Br".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpBr> for u32 {
    fn from(value: CpBr) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpBr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpBr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
