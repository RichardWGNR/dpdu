use dpdu_api_types::PduPc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::types::pdu_com_param::single::err_hdl::{CpRc21Completiontimeout, CpRc23Completiontimeout};
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_RC21RequestTime
///
/// Specifies the request timeout for NRC 0x21. This parameter defines the
/// maximum time the system shall wait before starting the handling procedure
/// or taking actions when NRC 0x21 occurs. It is used to control the time
/// period during which the system attempts to process the request before
/// considering it unsuccessful.
#[derive(Debug, Copy, Clone)]
pub enum CpRc21RequestTime {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpRc21RequestTime {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpRc21RequestTime::Micros(v) => v.to_owned(),
            CpRc21RequestTime::Millis(v) => v.wrapping_mul(1000),
            CpRc21RequestTime::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl CpRc21RequestTime {
    pub const ZERO: Self = Self::Millis(0);
}

impl From<CpRc21RequestTime> for ComParamDefinition {
    fn from(value: CpRc21RequestTime) -> Self {
        ComParamDefinition {
            class: PduPc::ErrHdl,
            short_name: "CP_RC21RequestTime".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpRc21RequestTime> for u32 {
    fn from(value: CpRc21RequestTime) -> Self {
        value.to_micros().into()
    }
}

impl Serialize for CpRc21RequestTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_micros().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CpRc21RequestTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = u32::deserialize(deserializer)?;
        Ok(Self::Micros(micros))
    }
}