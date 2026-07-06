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

        let Some(module_name) = &self.module_name else {
            return None;
        };

        if let Some(caps) = EDIC_RGX.captures(module_name) {
            return Some(caps.name("name").expect("vci name from regex").as_str().to_owned());
        }

        if let Some(caps) = ACTIA_RGX.captures(module_name) {
            return Some(caps.name("name").expect("vci name from regex").as_str().to_owned());
        }

        None
    }
}
