use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::timing::{CpP2Min, CpP3Phys};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_P3Func
///
/// Specifies the functional behavior of the P3 timer in the diagnostic system.
/// This parameter indicates whether the P3 timer shall be applied for functional
/// addressing of diagnostic requests. It is used to control the waiting time
/// between consecutive diagnostic messages during functional communication
/// according to the protocol requirements.
#[derive(Debug, Copy, Clone)]
pub enum CpP3Func {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpP3Func {
    pub const ZERO: Self = Self::Millis(0);

    pub fn to_micros(&self) -> u32 {
        match self {
            CpP3Func::Micros(v) => v.to_owned(),
            CpP3Func::Millis(v) => v.wrapping_mul(1000),
            CpP3Func::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpP3Func> for ComParamDefinition {
    fn from(value: CpP3Func) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P3Func".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP3Func> for u32 {
    fn from(value: CpP3Func) -> Self {
        value.to_micros().into()
    }
}

impl From<u32> for CpP3Func {
    fn from(value: u32) -> Self {
        Self::Micros(value)
    }
}

impl Serialize for CpP3Func {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpP3Func {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}