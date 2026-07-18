use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_TesterPresentTime
///
/// Specifies the time period during which the system shall wait for the
/// presence of the tester. This parameter defines the maximum time the system
/// shall wait for a response from the diagnostic device (tester) after
/// transmitting a TesterPresent request.
#[derive(Debug, Copy, Clone)]
pub enum CpTesterPresentTime {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpTesterPresentTime {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpTesterPresentTime::Micros(v) => v.to_owned(),
            CpTesterPresentTime::Millis(v) => v.wrapping_mul(1000),
            CpTesterPresentTime::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl From<CpTesterPresentTime> for ComParamDefinition {
    fn from(value: CpTesterPresentTime) -> Self {
        ComParamDefinition {
            class: PduPc::TesterPresent,
            short_name: "CP_TesterPresentTime".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpTesterPresentTime> for u32 {
    fn from(value: CpTesterPresentTime) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpTesterPresentTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpTesterPresentTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
