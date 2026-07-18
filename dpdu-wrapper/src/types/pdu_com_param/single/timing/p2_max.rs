use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::timing::{CpCyclicRespTimeout, CpP3Func};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_P2Max.
///
/// Specifies the maximum P2 server response time in the UDS protocol. This is
/// the maximum time the client waits for a response from the server after
/// transmitting a diagnostic request. This parameter is used to configure
/// response timeout handling for communication under varying system load
/// conditions.
#[derive(Debug, Copy, Clone)]
pub enum CpP2Max {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpP2Max {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpP2Max::Micros(v) => v.to_owned(),
            CpP2Max::Millis(v) => v.wrapping_mul(1000),
            CpP2Max::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpP2Max> for ComParamDefinition {
    fn from(value: CpP2Max) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P2Max".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP2Max> for u32 {
    fn from(value: CpP2Max) -> Self {
        value.to_micros().into()
    }
}

impl From<u32> for CpP2Max {
    fn from(value: u32) -> Self {
        Self::Micros(value)
    }
}

impl Serialize for CpP2Max {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpP2Max {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}