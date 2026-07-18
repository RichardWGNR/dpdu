use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::err_hdl::CpRc23RequestTime;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_RC78CompletionTimeout
///
/// Specifies the timeout for completion of request processing for NRC 0x78.
/// This parameter defines the maximum time the system shall wait for completion
/// of the operation associated with NRC 0x78 before considering the operation
/// unsuccessful and proceeding with the next step.
#[derive(Debug, Copy, Clone)]
pub enum CpRc78Completiontimeout {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpRc78Completiontimeout {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpRc78Completiontimeout::Micros(v) => v.to_owned(),
            CpRc78Completiontimeout::Millis(v) => v.wrapping_mul(1000),
            CpRc78Completiontimeout::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl CpRc78Completiontimeout {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpRc78Completiontimeout> for ComParamDefinition {
    fn from(value: CpRc78Completiontimeout) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RC78CompletionTimeout".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpRc78Completiontimeout> for u32 {
    fn from(value: CpRc78Completiontimeout) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpRc78Completiontimeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpRc78Completiontimeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}