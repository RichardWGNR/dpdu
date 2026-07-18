use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::err_hdl::CpRc21RequestTime;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_RC23CompletionTimeout
///
/// Specifies the timeout for completion of request processing for NRC 0x23.
/// This parameter defines the maximum time the system shall wait for completion
/// of the operation associated with NRC 0x23 before considering the operation
/// unsuccessful and proceeding with the next step.
#[derive(Debug, Copy, Clone)]
pub enum CpRc23Completiontimeout {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpRc23Completiontimeout {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpRc23Completiontimeout::Micros(v) => v.to_owned(),
            CpRc23Completiontimeout::Millis(v) => v.wrapping_mul(1000),
            CpRc23Completiontimeout::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpRc23Completiontimeout> for ComParamDefinition {
    fn from(value: CpRc23Completiontimeout) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RC23CompletionTimeout".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl CpRc23Completiontimeout {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpRc23Completiontimeout> for u32 {
    fn from(value: CpRc23Completiontimeout) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpRc23Completiontimeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpRc23Completiontimeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}