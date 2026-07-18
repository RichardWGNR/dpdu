use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_ECULayerShortName
///
/// Virtual parameter.
/// Must not be transmitted directly. Instead, a [`unique_resp_identifier`]
/// is generated from the hash of its value in [`PDUSetUniqueRespIdTable`].
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpEcuLayerShortName(pub String);

impl From<CpEcuLayerShortName> for ComParamDefinition {
    fn from(value: CpEcuLayerShortName) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_EcuLayerShortName".to_string(),
            variant: value.0.as_bytes().to_vec().into(),
        }
    }
}

impl CpEcuLayerShortName {
    pub fn empty() -> Self {
        CpEcuLayerShortName(String::new())
    }
}

impl From<String> for CpEcuLayerShortName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for CpEcuLayerShortName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
