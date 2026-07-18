use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::err_hdl::{CpRc21RequestTime, CpRc23Completiontimeout};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_RC23RequestTime
///
/// Specifies the request timeout for NRC 0x23. This parameter defines the
/// maximum time during which the system shall attempt to execute the request
/// or operation associated with NRC 0x23 before considering it unsuccessful
/// and proceeding with the next retry or operation.
#[derive(Debug, Copy, Clone)]
pub enum CpRc23RequestTime {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpRc23RequestTime {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpRc23RequestTime::Micros(v) => v.to_owned(),
            CpRc23RequestTime::Millis(v) => v.wrapping_mul(1000),
            CpRc23RequestTime::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl CpRc23RequestTime {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpRc23RequestTime> for ComParamDefinition {
    fn from(value: CpRc23RequestTime) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RC23RequestTime".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpRc23RequestTime> for u32 {
    fn from(value: CpRc23RequestTime) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpRc23RequestTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpRc23RequestTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}