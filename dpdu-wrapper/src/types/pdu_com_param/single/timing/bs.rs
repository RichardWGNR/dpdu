use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_Bs
///
/// Specifies the maximum number of Consecutive Frames (CF) that the sender may
/// transmit consecutively without receiving a Flow Control (FC) frame from the
/// receiver. This parameter defines the ISO-TP block size used to control the
/// flow of segmented message transmission.
#[derive(Debug, Copy, Clone)]
pub enum CpBs {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpBs {
    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpBs> for ComParamDefinition {
    fn from(value: CpBs) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_Bs".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpBs> for u32 {
    fn from(value: CpBs) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpBs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpBs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}