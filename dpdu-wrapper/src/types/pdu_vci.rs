use crate::api::Api;
use crate::types::PduModuleHandle;
use crate::types::pdu_status::PduStatusData;
use dpdu_api_types::PduStatus;
use std::sync::{LazyLock, Weak};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct PduVci {
    pub(crate) api: Weak<Api>,

    pub(crate) h_mod: PduModuleHandle,

    pub(crate) module_name: Option<String>,

    pub(crate) additional_info: Option<String>,

    pub(crate) status: PduStatusData,
}

impl PduVci {
    pub fn get_handle(&self) -> PduModuleHandle {
        self.h_mod
    }

    pub fn get_name(&self) -> Option<&String> {
        self.module_name.as_ref()
    }

    pub fn get_additional_info(&self) -> Option<&String> {
        self.additional_info.as_ref()
    }

    pub fn is_available_for_connection(&self) -> bool {
        match self.status.status_code {
            PduStatus::ModstReady | PduStatus::ModstAvail => true,
            _ => false,
        }
    }

    pub fn get_normalized_name(&self) -> Option<String> {
        static EDIC_RGX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"(?U)ModuleName='(?<name>.+)'"#).unwrap());
        static ACTIA_RGX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"(?U)MVCIFriendlyName='(?<name>.+)'"#).unwrap());

        let module_name = self.module_name.as_ref()?;

        let normalize = |name: &str| {
            if !name.is_empty() && name.chars().all(|c| c.is_ascii_digit()) {
                format!("VCI S/N: {name}")
            } else {
                name.to_owned()
            }
        };

        for regex in [&*EDIC_RGX, &*ACTIA_RGX] {
            if let Some(caps) = regex.captures(module_name) {
                return Some(normalize(caps.name("name").unwrap().as_str()));
            }
        }

        Some(normalize(module_name))
    }
}
