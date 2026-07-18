use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_Cr.
///
/// Controls the system behavior when a previously transmitted request needs to
/// be canceled. This is typically used in protocols where an ongoing data
/// transmission or request may need to be interrupted (for example, during
/// diagnostics using UDS or other protocols).
#[derive(Debug, Copy, Clone)]
pub enum CpCr {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpCr {
    pub fn to_micros(&self) -> u32 {
        match self {
            Self::Micros(v) => v.to_owned(),
            Self::Millis(v) => v.wrapping_mul(1000),
            Self::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl CpCr {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpCr> for ComParamDefinition {
    fn from(value: CpCr) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_Cr".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpCr> for u32 {
    fn from(value: CpCr) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpCr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpCr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
