use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_ChangeSpeedTxDelay
///
/// Specifies the transmission delay after a data rate change. This parameter
/// defines the delay time that shall be observed before starting data
/// transmission after the data rate has been changed. It is used to ensure
/// proper synchronization of communication when switching to a new data rate.
#[derive(Debug, Copy, Clone)]
pub enum CpChangeSpeedTxDelay {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpChangeSpeedTxDelay {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpChangeSpeedTxDelay::Micros(v) => v.to_owned(),
            CpChangeSpeedTxDelay::Millis(v) => v.wrapping_mul(1000),
            CpChangeSpeedTxDelay::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl CpChangeSpeedTxDelay {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpChangeSpeedTxDelay> for ComParamDefinition {
    fn from(value: CpChangeSpeedTxDelay) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_ChangeSpeedTxDelay".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpChangeSpeedTxDelay> for u32 {
    fn from(value: CpChangeSpeedTxDelay) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpChangeSpeedTxDelay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpChangeSpeedTxDelay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}