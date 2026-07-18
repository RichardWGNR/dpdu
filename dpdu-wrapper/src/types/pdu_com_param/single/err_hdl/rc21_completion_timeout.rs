use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// CP_RC21CompletionTimeout
///
/// Specifies the timeout for completion of request processing for NRC 0x21.
/// This parameter defines the maximum time the system shall wait for completion
/// of the operation associated with NRC 0x21 before considering the operation
/// unsuccessful and proceeding with the next step.
#[derive(Debug, Copy, Clone)]
pub enum CpRc21Completiontimeout {
    Micros(u32),
    Millis(u32),
    Secs(u32),
}

impl CpRc21Completiontimeout {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpRc21Completiontimeout::Micros(v) => v.to_owned(),
            CpRc21Completiontimeout::Millis(v) => v.wrapping_mul(1000),
            CpRc21Completiontimeout::Secs(v) => v.wrapping_mul(1000000),
        }
    }
}

impl From<CpRc21Completiontimeout> for ComParamDefinition {
    fn from(value: CpRc21Completiontimeout) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RC21CompletionTimeout".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpRc21Completiontimeout> for u32 {
    fn from(value: CpRc21Completiontimeout) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpRc21Completiontimeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpRc21Completiontimeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}
