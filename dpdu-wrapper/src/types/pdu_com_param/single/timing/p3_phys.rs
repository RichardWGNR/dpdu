use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::timing::{CpP2Min, CpP3Func, CpP3Min};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_P3Phys
///
/// Specifies the P3 timer value for physical addressing in the diagnostic system.
/// This parameter defines the minimum time that the diagnostic tester shall
/// wait between the completion of one diagnostic communication exchange and
/// transmission of the next request when using physical addressing of the ECU.
/// It is used to comply with the timing requirements of the UDS/KWP diagnostic
/// protocol.
#[derive(Debug, Copy, Clone)]
pub enum CpP3Phys {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpP3Phys {
    pub const ZERO: Self = Self::Micros(0);

    pub fn to_micros(&self) -> u32 {
        match self {
            CpP3Phys::Micros(v) => v.to_owned(),
            CpP3Phys::Millis(v) => v.wrapping_mul(1000),
            CpP3Phys::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpP3Phys> for ComParamDefinition {
    fn from(value: CpP3Phys) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P3Phys".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP3Phys> for u32 {
    fn from(value: CpP3Phys) -> Self {
        value.to_micros().into()
    }
}

impl From<u32> for CpP3Phys {
    fn from(value: u32) -> Self {
        Self::Micros(value)
    }
}

impl Serialize for CpP3Phys {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpP3Phys {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}